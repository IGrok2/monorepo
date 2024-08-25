<script>
    //import * as charts from "echarts";

    /*export function echarts(node, option) {
        const chart = charts.init(node);
        chart.setOption(option);
    }*/

    import Line from "$lib/components/charts/Line.svelte";
    import AnalyticsByExample from "./(components)/AnalyticsByExample.svelte";
    import * as Card from "$lib/components/ui/card";
    //import RequestsByExample from "./(components)/RequestsByExample.svelte";

    import * as Select from "$lib/components/ui/select";
    import * as Tabs from "$lib/components/ui/tabs";

    import { onMount, onDestroy } from "svelte";
    import { page } from "$app/stores";
    import { fade, slide } from "svelte/transition";

    import { PUBLIC_API } from "$env/static/public";
    import Popup from "$lib/Popup.svelte";
    import Notifications from "$lib/components/notifications/Notifications.svelte";
    import LiveBeacon from "$lib/components/LiveBeacon.svelte";
    import { getCookie } from "$lib/utils/auth";
    import Options from "$lib/Options.svelte";

    /** @type {import('./$types').PageData} */
    export let data;

    $: domain_info = data.resp.domain.domain;
    // variable holder for all the analytics by example that we'll have
    let analytics_by_example;
    let by_example_insights;
    let analytics_remaining = 0;
    let loaded = false;
    // let verification_required = data.verification.required;
    $: no_origins = data.resp.no_origins;
    //let verification_key = data.verification.key;
    //let verification_date = data.verification.date;
    let api_engine = "";
    let cache_engine = "";
    let bot_engine = "";
    let page_rules = "";
    let token;
    let total_requests = 0;
    let security_events = 0;
    let data_transferred = 0;
    let data_units = "";

    let tabs = ["Overview", "Bandwidth", "Security"];
    let activeTab = "Overview";

    let dates = [];
    let requests = [];
    let blocked = [];

    let option = {
        tooltip: {
            trigger: "axis",
            position: function (pt) {
                return [pt[0], "10%"];
            },
        },
        xAxis: {
            type: "category",
            boundaryGap: false,
            data: [],
        },
        yAxis: {
            type: "value",
            boundaryGap: [0, "100%"],
        },
        series: [
            {
                name: "Requests",
                type: "line",
                symbol: "none",
                sampling: "lttb",
                itemStyle: {
                    color: "rgb(255, 70, 131)",
                },
                data: [],
            },
        ],
    };

    let possibleRanges = ["5m", "1h", "24h", "48h", "7d"];
    let activeRange = $page.url.searchParams.get("range") || "48h";

    let changeDate = false;
    let changeTab = false;

    let date_since = new Date();

    const load = async () => {
        console.log("loading analytics");
        try {
            token = getCookie("jwt");

            await fetch(
                `${PUBLIC_API}/@/project/${$page.params.project}/domain/${$page.params.slug}/analytics`,
                {
                    method: "POST",
                    headers: new Headers({
                        "content-type": "application/json",
                        Authorization: token,
                    }),
                    body: JSON.stringify({
                        domain: $page.params.slug,
                        date_since: activeRange,
                    }),
                },
            ).then(async (rawBody) => {
                let body = await rawBody.json();
                console.log(body);

                analytics_by_example = body.analytics.by_example.by_example;
                by_example_insights = body.analytics.by_example.insights;
                analytics_remaining = body.analytics.by_example.left;

                //console.log(body);

                //Reset analytics
                dates = [];

                option.xAxis.data = [];

                option.series[0].data = [];

                // Ready analytics
                for (const analytic of body.analytics.base_analytics) {
                    option.xAxis.data.push(analytic.date);
                    dates.push(analytic.date);

                    option.series[0].data.push(analytic.total);
                }

                total_requests = body.analytics.total;
                security_events = body.analytics.security_events;
                data_transferred = body.analytics.data_transferred;

                if (data_transferred > 1024 ** 3) {
                    data_transferred = data_transferred / 1024 ** 3;
                    data_units = "TBs";
                } else if (data_transferred > 1024 ** 2) {
                    data_transferred = data_transferred / 1024 ** 2;
                    data_units = "GBs";
                } else if (data_transferred > 1024) {
                    data_transferred = data_transferred / 1024;
                    data_units = "MBs";
                } else {
                    data_units = "KBs";
                }

                // Round to 2 decimal places
                data_transferred = data_transferred.toFixed(2);

                loaded = true;
            });
        } catch (err) {
            console.log(err);

            //document.location.href = "/i/auth/login";
        }
    };

    let intervalId;
    let intervalStart;
    let timeLeft;
    let timeLeftIntervalId;

    onMount(() => {
        load(); // Run immediately on mount
        if (activeRange === "5m" || activeRange === "1h") {
            intervalStart = Date.now();
            intervalId = setInterval(() => {
                //load();
                intervalStart = Date.now();
            }, 10000); // Then every 10 seconds

            // Start an interval that updates timeLeft every 100ms
            timeLeftIntervalId = setInterval(getTimeLeft, 100);
        }
    });

    function getTimeLeft() {
        const now = Date.now();
        const timePassed = now - intervalStart;
        timeLeft = Math.floor((10000 - timePassed) / 1000);
    }

    onDestroy(() => {
        clearInterval(intervalId); // Clear the interval when the component is destroyed
        clearInterval(timeLeftIntervalId); // Clear the timeLeft interval as well
    });
</script>

<!--
<div class="flex-none fixed bottom-0 bg-fuchsia-500 text-slate-200 border-2 border-amber-300 z-20">
    <p class="sticky bottom-0 p-3">Build: opnieuw 1.0-alpha.r4<br>
        All rights reserved - Whole Lotta Heart, Corp.</p>
</div>
-->

{#if no_origins}
    <div class="flex w-full">
        <div class="w-full p-5">
            <p class="uppercase text-xl text-muted-foreground">No Origins</p>
            <p class="text-4xl pt-3">
                See analytics for {$page.params.slug}, you'll have to create and
                add
                <a
                    class="text-fuchsia-600 underline"
                    href="{$page.params.slug}/origins">origins</a
                > <br />
                <span class="text-muted-foreground text-xl"
                    >This will enable us to forward, load balance, and protect
                    traffic to your servers</span
                >
            </p>
        </div>
    </div>
{:else if loaded}
    <div class="flex w-full">
        <div class="w-full my-2">
            <p class="mt-2 uppercase text-primary">
                at a glance, last {activeRange}
            </p>
            <p class="text-2xl pt-3 text-primary">
                We've welcomed <a
                    class="underline decoration-dashed hover:italic hover:decoration-0"
                    >{total_requests}</a
                >
                requests, transferred
                <a
                    class="underline decoration-dashed hover:italic hover:decoration-0"
                    >{data_transferred}</a
                >
                {data_units}, and mitigated
                <a
                    class="underline decoration-dashed hover:italic hover:decoration-0"
                    >{security_events}</a
                >
                security threats. <br />
            </p>
            <div class="mt-2 flex justify-between">
                <div class="relative inline-block text-left">
                    <Select.Root>
                        <Select.Trigger class="w-[180px]">
                            <Select.Value placeholder="Overview" />
                        </Select.Trigger>
                        <Select.Content>
                            {#each tabs as tab}
                                <Select.Item
                                    value={tab}
                                    on:click={() => {
                                        activeTab = tab;
                                    }}>{tab}</Select.Item
                                >
                            {/each}
                        </Select.Content>
                    </Select.Root>
                </div>
                <div class="relative inline-block text-left">
                    <Select.Root>
                        <Select.Trigger class="w-[180px]">
                            <Select.Value placeholder={activeRange} />
                        </Select.Trigger>
                        <Select.Content>
                            {#each possibleRanges as range}
                                <Select.Item
                                    value={range}
                                    on:click={() => {
                                        document.location.href = `/i/dash/domains/${$page.params.slug}?range=${range}`;
                                    }}>{range}</Select.Item
                                >
                            {/each}
                        </Select.Content>
                    </Select.Root>
                </div>
            </div>
        </div>
    </div>
    <Tabs.Root value="chart">
        <Tabs.List>
            <Tabs.Trigger value="chart">Chart</Tabs.Trigger>
            <Tabs.Trigger value="table">Table</Tabs.Trigger>
            <Tabs.Trigger value="cards">Cards</Tabs.Trigger>
        </Tabs.List>
        <Tabs.Content value="chart">
            <!--<div class="border rounded w-full h-[500px]" use:echarts={option} />-->
            <Card.Root>
                <Card.Header>
                    <Card.Title>Requests</Card.Title>
                    <Card.Description>How many requests have you had in the time frame.</Card.Description>
                </Card.Header>
                <Card.Content>
                    <Line />
                </Card.Content>
            </Card.Root>
        </Tabs.Content>
        <Tabs.Content value="table"
            ><!-- <RequestsByExample
                data={analytics_by_example.slice(0, 10)}
            /> --></Tabs.Content
        >
        <Tabs.Content value="cards"
            ><AnalyticsByExample
                insights={by_example_insights}
                by_example={analytics_by_example}
                remaining={analytics_remaining}
            /></Tabs.Content
        >
    </Tabs.Root>
{/if}
