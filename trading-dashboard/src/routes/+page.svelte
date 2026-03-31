<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import {
    Chart,
    LinearScale,
    CategoryScale,
    PointElement,
    LineElement,
    LineController,
    Tooltip,
    Legend,
    Filler,
  } from "chart.js";

  type LogRow = {
    timestamp: string;
    action: string;
    price: number;
    confidence: number;
    reasoning: string;
    pnl: number;
    drawdown: number;
  };

  let logs = $state<LogRow[]>([]);
  let fetchError = $state<string | null>(null);
  let pnlCanvas = $state<HTMLCanvasElement | null>(null);
  let ddCanvas = $state<HTMLCanvasElement | null>(null);
  let pnlChart: Chart | null = null;
  let ddChart: Chart | null = null;
  let poll: ReturnType<typeof setInterval> | undefined;

  const accent = "#5eead4";
  const warn = "#fbbf24";
  const grid = "rgba(148, 163, 184, 0.15)";

  function destroyCharts() {
    pnlChart?.destroy();
    ddChart?.destroy();
    pnlChart = null;
    ddChart = null;
  }

  function rebuildCharts(rows: LogRow[]) {
    destroyCharts();
    if (!pnlCanvas || !ddCanvas || rows.length === 0) return;

    const labels = rows.map((r) => {
      const d = new Date(r.timestamp);
      return Number.isNaN(d.getTime())
        ? r.timestamp.slice(11, 19)
        : d.toLocaleTimeString(undefined, { hour: "2-digit", minute: "2-digit", second: "2-digit" });
    });

    Chart.defaults.color = "#94a3b8";
    Chart.defaults.borderColor = grid;

    pnlChart = new Chart(pnlCanvas, {
      type: "line",
      data: {
        labels,
        datasets: [
          {
            label: "PnL",
            data: rows.map((r) => r.pnl),
            borderColor: accent,
            backgroundColor: "rgba(94, 234, 212, 0.12)",
            fill: true,
            tension: 0.25,
            pointRadius: 0,
            pointHoverRadius: 4,
          },
        ],
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        plugins: { legend: { display: false } },
        scales: {
          x: { ticks: { maxTicksLimit: 8 } },
          y: { ticks: { callback: (v) => `${v}` } },
        },
      },
    });

    ddChart = new Chart(ddCanvas, {
      type: "line",
      data: {
        labels,
        datasets: [
          {
            label: "Drawdown %",
            data: rows.map((r) => r.drawdown * 100),
            borderColor: warn,
            backgroundColor: "rgba(251, 191, 36, 0.08)",
            fill: true,
            tension: 0.25,
            pointRadius: 0,
            pointHoverRadius: 4,
          },
        ],
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        plugins: { legend: { display: false } },
        scales: {
          x: { ticks: { maxTicksLimit: 8 } },
          y: {
            ticks: {
              callback: (v) => `${Number(v).toFixed(2)}%`,
            },
          },
        },
      },
    });
  }

  async function loadLogs() {
    try {
      const res = await fetch("/logs");
      if (!res.ok) throw new Error(`${res.status} ${res.statusText}`);
      logs = await res.json();
      fetchError = null;
      rebuildCharts(logs);
    } catch (e) {
      fetchError = e instanceof Error ? e.message : "Failed to load logs";
    }
  }

  onMount(() => {
    Chart.register(
      LinearScale,
      CategoryScale,
      PointElement,
      LineElement,
      LineController,
      Tooltip,
      Legend,
      Filler,
    );
    void loadLogs();
    poll = setInterval(loadLogs, 4000);
  });

  onDestroy(() => {
    if (poll) clearInterval(poll);
    destroyCharts();
  });

  const last = $derived(logs.length ? logs[logs.length - 1] : null);
</script>

<svelte:head>
  <title>Agent Dashboard</title>
</svelte:head>

<main class="shell">
  <header class="hero">
    <p class="eyebrow">Kraken paper · live metrics</p>
    <h1>Agent Dashboard</h1>
    <p class="lede">
      PnL, drawdown, last decision, and trade log. Point the dev server proxy at your Rust agent
      (<code>AGENT_HTTP_PORT</code>, default 3030).
    </p>
  </header>

  {#if fetchError}
    <p class="banner error" role="alert">{fetchError}</p>
  {/if}

  <section class="grid-metrics">
    <article class="card metric">
      <h2>PnL</h2>
      <p class="metric-value" class:pos={(last?.pnl ?? 0) >= 0} class:neg={(last?.pnl ?? 0) < 0}>
        {last != null ? last.pnl.toFixed(2) : "—"}
      </p>
    </article>
    <article class="card metric">
      <h2>Drawdown</h2>
      <p class="metric-value muted">
        {last != null ? `${(last.drawdown * 100).toFixed(2)}%` : "—"}
      </p>
    </article>
  </section>

  <section class="card chart-card">
    <h2>PnL over time</h2>
    <div class="chart-wrap">
      <canvas bind:this={pnlCanvas} aria-label="PnL chart"></canvas>
    </div>
  </section>

  <section class="card chart-card">
    <h2>Drawdown</h2>
    <div class="chart-wrap">
      <canvas bind:this={ddCanvas} aria-label="Drawdown chart"></canvas>
    </div>
  </section>

  <section class="card decision">
    <h2>Last decision</h2>
    {#if last}
      <p class="decision-action">
        {last.action}
        <span class="conf">({last.confidence.toFixed(2)})</span>
      </p>
      <p class="reasoning">"{last.reasoning}"</p>
    {:else}
      <p class="muted">No log rows yet. Run the trading agent.</p>
    {/if}
  </section>

  <section class="card table-card">
    <h2>Trade log</h2>
    <div class="table-scroll">
      <table>
        <thead>
          <tr>
            <th>Time</th>
            <th>Action</th>
            <th>Price</th>
            <th>Conf.</th>
            <th>PnL</th>
            <th>DD</th>
          </tr>
        </thead>
        <tbody>
          {#each [...logs].reverse() as log}
            <tr>
              <td class="mono">{log.timestamp}</td>
              <td><span class="pill">{log.action}</span></td>
              <td class="mono">{log.price.toFixed(2)}</td>
              <td class="mono">{log.confidence.toFixed(2)}</td>
              <td class="mono" class:pos={log.pnl >= 0}>{log.pnl.toFixed(2)}</td>
              <td class="mono">{(log.drawdown * 100).toFixed(2)}%</td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  </section>
</main>

<style>
  :global(body) {
    margin: 0;
    min-height: 100vh;
    background: radial-gradient(1200px 600px at 10% -10%, #134e4a 0%, transparent 55%),
      radial-gradient(900px 500px at 100% 0%, #1e3a5f 0%, transparent 50%), #0b1220;
    color: #e2e8f0;
    font-family: "Outfit", system-ui, sans-serif;
  }

  .shell {
    max-width: 1100px;
    margin: 0 auto;
    padding: 2.5rem 1.5rem 4rem;
  }

  .hero {
    margin-bottom: 2rem;
  }

  .eyebrow {
    font-family: "IBM Plex Mono", monospace;
    font-size: 0.75rem;
    letter-spacing: 0.12em;
    text-transform: uppercase;
    color: #5eead4;
    margin: 0 0 0.5rem;
  }

  h1 {
    font-size: clamp(2rem, 4vw, 2.75rem);
    font-weight: 700;
    margin: 0 0 0.5rem;
    letter-spacing: -0.03em;
  }

  .lede {
    max-width: 52ch;
    color: #94a3b8;
    line-height: 1.55;
    margin: 0;
  }

  .lede code {
    font-family: "IBM Plex Mono", monospace;
    font-size: 0.9em;
    color: #cbd5e1;
  }

  .banner.error {
    background: rgba(248, 113, 113, 0.12);
    border: 1px solid rgba(248, 113, 113, 0.35);
    color: #fecaca;
    padding: 0.75rem 1rem;
    border-radius: 10px;
    margin-bottom: 1.25rem;
  }

  .grid-metrics {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
    margin-bottom: 1.25rem;
  }

  .card {
    background: rgba(15, 23, 42, 0.72);
    border: 1px solid rgba(148, 163, 184, 0.18);
    border-radius: 16px;
    padding: 1.25rem 1.35rem;
    backdrop-filter: blur(8px);
  }

  .card h2 {
    font-size: 0.85rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: #94a3b8;
    margin: 0 0 0.75rem;
  }

  .metric-value {
    font-size: 1.85rem;
    font-weight: 700;
    margin: 0;
    font-variant-numeric: tabular-nums;
  }

  .metric-value.pos {
    color: #5eead4;
  }
  .metric-value.neg {
    color: #fca5a5;
  }
  .metric-value.muted {
    color: #fbbf24;
  }

  .chart-card {
    margin-bottom: 1.25rem;
  }

  .chart-wrap {
    height: 220px;
    position: relative;
  }

  .decision-action {
    font-size: 1.35rem;
    font-weight: 600;
    margin: 0 0 0.5rem;
  }

  .conf {
    color: #94a3b8;
    font-weight: 500;
    font-size: 1rem;
  }

  .reasoning {
    margin: 0;
    color: #cbd5e1;
    line-height: 1.5;
    font-style: italic;
  }

  .muted {
    color: #64748b;
    margin: 0;
  }

  .table-card {
    margin-top: 0.5rem;
  }

  .table-scroll {
    overflow-x: auto;
    margin-top: 0.5rem;
  }

  table {
    width: 100%;
    border-collapse: collapse;
    font-size: 0.9rem;
  }

  th,
  td {
    text-align: left;
    padding: 0.55rem 0.65rem;
    border-bottom: 1px solid rgba(148, 163, 184, 0.12);
  }

  th {
    font-size: 0.7rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: #64748b;
  }

  .mono {
    font-family: "IBM Plex Mono", monospace;
    font-size: 0.82rem;
  }

  td.pos {
    color: #5eead4;
  }

  .pill {
    display: inline-block;
    padding: 0.15rem 0.5rem;
    border-radius: 999px;
    background: rgba(94, 234, 212, 0.12);
    color: #99f6e4;
    font-size: 0.8rem;
    font-weight: 600;
  }
</style>
