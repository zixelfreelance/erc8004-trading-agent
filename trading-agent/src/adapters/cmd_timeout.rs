use std::process::{Command, Output, Stdio};
use std::time::{Duration, Instant};

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// Run a command with a timeout. Kills the child if it exceeds the deadline.
pub fn run_with_timeout(program: &str, args: &[&str]) -> anyhow::Result<Output> {
    run_with_timeout_duration(program, args, DEFAULT_TIMEOUT)
}

pub fn run_with_timeout_duration(
    program: &str,
    args: &[&str],
    timeout: Duration,
) -> anyhow::Result<Output> {
    let mut child = Command::new(program)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let start = Instant::now();
    loop {
        match child.try_wait()? {
            Some(_) => return child.wait_with_output().map_err(Into::into),
            None if start.elapsed() > timeout => {
                let _ = child.kill();
                let _ = child.wait();
                anyhow::bail!(
                    "{} {} timed out after {}s",
                    program,
                    args.join(" "),
                    timeout.as_secs()
                );
            }
            None => std::thread::sleep(Duration::from_millis(100)),
        }
    }
}
