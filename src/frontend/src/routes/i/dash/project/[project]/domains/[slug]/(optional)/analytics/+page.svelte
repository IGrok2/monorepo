<script>
    import SimpleLineChart from "/src/lib/components/charts/SimpleLineChart.svelte";
    import SimplePieChart from "/src/lib/components/charts/SimplePieChart.svelte";
    import { page } from "$app/stores";
    //import { onMount } from "svelte";
    import { fade, fly } from "svelte/transition";
    import { getCookie } from "$lib/utils/auth";

    import { PUBLIC_API } from "$env/static/public";

    let loaded = false;

    let tabs = ["Base", "Buckets", "Origin", "Rules"];
    let activeTab = "Base";

    // For analytics
    let dates = [];
    let overviewDatasets = [
        {
            label: "Requests",
            data: [],
            borderWidth: 4,
            borderColor: "purple",
            fill: "origin",
            backgroundColor: "rgba(200, 160, 220, 0.1)",
        },
        {
            label: "Blocked",
            data: [],
            borderWidth: 4,
            borderColor: "red",
            fill: "origin",
            backgroundColor: "rgba(219, 160, 160, 0.2)",
        },
    ];

    const requestTypes = [
        "Origin Error",
        "Smart Challenged",
        "Cookie Verified",
    ];
    let requestTypeDatasets = [
        {
            label: "Requests",
            data: [1, 1, 1],
            backgroundColor: [
                "rgb(255, 99, 132)",
                "rgb(54, 162, 235)",
                "rgb(255, 205, 86)",
            ],
            hoverOffset: 4,
            borderWidth: 0,
        },
    ];

    let ratelimitedDatasets = [
        {
            label: "Requests",
            data: [],
            borderWidth: 4,
            borderColor: "yellow",
            fill: "origin",
            backgroundColor: "rgba(215, 219, 160, 0.1)",
        },
    ];

    const load = async () => {
        try {
            const slug = $page.params.slug;

            const token = getCookie("jwt");

            // Get the current date
            const currentDate = new Date();

            // Set the date to 7 days ago
            const sevenDaysAgo = new Date();
            sevenDaysAgo.setDate(currentDate.getDate() - 7);

            await fetch(
                `${PUBLIC_API}/@/project/${$page.params.project}/domain/${$page.params.id}/analytics`,
                {
                    method: "POST",
                    headers: new Headers({
                        "content-type": "application/json",
                        Authorization: token,
                    }),
                    body: JSON.stringify({
                        domain: slug,
                        date_since: sevenDaysAgo,
                    }),
                }
            ).then(async (rawBody) => {
                let body = await rawBody.json();

                console.log(body);

                // Ready analytics
                for (const analytic of body.analytics.base_analytics) {
                    console.log("test");
                    dates.push(analytic.date);
                    overviewDatasets[0].data.push(analytic.proxied_reqs);
                    overviewDatasets[1].data.push(analytic.blocked);

                    ratelimitedDatasets[0].data.push(analytic.ratelimited);
                }

                loaded = true;
            });
        } catch (err) {
            console.log(err);
        }
    };

    load();
</script>

<div class="flex justify-center">
    <div class="border-b border-white-800 pb-5 sm:pb-0">
        <!-- Updated border color for dark background -->
        <div class="mt-3 sm:mt-4">
            <nav class="-mb-px flex space-x-8">
                <!-- Current: "border-green-500 text-green-600", Default: "border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700" -->
                {#each tabs as tab}
                    <option
                        class="{tab === activeTab
                            ? 'border-green-500 text-gray-300'
                            : 'border-transparent text-gray-500 hover:border-gray-500 hover:text-gray-400'} whitespace-nowrap border-b-2 px-1 pb-4 text-sm font-medium"
                        on:click={() => (activeTab = tab)}
                    >
                        {tab}
                    </option>
                {/each}
            </nav>
        </div>
    </div>
</div>

<div transition:fade class="flex h-full w-full">
    {#if loaded}
        {#if activeTab === "Base"}
            <div
                class="w-full p-5 space-y-4 backdrop-blur-lg rounded-b-2xl shadow"
            >
                <SimpleLineChart
                    title="Overview"
                    labels={dates}
                    datasets={overviewDatasets}
                />
            </div>
        {/if}
        {#if activeTab === "Buckets"}
            <div
                class="w-full p-5 space-y-4 backdrop-blur-lg rounded-b-2xl shadow"
            >
                <SimplePieChart
                    title="Request Types"
                    labels={requestTypes}
                    datasets={requestTypeDatasets}
                />
            </div>
        {/if}
        {#if activeTab === "Origin"}
            <div
                class="w-full p-5 space-y-4 backdrop-blur-lg rounded-b-2xl shadow"
            >
                <SimpleLineChart labels={dates} datasets={overviewDatasets} />
            </div>
        {/if}
        {#if activeTab === "Rules"}
            <div
                class="w-full p-5 space-y-4 backdrop-blur-lg rounded-b-2xl shadow"
            >
                <SimpleLineChart
                    title="Rate Limited"
                    labels={dates}
                    datasets={ratelimitedDatasets}
                />
            </div>
        {/if}
    {/if}
</div>
