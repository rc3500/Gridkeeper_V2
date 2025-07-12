<script>
  import Chart from 'chart.js/auto';
  let chartData = { labels: [], datasets: [] };

  async function fetchData() {
    chartData = await window.__TAURI__.invoke('get_chart_data');
    new Chart(document.getElementById('chart'), { type: 'line', data: chartData });
  }
</script>

<div>
  <h1>Gridkeeper Dashboard</h1>
  <canvas id="chart"></canvas>
  <button on:click={fetchData}>Load Charts</button>
  <!-- Add wizard forms for onboarding -->
</div>
