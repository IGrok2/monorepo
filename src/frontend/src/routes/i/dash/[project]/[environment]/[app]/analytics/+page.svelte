<script>
    import axios from "axios";

    import * as charts from "echarts";

    export function echarts(node, option) {
        const chart = charts.init(node);
        chart.setOption(option);
    }

    let loading = true;

    let option = {
        /*tooltip: {
            trigger: "axis",
            position: function (pt) {
                return [pt[0], "10%"];
            },
        },*/
        xAxis: {
            type: "category",
            data: [],
        },
        yAxis: {
            type: "value",
        },
        series: [
            {
                name: "CPU",
                type: "line",
                data: [],
            },
        ],
    };

    /*let option = {
        xAxis: {
            type: "category",
            data: ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"],
        },
        yAxis: {
            type: "value",
        },
        series: [
            {
                data: [150, 230, 224, 218, 135, 147, 260],
                type: "line",
            },
        ],
    };*/

    async function queryPrometheus(query) {
        const prometheusUrl =
            "http://prometheus.aqueous.cloud/api/v1/query_range"; // Replace with your Prometheus server URL

        try {
            const response = await axios.get(prometheusUrl, {
                params: {
                    query: query,
                    start: 1717626261.353,
                    end: 1717628061.353,
                    step: 7,
                },
            });

            if (response.data.status === "success") {
                return response.data.data.result;
            } else {
                throw new Error(`Query failed: ${response.data.error}`);
            }
        } catch (error) {
            console.error("Error querying Prometheus:", error);
            throw error;
        }
    }

    // Example usage
    const queryString =
        'rate(container_cpu_usage_seconds_total{pod=~"app-665e451dfd86e2df8614e5ef.*"}[1m])';

    function fetchDataAndUpdateChart() {
        queryPrometheus(queryString)
            .then((result) => {
                console.log("Query result:", result);

                option.xAxis.data = [];
                option.series[0].data = [];

                for (let i = 0; i < result[0].values.length; i++) {
                    const value = result[0].values[i];
                    option.xAxis.data.push(new Date(value[0]));
                    option.series[0].data.push(Number(value[1]));
                }

                console.log("Option: ", option);
                loading = false;
            })
            .catch((error) => {
                console.error("Error:", error);
            });
    }

    fetchDataAndUpdateChart();

    // Set up the interval to fetch and update the chart data every second
    //setInterval(fetchDataAndUpdateChart, 1000);
</script>

{#if !loading}
    <div
        class="bg-muted bg-opacity-20 rounded w-full h-[500px]"
        use:echarts={option}
    />
{/if}
