<script>
    import Chart from "chart.js/auto";
    import { onMount } from "svelte";

    export let labels = [];
    export let datasets = [];
    export let title = "";

    let ctx;

    let chartCanvas;

    onMount(async () => {
        ctx = chartCanvas.getContext("2d");
        console.log(ctx);

        new Chart(ctx, {
            type: "doughnut",
            data: {
                labels: labels,
                datasets: datasets,
            },
            options: {
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
    });
</script>

<div class="w-full flex items-center justify-center">
    <div class="chart-container w-full h-full overflow-hidden">
        <canvas bind:this={chartCanvas} class="chart" />
    </div>
</div>

<style>
    .chart-container {
        position: relative;
        width: 50%;
        height: 50%;
    }

    .chart {
        width: 100%;
        height: 100%;
    }
</style>
