<script lang="ts">
  import { onMount } from 'svelte';
  import {
    Chart,
    LineController,
    LineElement,
    PointElement,
    LinearScale,
    CategoryScale,
    Filler,
    Tooltip,
    Legend,
  } from 'chart.js';

  Chart.register(
    LineController,
    LineElement,
    PointElement,
    LinearScale,
    CategoryScale,
    Filler,
    Tooltip,
    Legend
  );

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

  let logs = $state<LogRow[]>([]);
  let error = $state<string | null>(null);
  let pnlCanvas = $state<HTMLCanvasElement | null>(null);
  let ddCanvas = $state<HTMLCanvasElement | null>(null);
  let pnlChart: Chart | null = null;
  let ddChart: Chart | null = null;

  const apiBase = import.meta.env.VITE_LOGS_URL ?? 'http://127.0.0.1:3030';

  async function refresh() {
    try {
      const r = await fetch(`${apiBase}/logs`);
      if (!r.ok) throw new Error(`${r.status} ${r.statusText}`);
      logs = await r.json();
      error = null;
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    }
  }

  function shortTs(ts: string) {
    try {
      const d = new Date(ts);
      return d.toLocaleTimeString(undefined, { hour: '2-digit', minute: '2-digit', second: '2-digit' });
    } catch {
      return ts;
    }
  }

  $effect(() => {
    const labels = logs.map((l) => shortTs(l.timestamp));
    const pnlData = logs.map((l) => l.pnl);
    const ddData = logs.map((l) => l.drawdown * 100);

    if (pnlCanvas) {
      pnlChart?.destroy();
      pnlChart = new Chart(pnlCanvas, {
        type: 'line',
        data: {
          labels,
          datasets: [
            {
              label: 'PnL',
              data: pnlData,
              borderColor: '#0d9488',
              backgroundColor: 'rgba(13, 148, 136, 0.12)',
              fill: true,
              tension: 0.25,
            },
          ],
        },
        options: {
          responsive: true,
          maintainAspectRatio: false,
          plugins: { legend: { display: false } },
          scales: {
            x: { ticks: { maxRotation: 0, autoSkip: true, maxTicksLimit: 8 } },
            y: { title: { display: true, text: 'PnL' } },
          },
        },
      });
    }

    if (ddCanvas) {
      ddChart?.destroy();
      ddChart = new Chart(ddCanvas, {
        type: 'line',
        data: {
          labels,
          datasets: [
            {
              label: 'Drawdown %',
              data: ddData,
              borderColor: '#c2410c',
              backgroundColor: 'rgba(194, 65, 12, 0.1)',
              fill: true,
              tension: 0.25,
            },
          ],
        },
        options: {
          responsive: true,
          maintainAspectRatio: false,
          plugins: { legend: { display: false } },
          scales: {
            x: { ticks: { maxRotation: 0, autoSkip: true, maxTicksLimit: 8 } },
            y: { title: { display: true, text: '%' } },
          },
        },
      });
    }
  });

  onMount(() => {
    void refresh();
    const id = setInterval(refresh, 3000);
    return () => {
      clearInterval(id);
      pnlChart?.destroy();
      ddChart?.destroy();
    };
  });

  const last = $derived(logs.length ? logs[logs.length - 1] : null);
</script>

<svelte:head>
  <title>Trading agent — demo</title>
</svelte:head>

<div class="page">
  <header class="hero">
    <p class="eyebrow">Hackathon demo</p>
    <h1>Agent decisions</h1>
    <p class="lede">Live feed from <code>GET /logs</code> — trades, PnL, drawdown.</p>
    <button type="button" class="refresh" onclick={() => refresh()}>Refresh now</button>
  </header>

  {#if error}
    <p class="err" role="alert">{error}</p>
  {/if}

  <section class="grid">
    <article class="card decision">
      <h2>Last decision</h2>
      {#if last}
        <dl>
          <div><dt>Action</dt><dd>{last.action}</dd></div>
          <div><dt>Price</dt><dd>{last.price.toLocaleString()}</dd></div>
          <div><dt>Confidence</dt><dd>{(last.confidence * 100).toFixed(1)}%</dd></div>
          <div><dt>PnL</dt><dd>{last.pnl.toFixed(2)}</dd></div>
          <div><dt>Drawdown</dt><dd>{(last.drawdown * 100).toFixed(2)}%</dd></div>
          <div class="full"><dt>Timestamp</dt><dd>{last.timestamp}</dd></div>
          <div class="full reasoning">
            <dt>Reasoning</dt>
            <dd>{last.reasoning || '—'}</dd>
          </div>
          <div><dt>Balance</dt><dd>{last.balance.toFixed(2)}</dd></div>
          <div><dt>Peak</dt><dd>{last.peak_balance.toFixed(2)}</dd></div>
          <div><dt>Risk block</dt><dd>{last.blocked_by_risk ? 'Yes' : 'No'}</dd></div>
        </dl>
      {:else}
        <p class="muted">No rows yet — start the Rust agent.</p>
      {/if}
    </article>

    <article class="card chart">
      <h2>PnL</h2>
      <div class="canvas-wrap"><canvas bind:this={pnlCanvas}></canvas></div>
    </article>

    <article class="card chart">
      <h2>Drawdown</h2>
      <div class="canvas-wrap"><canvas bind:this={ddCanvas}></canvas></div>
    </article>
  </section>

  <section class="card table-card">
    <h2>Trade log</h2>
    <div class="table-wrap">
      <table>
        <thead>
          <tr>
            <th>Time</th>
            <th>Action</th>
            <th>Price</th>
            <th>Conf.</th>
            <th>Risk</th>
            <th>Balance</th>
            <th>PnL</th>
            <th>DD %</th>
          </tr>
        </thead>
        <tbody>
          {#each [...logs].reverse() as row}
            <tr>
              <td class="mono">{shortTs(row.timestamp)}</td>
              <td><span class="pill {row.action.toLowerCase()}">{row.action}</span></td>
              <td>{row.price.toLocaleString()}</td>
              <td>{(row.confidence * 100).toFixed(0)}%</td>
              <td>{row.blocked_by_risk ? 'block' : '—'}</td>
              <td>{row.balance.toFixed(2)}</td>
              <td>{row.pnl.toFixed(2)}</td>
              <td>{(row.drawdown * 100).toFixed(2)}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  </section>
</div>

<style>
  :global(body) {
    margin: 0;
    min-height: 100vh;
    font-family: 'DM Sans', system-ui, sans-serif;
    background: radial-gradient(120% 80% at 10% 0%, #1c1917 0%, #0c0a09 45%, #020617 100%);
    color: #e7e5e4;
  }

  .page {
    max-width: 1120px;
    margin: 0 auto;
    padding: 2.5rem 1.25rem 4rem;
  }

  .hero {
    margin-bottom: 2rem;
  }

  .eyebrow {
    text-transform: uppercase;
    letter-spacing: 0.12em;
    font-size: 0.72rem;
    color: #a8a29e;
    margin: 0 0 0.5rem;
  }

  h1 {
    font-family: 'Instrument Serif', Georgia, serif;
    font-weight: 400;
    font-size: clamp(2.25rem, 5vw, 3.25rem);
    margin: 0 0 0.5rem;
    color: #fafaf9;
  }

  .lede {
    margin: 0 0 1rem;
    color: #a8a29e;
    max-width: 42rem;
    line-height: 1.5;
  }

  code {
    font-size: 0.9em;
    background: rgba(255, 255, 255, 0.06);
    padding: 0.1rem 0.35rem;
    border-radius: 4px;
  }

  .refresh {
    cursor: pointer;
    border: 1px solid rgba(255, 255, 255, 0.2);
    background: rgba(255, 255, 255, 0.06);
    color: #fafaf9;
    padding: 0.5rem 1rem;
    border-radius: 8px;
    font-weight: 600;
  }

  .refresh:hover {
    background: rgba(255, 255, 255, 0.1);
  }

  .err {
    color: #fecaca;
    background: rgba(127, 29, 29, 0.35);
    padding: 0.75rem 1rem;
    border-radius: 8px;
  }

  .grid {
    display: grid;
    gap: 1rem;
    grid-template-columns: 1fr;
  }

  @media (min-width: 900px) {
    .grid {
      grid-template-columns: 1fr 1fr;
      grid-template-rows: auto auto;
    }
    .decision {
      grid-column: 1 / -1;
    }
  }

  .card {
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 14px;
    padding: 1.25rem 1.35rem;
    box-shadow: 0 20px 50px rgba(0, 0, 0, 0.35);
  }

  .card h2 {
    margin: 0 0 1rem;
    font-size: 1rem;
    font-weight: 600;
    color: #d6d3d1;
  }

  .muted {
    color: #78716c;
    margin: 0;
  }

  dl {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
    gap: 0.75rem 1.25rem;
    margin: 0;
  }

  dl div {
    margin: 0;
  }

  dl div.full {
    grid-column: 1 / -1;
  }

  dt {
    font-size: 0.7rem;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: #78716c;
    margin: 0 0 0.2rem;
  }

  dd {
    margin: 0;
    font-size: 1.05rem;
    font-weight: 600;
    color: #fafaf9;
  }

  .reasoning dd {
    font-weight: 400;
    font-size: 0.9rem;
    line-height: 1.45;
    color: #d6d3d1;
  }

  .canvas-wrap {
    height: 220px;
    position: relative;
  }

  .table-card {
    margin-top: 1rem;
  }

  .table-wrap {
    overflow: auto;
    max-height: 360px;
    border-radius: 8px;
  }

  table {
    width: 100%;
    border-collapse: collapse;
    font-size: 0.875rem;
  }

  th,
  td {
    text-align: left;
    padding: 0.5rem 0.65rem;
    border-bottom: 1px solid rgba(255, 255, 255, 0.06);
  }

  th {
    position: sticky;
    top: 0;
    background: #1c1917;
    color: #a8a29e;
    font-weight: 600;
    font-size: 0.72rem;
    text-transform: uppercase;
    letter-spacing: 0.06em;
  }

  .mono {
    font-variant-numeric: tabular-nums;
    color: #d6d3d1;
  }

  .pill {
    display: inline-block;
    padding: 0.15rem 0.5rem;
    border-radius: 999px;
    font-size: 0.75rem;
    font-weight: 600;
  }

  .pill.buy {
    background: rgba(13, 148, 136, 0.25);
    color: #99f6e4;
  }

  .pill.sell {
    background: rgba(194, 65, 12, 0.25);
    color: #fed7aa;
  }

  .pill.hold {
    background: rgba(120, 113, 108, 0.35);
    color: #e7e5e4;
  }
</style>
