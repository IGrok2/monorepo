<script>
    import Chart from "chart.js/auto";
    import { onMount } from "svelte";

    export let labels = [];
    export let datasets = [];
    export let title = "";

    let ctx;
    let chart;
    let chartCanvas;

    /*onMount(async () => {
        ctx = chartCanvas.getContext("2d");
        console.log(ctx);

        new Chart(ctx, {
            type: "line",
            data: {
                labels: labels,
                datasets: datasets,
            },
            options: {
                animations: {
                    tension: {
                        duration: 5000,
                        easing: "easeOutQuart",
                        from: 0.5,
                        to: 0.25,
                    },
                },
                scales: {
                    y: {
                        beginAtZero: true,
                        ticks: {
                            color: "lightgrey", // Set the color of the y-axis labels to white
                        },
                        grid: {
                            color: "rgba(211, 211, 211, 0.1)", // Light grey with 25% opacity
                        },
                    },
                    x: {
                        ticks: {
                            color: "lightgrey", // Set the color of the x-axis labels to white
                        },
                        grid: {
                            color: "rgba(211, 211, 211, 0.1)", // Light grey with 25% opacity
                        },
                    },
                },
                plugins: {
                    legend: {
                        display: false, // Disable the legend
                    },
                    title: {
                        display: true,
                        text: title,
                        color: "lightgrey", // Set the color of the title text to white
                        font: {
                            size: 20, // Set the font size to 20
                        },
                    },
                },
            },
        });
    });*/

    onMount(async () => {
        ctx = chartCanvas.getContext("2d");
        createChart();
    });

    $: if (datasets && ctx) {
        if (chart) {
            chart.destroy();
        }
        createChart();
    }

    function createChart() {
        chart = new Chart(ctx, {
            type: "line",
            data: {
                labels: labels,
                datasets: datasets,
            },
            options: {
                elements: {
                    line: {
                        tension: 0.5,
                    },
                },
                animations: true,
                scales: {
                    y: {
                        beginAtZero: true,
                        ticks: {
                            color: "lightgrey", // Set the color of the y-axis labels to white
                        },
                        grid: {
                            color: "rgba(211, 211, 211, 0.1)", // Light grey with 25% opacity
                        },
                    },
                    x: {
                        ticks: false,
                        grid: {
                            color: "rgba(211, 211, 211, 0.1)", // Light grey with 25% opacity
                        },
                    },
                },
                plugins: {
                    options: {
                        legend: {
                            display: true,
                            labels: {
                                usePointStyle: true, // Use point styles instead of color boxes
                            },
                        },
                    },
                    title: {
                        display: true,
                        text: title,
                        color: "lightgrey", // Set the color of the title text to white
                        font: {
                            size: 20, // Set the font size to 20
                        },
                    },
                },
            },
        });
    }
</script>

<div class="w-full flex items-center justify-center">
    <div class="chart-container w-full h-full overflow-hidden">
        <canvas bind:this={chartCanvas} class="chart" />
    </div>
</div>

<style>
    .chart-container {
        position: relative;
        width: 100%;
        height: 100%;
    }

    .chart {
        width: 100%;
        height: 100%;
    }
</style>
