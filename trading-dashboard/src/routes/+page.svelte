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
    balance: number;
    peak_balance: number;
    blocked_by_risk: boolean;
  };

  const agentOrigin = (import.meta.env.VITE_AGENT_ORIGIN as string | undefined)?.replace(
    /\/$/,
    "",
  );
  const logsUrl = agentOrigin ? `${agentOrigin}/logs` : "/logs";

  let logs = $state<LogRow[]>([]);
  let fetchError = $state<string | null>(null);
  let connected = $state(false);
  let lastPoll = $state<Date | null>(null);

  let pnlCanvas = $state<HTMLCanvasElement | null>(null);
  let ddCanvas = $state<HTMLCanvasElement | null>(null);
  let priceCanvas = $state<HTMLCanvasElement | null>(null);
  let pnlChart: Chart | null = null;
  let ddChart: Chart | null = null;
  let priceChart: Chart | null = null;
  let poll: ReturnType<typeof setInterval> | undefined;

  const accent = "#5eead4";
  const warn = "#fbbf24";
  const priceLine = "#a5b4fc";
  const grid = "rgba(148, 163, 184, 0.15)";

  function normalizeLogs(raw: unknown): LogRow[] {
    if (!Array.isArray(raw)) return [];
    return raw.map((r) => {
      const o = r as Record<string, unknown>;
      return {
        timestamp: String(o.timestamp ?? ""),
        action: String(o.action ?? "Hold"),
        price: Number(o.price ?? 0),
        confidence: Number(o.confidence ?? 0),
        reasoning: String(o.reasoning ?? ""),
        pnl: Number(o.pnl ?? 0),
        drawdown: Number(o.drawdown ?? 0),
        balance: Number(o.balance ?? 0),
        peak_balance: Number(o.peak_balance ?? 0),
        blocked_by_risk: Boolean(o.blocked_by_risk),
      };
    });
  }

  function destroyCharts() {
    pnlChart?.destroy();
    ddChart?.destroy();
    priceChart?.destroy();
    pnlChart = null;
    ddChart = null;
    priceChart = null;
  }

  function rebuildCharts(rows: LogRow[]) {
    destroyCharts();
    if (!pnlCanvas || !ddCanvas || !priceCanvas || rows.length === 0) return;

    const labels = rows.map((r) => {
      const d = new Date(r.timestamp);
      return Number.isNaN(d.getTime())
        ? r.timestamp.slice(11, 19)
        : d.toLocaleTimeString(undefined, {
            hour: "2-digit",
            minute: "2-digit",
            second: "2-digit",
          });
    });

    Chart.defaults.color = "#94a3b8";
    Chart.defaults.borderColor = grid;

    const commonOptions = {
      responsive: true,
      maintainAspectRatio: false,
      plugins: { legend: { display: false } },
      scales: {
        x: { ticks: { maxTicksLimit: 8 } },
      },
    };

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
        ...commonOptions,
        scales: {
          ...commonOptions.scales,
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
        ...commonOptions,
        scales: {
          ...commonOptions.scales,
          y: {
            ticks: {
              callback: (v) => `${Number(v).toFixed(2)}%`,
            },
          },
        },
      },
    });

    priceChart = new Chart(priceCanvas, {
      type: "line",
      data: {
        labels,
        datasets: [
          {
            label: "Price",
            data: rows.map((r) => r.price),
            borderColor: priceLine,
            backgroundColor: "rgba(165, 180, 252, 0.1)",
            fill: true,
            tension: 0.2,
            pointRadius: 0,
            pointHoverRadius: 4,
          },
        ],
      },
      options: {
        ...commonOptions,
        scales: {
          ...commonOptions.scales,
          y: { ticks: { callback: (v) => Number(v).toLocaleString() } },
        },
      },
    });
  }

  async function loadLogs() {
    try {
      const res = await fetch(logsUrl);
      if (!res.ok) throw new Error(`${res.status} ${res.statusText}`);
      logs = normalizeLogs(await res.json());
      fetchError = null;
      connected = true;
      lastPoll = new Date();
    } catch (e) {
      fetchError = e instanceof Error ? e.message : "Failed to load logs";
      connected = false;
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

  $effect(() => {
    if (!pnlCanvas || !ddCanvas || !priceCanvas) return;
    rebuildCharts(logs);
  });

  const last = $derived(logs.length ? logs[logs.length - 1] : null);
</script>

<svelte:head>
  <title>Verifiable agent · dashboard</title>
</svelte:head>

<main class="shell">
  <header class="hero">
    <div class="hero-top">
      <p class="eyebrow">Kraken paper · live audit trail</p>
      <span class="status" class:live={connected} class:down={!connected}>
        {connected ? "Connected" : "Offline"}
      </span>
    </div>
    <h1>Trading agent dashboard</h1>
    <p class="lede">
      Risk-adjusted view of PnL, drawdown, and decisions — fed by the Rust agent’s
      <code>/logs</code> API (signed intents + artifacts in the terminal).
    </p>
    <div class="pipeline" aria-hidden="true">
      <span>Market</span>
      <span class="arrow">→</span>
      <span>Decision</span>
      <span class="arrow">→</span>
      <span>Risk</span>
      <span class="arrow">→</span>
      <span>Intent</span>
      <span class="arrow">→</span>
      <span>Execute</span>
      <span class="arrow">→</span>
      <span>Validate</span>
    </div>
    <p class="meta">
      {#if agentOrigin}
        API: <code>{logsUrl}</code>
      {:else}
        Dev proxy → <code>VITE_AGENT_URL</code> (default <code>127.0.0.1:3030</code>) · or set
        <code>VITE_AGENT_ORIGIN</code> for direct CORS
      {/if}
      {#if lastPoll}
        · updated {lastPoll.toLocaleTimeString()}
      {/if}
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
    <article class="card metric">
      <h2>Paper balance</h2>
      <p class="metric-value balance">
        {last != null ? last.balance.toFixed(2) : "—"}
      </p>
      {#if last && last.peak_balance > 0}
        <p class="sub">Peak {last.peak_balance.toFixed(2)}</p>
      {/if}
    </article>
  </section>

  <section class="grid-charts">
    <article class="card chart-card">
      <h2>PnL over time</h2>
      <div class="chart-wrap">
        <canvas bind:this={pnlCanvas} aria-label="PnL chart"></canvas>
      </div>
    </article>
    <article class="card chart-card">
      <h2>Drawdown</h2>
      <div class="chart-wrap">
        <canvas bind:this={ddCanvas} aria-label="Drawdown chart"></canvas>
      </div>
    </article>
    <article class="card chart-card">
      <h2>Mark price</h2>
      <div class="chart-wrap">
        <canvas bind:this={priceCanvas} aria-label="Price chart"></canvas>
      </div>
    </article>
  </section>

  <section class="card decision">
    <h2>Last decision</h2>
    {#if last}
      <p class="decision-action">
        {last.action}
        <span class="conf">({last.confidence.toFixed(2)})</span>
        {#if last.blocked_by_risk}
          <span class="risk-pill">Risk override</span>
        {/if}
      </p>
      <p class="reasoning">"{last.reasoning}"</p>
    {:else}
      <p class="muted">No log rows yet. Run the trading agent.</p>
    {/if}
  </section>

  <section class="card table-card">
    <h2>Tick log</h2>
    <div class="table-scroll">
      <table>
        <thead>
          <tr>
            <th>Time</th>
            <th>Action</th>
            <th>Risk</th>
            <th>Price</th>
            <th>Conf.</th>
            <th>PnL</th>
            <th>DD</th>
            <th>Balance</th>
          </tr>
        </thead>
        <tbody>
          {#each [...logs].reverse() as log}
            <tr>
              <td class="mono">{log.timestamp}</td>
              <td><span class="pill">{log.action}</span></td>
              <td>
                {#if log.blocked_by_risk}
                  <span class="pill warn">Blocked</span>
                {:else}
                  <span class="pill ok">OK</span>
                {/if}
              </td>
              <td class="mono">{log.price.toFixed(2)}</td>
              <td class="mono">{log.confidence.toFixed(2)}</td>
              <td class="mono" class:pos={log.pnl >= 0}>{log.pnl.toFixed(2)}</td>
              <td class="mono">{(log.drawdown * 100).toFixed(2)}%</td>
              <td class="mono">{log.balance.toFixed(2)}</td>
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
    max-width: 1200px;
    margin: 0 auto;
    padding: 2.5rem 1.5rem 4rem;
  }

  .hero {
    margin-bottom: 2rem;
  }

  .hero-top {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    flex-wrap: wrap;
    margin-bottom: 0.5rem;
  }

  .eyebrow {
    font-family: "IBM Plex Mono", monospace;
    font-size: 0.75rem;
    letter-spacing: 0.12em;
    text-transform: uppercase;
    color: #5eead4;
    margin: 0;
  }

  .status {
    font-family: "IBM Plex Mono", monospace;
    font-size: 0.7rem;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    padding: 0.35rem 0.65rem;
    border-radius: 999px;
    border: 1px solid rgba(148, 163, 184, 0.35);
    color: #94a3b8;
  }

  .status.live {
    border-color: rgba(94, 234, 212, 0.45);
    color: #5eead4;
    background: rgba(94, 234, 212, 0.08);
  }

  .status.down {
    border-color: rgba(248, 113, 113, 0.4);
    color: #fca5a5;
    background: rgba(248, 113, 113, 0.08);
  }

  h1 {
    font-size: clamp(2rem, 4vw, 2.75rem);
    font-weight: 700;
    margin: 0 0 0.5rem;
    letter-spacing: -0.03em;
  }

  .lede {
    max-width: 58ch;
    color: #94a3b8;
    line-height: 1.55;
    margin: 0 0 1.25rem;
  }

  .lede code {
    font-family: "IBM Plex Mono", monospace;
    font-size: 0.9em;
    color: #cbd5e1;
  }

  .pipeline {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.35rem 0.5rem;
    font-family: "IBM Plex Mono", monospace;
    font-size: 0.72rem;
    color: #64748b;
    margin-bottom: 0.75rem;
  }

  .pipeline span:not(.arrow) {
    padding: 0.2rem 0.45rem;
    border-radius: 6px;
    background: rgba(30, 41, 59, 0.9);
    border: 1px solid rgba(148, 163, 184, 0.15);
    color: #cbd5e1;
  }

  .arrow {
    color: #475569;
    padding: 0;
    border: none;
    background: none;
  }

  .meta {
    margin: 0;
    font-size: 0.8rem;
    color: #64748b;
    font-family: "IBM Plex Mono", monospace;
  }

  .meta code {
    color: #94a3b8;
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
    grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
    gap: 1rem;
    margin-bottom: 1.25rem;
  }

  .grid-charts {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
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

  .metric-value.balance {
    color: #a5b4fc;
  }

  .sub {
    margin: 0.35rem 0 0;
    font-size: 0.8rem;
    color: #64748b;
  }

  .chart-card .chart-wrap {
    height: 200px;
    position: relative;
  }

  .decision-action {
    font-size: 1.35rem;
    font-weight: 600;
    margin: 0 0 0.5rem;
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.5rem;
  }

  .conf {
    color: #94a3b8;
    font-weight: 500;
    font-size: 1rem;
  }

  .risk-pill {
    font-size: 0.7rem;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    padding: 0.2rem 0.5rem;
    border-radius: 999px;
    background: rgba(251, 191, 36, 0.15);
    color: #fbbf24;
    border: 1px solid rgba(251, 191, 36, 0.35);
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

  .pill.ok {
    background: rgba(94, 234, 212, 0.1);
    color: #5eead4;
  }

  .pill.warn {
    background: rgba(251, 191, 36, 0.12);
    color: #fcd34d;
  }
</style>
