use std::io::{BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio};

use serde_json::Value;

/// Manages the Kraken MCP subprocess (stdio JSON-RPC).
pub struct KrakenMcp {
    child: Option<Child>,
}

impl KrakenMcp {
    /// Start the Kraken MCP server subprocess.
    pub fn start() -> anyhow::Result<Self> {
        let child = Command::new("kraken")
            .args(["mcp"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn();

        match child {
            Ok(child) => {
                eprintln!("kraken-mcp: started MCP server (pid={})", child.id());
                Ok(Self { child: Some(child) })
            }
            Err(e) => {
                eprintln!("kraken-mcp: failed to start MCP server: {e}");
                eprintln!("kraken-mcp: Claude will not have access to Kraken tools");
                Ok(Self { child: None })
            }
        }
    }

    /// Create a no-op instance (when kraken CLI is not available).
    pub fn noop() -> Self {
        Self { child: None }
    }

    pub fn is_running(&self) -> bool {
        self.child.is_some()
    }

    /// Send a JSON-RPC request and get response.
    pub fn call(&mut self, method: &str, params: Value) -> anyhow::Result<Value> {
        let child = self
            .child
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("MCP server not running"))?;

        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": method,
            "params": params
        });

        let stdin = child
            .stdin
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("MCP stdin not available"))?;

        let mut request_str = serde_json::to_string(&request)?;
        request_str.push('\n');
        stdin.write_all(request_str.as_bytes())?;
        stdin.flush()?;

        let stdout = child
            .stdout
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("MCP stdout not available"))?;

        let mut reader = BufReader::new(stdout);
        let mut line = String::new();
        reader.read_line(&mut line)?;

        let response: Value = serde_json::from_str(&line)?;
        Ok(response)
    }

    /// List available tools from the MCP server.
    pub fn list_tools(&mut self) -> anyhow::Result<Vec<McpTool>> {
        let response = self.call("tools/list", serde_json::json!({}))?;
        let tools = response
            .get("result")
            .and_then(|r| r.get("tools"))
            .and_then(|t| t.as_array())
            .cloned()
            .unwrap_or_default();

        let mcp_tools: Vec<McpTool> = tools
            .iter()
            .filter_map(|t| {
                Some(McpTool {
                    name: t.get("name")?.as_str()?.to_string(),
                    description: t
                        .get("description")
                        .and_then(|d| d.as_str())
                        .unwrap_or("")
                        .to_string(),
                    schema: t
                        .get("inputSchema")
                        .cloned()
                        .unwrap_or(serde_json::json!({})),
                })
            })
            .collect();

        eprintln!("kraken-mcp: discovered {} tools", mcp_tools.len());
        Ok(mcp_tools)
    }

    /// Call a specific tool.
    pub fn call_tool(&mut self, name: &str, arguments: Value) -> anyhow::Result<Value> {
        let response = self.call(
            "tools/call",
            serde_json::json!({
                "name": name,
                "arguments": arguments
            }),
        )?;

        let result = response
            .get("result")
            .cloned()
            .unwrap_or(serde_json::json!({"error": "no result"}));

        Ok(result)
    }
}

impl Drop for KrakenMcp {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
            let _ = child.wait();
            eprintln!("kraken-mcp: server stopped");
        }
    }
}

#[derive(Debug, Clone)]
pub struct McpTool {
    pub name: String,
    pub description: String,
    pub schema: Value,
}

/// Curated list of safe market data tools (no trading, no auth).
pub fn market_tool_names() -> Vec<&'static str> {
    vec![
        "get_ticker",
        "get_ohlc",
        "get_order_book",
        "get_recent_trades",
        "get_recent_spreads",
    ]
}

/// Paper trading tool names.
pub fn paper_tool_names() -> Vec<&'static str> {
    vec![
        "paper_buy",
        "paper_sell",
        "paper_get_balance",
        "paper_get_positions",
        "paper_get_open_orders",
        "paper_get_trades",
    ]
}
