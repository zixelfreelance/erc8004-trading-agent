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

  type Metrics = {
    ticks: number;
    trades_executed: number;
    trades_blocked: number;
    holds: number;
    errors: number;
    wins: number;
    losses: number;
    win_rate: number;
    sharpe_ratio: number;
    regime: string;
  };

  let logs = $state<LogRow[]>([]);
  let metrics = $state<Metrics | null>(null);
  let error = $state<string | null>(null);
  let pnlCanvas = $state<HTMLCanvasElement | null>(null);
  let ddCanvas = $state<HTMLCanvasElement | null>(null);
  let priceCanvas = $state<HTMLCanvasElement | null>(null);
  let pnlChart: Chart | null = null;
  let ddChart: Chart | null = null;
  let priceChart: Chart | null = null;

  const apiBase = import.meta.env.VITE_LOGS_URL ?? 'http://127.0.0.1:3030';

  async function refresh() {
    try {
      const [logsRes, metricsRes] = await Promise.all([
        fetch(`${apiBase}/logs`),
        fetch(`${apiBase}/metrics`),
      ]);
      if (logsRes.ok) logs = await logsRes.json();
      if (metricsRes.ok) metrics = await metricsRes.json();
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

    if (priceCanvas && logs.length > 20) {
      priceChart?.destroy();

      const period = 20;
      const prices = logs.map(l => l.price);
      const upper: (number | null)[] = [];
      const middle: (number | null)[] = [];
      const lower: (number | null)[] = [];

      for (let i = 0; i < prices.length; i++) {
        if (i < period - 1) {
          upper.push(null);
          middle.push(null);
          lower.push(null);
        } else {
          const slice = prices.slice(i - period + 1, i + 1);
          const avg = slice.reduce((a, b) => a + b, 0) / period;
          const std = Math.sqrt(slice.reduce((a, b) => a + (b - avg) ** 2, 0) / period);
          middle.push(avg);
          upper.push(avg + 2 * std);
          lower.push(avg - 2 * std);
        }
      }

      priceChart = new Chart(priceCanvas, {
        type: 'line',
        data: {
          labels,
          datasets: [
            {
              label: 'Price',
              data: prices,
              borderColor: '#fafaf9',
              borderWidth: 2,
              pointRadius: 0,
              tension: 0.1,
            },
            {
              label: 'Upper BB',
              data: upper,
              borderColor: 'rgba(99, 102, 241, 0.5)',
              borderWidth: 1,
              borderDash: [4, 4],
              pointRadius: 0,
              fill: false,
            },
            {
              label: 'SMA(20)',
              data: middle,
              borderColor: 'rgba(99, 102, 241, 0.8)',
              borderWidth: 1,
              pointRadius: 0,
              fill: false,
            },
            {
              label: 'Lower BB',
              data: lower,
              borderColor: 'rgba(99, 102, 241, 0.5)',
              borderWidth: 1,
              borderDash: [4, 4],
              pointRadius: 0,
              fill: '+1',
              backgroundColor: 'rgba(99, 102, 241, 0.06)',
            },
            {
              label: 'Buy',
              data: logs.map((l, i) => l.action === 'Buy' && !l.blocked_by_risk ? prices[i] : null),
              borderColor: '#10b981',
              backgroundColor: '#10b981',
              pointRadius: 6,
              pointStyle: 'triangle',
              showLine: false,
              type: 'line' as const,
            },
            {
              label: 'Sell',
              data: logs.map((l, i) => l.action === 'Sell' && !l.blocked_by_risk ? prices[i] : null),
              borderColor: '#f97316',
              backgroundColor: '#f97316',
              pointRadius: 6,
              pointStyle: 'trianglePerp' as any,
              showLine: false,
              type: 'line' as const,
            },
          ],
        },
        options: {
          responsive: true,
          maintainAspectRatio: false,
          plugins: { legend: { display: true, labels: { color: '#a8a29e', boxWidth: 12 } } },
          scales: {
            x: { ticks: { maxRotation: 0, autoSkip: true, maxTicksLimit: 8, color: '#78716c' } },
            y: { title: { display: true, text: 'Price', color: '#a8a29e' }, ticks: { color: '#78716c' } },
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
      priceChart?.destroy();
    };
  });

  const last = $derived(logs.length ? logs[logs.length - 1] : null);
</script>

<svelte:head>
  <title>Trading agent — demo</title>
</svelte:head>

<div class="page">
  <header class="hero">
    <p class="eyebrow">AI Trading Agents Hackathon</p>
    <h1>Proof-of-Trust Trading Agent</h1>
    <p class="lede">Regime-aware AI agent with verifiable decisions, risk enforcement, and EIP-712 signed intents.</p>
    <button type="button" class="refresh" onclick={() => refresh()}>Refresh now</button>
  </header>

  {#if metrics}
    <section class="metrics-bar">
      <div class="metric">
        <span class="metric-value">{metrics.ticks}</span>
        <span class="metric-label">Ticks</span>
      </div>
      <div class="metric executed">
        <span class="metric-value">{metrics.trades_executed}</span>
        <span class="metric-label">Executed</span>
      </div>
      <div class="metric blocked">
        <span class="metric-value">{metrics.trades_blocked}</span>
        <span class="metric-label">Blocked</span>
      </div>
      <div class="metric">
        <span class="metric-value">{metrics.holds}</span>
        <span class="metric-label">Holds</span>
      </div>
      <div class="metric">
        <span class="metric-value">{metrics.errors}</span>
        <span class="metric-label">Errors</span>
      </div>
    </section>

    <section class="metrics-bar institutional">
      <div class="metric sharpe">
        <span class="metric-value">{metrics.sharpe_ratio.toFixed(2)}</span>
        <span class="metric-label">Sharpe Ratio</span>
      </div>
      <div class="metric winrate">
        <span class="metric-value">{(metrics.win_rate * 100).toFixed(1)}%</span>
        <span class="metric-label">Win Rate ({metrics.wins}W / {metrics.losses}L)</span>
      </div>
      <div class="metric regime-card">
        <span class="metric-value regime-badge {metrics.regime || 'transition'}">{(metrics.regime || 'transition').toUpperCase()}</span>
        <span class="metric-label">Market Regime</span>
      </div>
    </section>
  {/if}

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

    <article class="card chart chart-full">
      <h2>Price & Bollinger Bands</h2>
      <div class="canvas-wrap"><canvas bind:this={priceCanvas}></canvas></div>
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
            <th>Regime</th>
            <th>Price</th>
            <th>Conf.</th>
            <th>Risk</th>
            <th>Balance</th>
            <th>PnL</th>
            <th>DD %</th>
            <th>Chain</th>
          </tr>
        </thead>
        <tbody>
          {#each [...logs].reverse() as row}
            <tr>
              <td class="mono">{shortTs(row.timestamp)}</td>
              <td><span class="pill {row.action.toLowerCase()}">{row.action}</span></td>
              <td>{#if row.regime}<span class="regime-pill {row.regime}">{row.regime}</span>{:else}—{/if}</td>
              <td>{row.price.toLocaleString()}</td>
              <td>{(row.confidence * 100).toFixed(0)}%</td>
              <td class={row.blocked_by_risk ? 'risk-blocked' : 'risk-ok'}>
                {row.blocked_by_risk ? (row.blocked_reason || row.reasoning || '').slice(0, 30) + '...' : '—'}
              </td>
              <td>{row.balance.toFixed(2)}</td>
              <td>{row.pnl.toFixed(2)}</td>
              <td>{(row.drawdown * 100).toFixed(2)}</td>
              <td>{#if row.tx_hash}<a href="https://sepolia.etherscan.io/tx/{row.tx_hash}" target="_blank" class="tx-link" title={row.tx_hash}>{row.tx_hash.slice(0, 10)}...</a>{:else}<span class="muted">—</span>{/if}</td>
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

  .metrics-bar {
    display: flex;
    gap: 1rem;
    margin-bottom: 1.5rem;
    flex-wrap: wrap;
  }

  .metric {
    flex: 1;
    min-width: 100px;
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 10px;
    padding: 1rem;
    text-align: center;
  }

  .metric-value {
    display: block;
    font-size: 1.75rem;
    font-weight: 700;
    color: #fafaf9;
    font-variant-numeric: tabular-nums;
  }

  .metric-label {
    display: block;
    font-size: 0.7rem;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: #78716c;
    margin-top: 0.25rem;
  }

  .metric.executed .metric-value {
    color: #99f6e4;
  }

  .metric.blocked .metric-value {
    color: #fed7aa;
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
      grid-template-rows: auto auto auto;
    }
    .decision {
      grid-column: 1 / -1;
    }
    .chart-full {
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

  .risk-blocked { color: #fed7aa; font-size: 0.75rem; }
  .risk-ok { color: #6ee7b7; }
</style>
