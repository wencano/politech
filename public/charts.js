/**
 * PolitechCharts: radar (topic traits) and regional bar (alignment quick view).
 * Reuses canvas by destroying previous Chart instance before creating a new one.
 */
(function () {
  const instances = {};

  function destroy(canvasId) {
    if (instances[canvasId]) {
      instances[canvasId].destroy();
      instances[canvasId] = null;
    }
  }

  function radar(canvasId, opts) {
    if (!window.Chart) return;
    const canvas = document.getElementById(canvasId);
    if (!canvas) return;
    destroy(canvasId);
    const ctx = canvas.getContext('2d');
    const labels = opts.labels || [];
    const values = opts.values || [];
    instances[canvasId] = new Chart(ctx, {
      type: 'radar',
      data: {
        labels: labels,
        datasets: [{
          label: 'Traits (0–1)',
          data: values,
          fill: true,
          backgroundColor: 'rgba(21, 50, 92, 0.2)',
          borderColor: 'rgb(21, 50, 92)',
          pointBackgroundColor: 'rgb(21, 50, 92)',
          pointBorderColor: '#fff',
          pointHoverBackgroundColor: '#fff',
          pointHoverBorderColor: 'rgb(21, 50, 92)'
        }]
      },
      options: {
        scales: {
          r: {
            min: 0,
            max: 1,
            ticks: { stepSize: 0.2 }
          }
        },
        plugins: {
          legend: { display: false }
        }
      }
    });
  }

  function regionalBar(canvasId, opts) {
    if (!window.Chart) return;
    const canvas = document.getElementById(canvasId);
    if (!canvas) return;
    destroy(canvasId);
    const ctx = canvas.getContext('2d');
    const labels = opts.labels || [];
    const values = opts.values || [];
    instances[canvasId] = new Chart(ctx, {
      type: 'bar',
      data: {
        labels: labels,
        datasets: [{
          label: 'Alignment (%)',
          data: values,
          backgroundColor: 'rgba(21, 50, 92, 0.7)',
          borderColor: 'rgb(21, 50, 92)',
          borderWidth: 1
        }]
      },
      options: {
        indexAxis: 'y',
        scales: {
          x: {
            min: 0,
            max: 1,
            ticks: {
              stepSize: 0.2,
              callback: function (v) { return Math.round(v * 100) + '%'; }
            }
          }
        },
        plugins: {
          legend: { display: false },
          tooltip: {
            callbacks: {
              label: function (ctx) { return Math.round((ctx.raw || 0) * 100) + '%'; }
            }
          }
        },
        maintainAspectRatio: false
      }
    });
  }

  window.PolitechCharts = {
    destroy: destroy,
    radar: radar,
    regionalBar: regionalBar
  };
})();
