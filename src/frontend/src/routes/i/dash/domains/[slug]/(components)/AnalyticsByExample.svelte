<script>
    import {onMount} from "svelte";
    import {Label} from "$lib/components/ui/label";
    import {Input} from "$lib/components/ui/input";

    function balls() {
        console.log(`Mounted`);
        let start = shown_examples.length;
        console.log(`Start: ${start + 10}`, shown_examples.length, by_example.length);
        for (let i = shown_examples.length; i < start + 10 && i < by_example.length; i++) {
            console.log("iteration");
            if (by_example[i].path?.length > 20) {
                by_example[i].path = by_example[i].path.substring(0, 20) + "...";
            }
            shown_examples.push({
                expanded: false,
                ...by_example[i]
            });
        }

        console.log("Shown examples", shown_examples.length);

        shown_examples = shown_examples;

        return by_example;
    }

    function displayStatus(example) {
        if (!example.rproxy_hit) {
            return "Backend not requested";
        }
        if (example.errored.includes("Some") && example.response_code.includes("Some")) {
            return `${example.response_code.replace("Some(", "").replace(")", "")}: ${example.errored.replace("Some(", "").replace(")", "")}`;
        } else if (example.errored.includes("Some")) {
            return `Err: ${example.errored.replace("Some(", "").replace(")", "")}`;
        } else if (example.response_code.includes("Some")) {
            return example.response_code.replace("Some(", "").replace(")", "");
        } else {
            return "None";
        }
    }

    function displayAction(example) {
        // actions can look like Trustbusting([]) or Monopoly()
        for (let i = 0; i < example.action.length; i++) {
            const action = example.action[i];
            if (action.includes("Trustbusting")) {
                return action.replace("Trustbusting([", "").replace("])", "")
                    .split("(")[0];
            } else if (action.includes("Monopoly")) {
                return "Monopoly".replace("Monopoly(", "").replace(")", "");
            } else {
                return "None";
            }
        }
    }

    export let by_example;
    export let insights;
    let shown_examples = [];
    let searched_examples = [];
    export let remaining;

    $: by_example = balls();

    // search functionality
    let searchQuery = "";
    let countThreshold = 10;
    let matches = 0;

    $: searchQuery, searchHandler();
    $: countThreshold, countThresholdChange();

    function countThresholdChange() {
        // when the count threshold changes, we need to get the searches again
        if (countThreshold > 10) {
            searchHandler();
        }
    }


    // search for elements based on any category, showing the first ten
    function searchHandler() {
        matches = 0;
        if (searchQuery === "") {
            console.log("searchQuery is empty")
            shown_examples = [];
            countThreshold = 0;
            balls();
            return;
        } else if (searchQuery !== "") {
            if (countThreshold === 0) {
                countThreshold = 10;
            }

            let count = 0;
            shown_examples = by_example.filter((example) => {
                let resp = false;

                for (let i = 0; i < example.action.length; i++) {
                    if (example.action[i].includes(searchQuery)) {
                        resp = true;
                    }
                }

                if (resp) {
                    matches++;
                    if (count < countThreshold) {
                        count++;
                        return true;
                    } else {
                        return false;
                    }
                }

                resp = example.date.includes(searchQuery) || example.ip.includes(searchQuery) || example.path.includes(searchQuery)
                    || example.country.includes(searchQuery) || example.fingerprint.includes(searchQuery) || example.asn.includes(searchQuery)
                    || example.user_agent.includes(searchQuery) || example.elapsed.includes(searchQuery) || example.response_code.includes(searchQuery)
                    || example.errored.includes(searchQuery) || example.threat_score.toString().includes(searchQuery)
                    || example.setting.includes(searchQuery) || example.rule.includes(searchQuery);

                if (resp) {
                    matches++;
                    if (count < countThreshold) {
                        count++;
                        return true;
                    } else {
                        return false;
                    }
                }

                return resp;
            });
        } else {
            return [];
        }
    }
</script>

<div class="">
    <!-- section for insights -->
    <div class="grid grid-cols-3">
        <div class="text-center p-5">
            <p>Top ASNs</p>
            {#each insights.asns as asn}
                <p class="text-sm text-muted-foreground">
                {asn.name}: {asn.percentage}%
                </p>
            {/each}
        </div>

        <div class="text-center p-5">
            <p>Top countries</p>
            {#each insights.countries as country}
                <p class="text-sm text-muted-foreground">
                    {country.name}: {country.percentage}%
                </p>
            {/each}
        </div>

        <div class="text-center p-5">
            <p>Top IP addresses</p>
            {#each insights.ips as ip}
                <p class="text-sm text-muted-foreground">
                    {ip.ip}: {ip.percentage}%
                </p>
            {/each}
        </div>

        <div class="text-center p-5">
            <p>Top paths</p>
            {#each insights.paths as path}
                <p class="text-sm text-muted-foreground overflow-ellipsis overflow-hidden">
                    <a class="overflow-ellipsis overflow-hidden whitespace-nowrap">{path.path}</a>: {path.percentage}%
                </p>
            {/each}
        </div>

        <div class="text-center p-5">
            <p>Top user agents</p>
            {#each insights.user_agents as user_agent}
                <div class="text-sm text-muted-foreground overflow-ellipsis overflow-hidden">
                    <a class="overflow-ellipsis overflow-hidden whitespace-nowrap">{user_agent.user_agent}</a>: {user_agent.percentage}%
                </div>
            {/each}
        </div>

        <div class="text-center p-5 text-sm">
            <p>
                Bot traffic
            </p>
            <p class="text-muted-foreground">
                Off first inspection, for every 1 request made by a real browser, {insights.bot_vs_real} request(s) were made by bots.
            </p>
            <br>
            <p class="text-muted-foreground">
                On a scale of 0 to 100, the average threat score for incoming traffic was {insights.threat_score}.
            </p>
        </div>
    </div>

    <h3 class="py-2 uppercase text-base font-semibold leading-7 text-muted-foreground">
        {#if searchQuery === ""}
            <a>Analytics by Example</a>
            - showing {shown_examples.length} of {(by_example.length + remaining) - shown_examples.length}
        {:else}
            <a>Search results for "{searchQuery}"</a>
            - showing {shown_examples.length} of {(by_example.length + remaining) - shown_examples.length}, {matches} matches
        {/if}
    </h3>

    <div>
        <div class="mt-2 py-2">
            <Input
                    bind:value={searchQuery}
                    id="search"
                    name="search"
                    type="text"
                    placeholder="Search for anything ..."
                    required
            />
        </div>
    </div>

    {#each shown_examples as general}
    <div class="border-2">
        <div class="flex w-full">
            <div class="px-4 py-6 sm:px-6 lg:px-8">
                <p class="text-sm font-medium leading-6 text-muted-foreground">
                    Date
                </p>
                <p class="mt-2 flex items-baseline gap-x-2">
                    <span class="text-xl font-semibold tracking-tight text-primary"
                        >{general.date}</span
                    >
                </p>
            </div>
            <div class="px-4 py-6 sm:px-6 lg:px-8">
                <p class="text-sm font-medium leading-6 text-muted-foreground">
                    IP
                </p>
                <p class="mt-2 flex items-baseline gap-x-2">
                    <span class="text-xl font-semibold tracking-tight text-primary"
                        >{general.ip}</span
                    >
                </p>
            </div>
            <div class="px-4 py-6 sm:px-6 lg:px-8">
                <p class="text-sm font-medium leading-6 text-muted-foreground">
                    Path
                </p>
                <p class="mt-2 flex items-baseline gap-x-2">
                    <span class="text-xl font-semibold tracking-tight text-primary overflow-ellipsis overflow-hidden"
                    >{general.path}</span>
                </p>
            </div>
            <div class="px-4 py-6 sm:px-6 lg:px-8">
                <p class="text-sm font-medium leading-6 text-muted-foreground">
                    Country
                </p>
                <p class="mt-2 flex items-baseline gap-x-2">
                    <span class="text-xl font-semibold tracking-tight text-primary"
                        >{general.country}</span
                    >
                </p>
            </div>
            <div class="px-4 py-6 sm:px-6 lg:px-8">
                <p class="text-sm font-medium leading-6 text-muted-foreground">
                    Initial Detection
                </p>
                <p class="mt-2 flex items-baseline gap-x-2">
                    <span class="text-xl font-semibold tracking-tight text-primary"
                    >{general.fingerprint}</span>
                </p>
            </div>
            <div class="px-4 py-6 sm:px-6 lg:px-8">
                <p class="text-sm font-medium leading-6 text-muted-foreground">
                    Status
                </p>
                <p class="mt-2 flex items-baseline gap-x-2">
                    <span class="text-xl font-semibold tracking-tight text-primary"
                    >{displayStatus(general)}</span>
                </p>
            </div>
        </div>
        {#if general.expanded}
            <div class="grid grid-cols-6">
                <div class="px-4 py-6 sm:px-6 lg:px-8">
                    <p class="text-sm font-medium leading-6 text-muted-foreground">
                        ASN
                    </p>
                    <p class="mt-2 flex items-baseline gap-x-2">
                    <span class="text-xl font-semibold tracking-tight text-primary"
                    >{general.asn}</span>
                    </p>
                </div>
                <div class="px-4 py-6 sm:px-6 lg:px-8">
                    <p class="text-sm font-medium leading-6 text-muted-foreground">
                        User agent
                    </p>
                    <p class="mt-2 flex items-baseline gap-x-2">
                    <span class="text-xl font-semibold tracking-tight text-primary overflow-ellipsis overflow-hidden"
                    >{general.user_agent}</span>
                    </p>
                </div>
                <div class="px-4 py-6 sm:px-6 lg:px-8">
                    <p class="text-sm font-medium leading-6 text-muted-foreground">
                        Total request time
                    </p>
                    <p class="mt-2 flex items-baseline gap-x-2">
                    <span class="text-xl font-semibold tracking-tight text-primary"
                    >{general.elapsed}</span>
                    </p>
                </div>
                <div class="px-4 py-6 sm:px-6 lg:px-8">
                    <p class="text-sm font-medium leading-6 text-muted-foreground">
                        Cache response
                    </p>
                    <p class="mt-2 flex items-baseline gap-x-2">
                    <span class="text-xl font-semibold tracking-tight text-primary"
                    >{general.caching_hit ? "Hit" : "None"}</span>
                    </p>
                </div>
                <div class="px-4 py-6 sm:px-6 lg:px-8">
                    <p class="text-sm font-medium leading-6 text-muted-foreground">
                        Domain Traffic
                    </p>
                    <p class="mt-2 flex items-baseline gap-x-2">
                        <span class="text-xl font-semibold tracking-tight text-primary"
                        >{general.domain_traffic}</span>
                        <span class="text-sm text-muted-foreground">reqs</span>
                    </p>
                </div>
                {#if general.smart_challenge_served}
                    <div class="px-4 py-6 sm:px-6 lg:px-8">
                        <p class="text-sm font-medium leading-6 text-muted-foreground">
                            Challenge
                        </p>
                        <p class="mt-2 flex items-baseline gap-x-2">
                        <span class="text-xl font-semibold tracking-tight text-primary"
                        >Served</span
                        >
                        </p>
                    </div>
                {/if}
                {#if general.api_hit}
                    <div class="px-4 py-6 sm:px-6 lg:px-8">
                        <p class="text-sm font-medium leading-6 text-muted-foreground">
                            API Engine Hit
                        </p>
                        <p class="mt-2 flex items-baseline gap-x-2">
                    <span class="text-xl font-semibold tracking-tight text-primary"
                    >Setting: {general.setting}</span>
                        </p>
                    </div>
                {/if}
                {#if general.pr_hit}
                    <div class="px-4 py-6 sm:px-6 lg:px-8">
                        <p class="text-sm font-medium leading-6 text-muted-foreground">
                            Page Rule Hit
                        </p>
                        <p class="mt-2 flex items-baseline gap-x-2">
                    <span class="text-xl font-semibold tracking-tight text-primary"
                    >Rule: {general.rule}</span>
                        </p>
                    </div>
                {/if}
                {#if general.bm_allowed}
                    <div class="px-4 py-6 sm:px-6 lg:px-8">
                        <p class="text-sm font-medium leading-6 text-muted-foreground">
                            Bot Engine
                        </p>
                        <p class="mt-2 flex items-baseline gap-x-2">
                    <span class="text-xl font-semibold tracking-tight text-primary"
                    >Allowed</span>
                        </p>
                    </div>
                {/if}
                {#if general.he_hit}
                    <div class="px-4 py-6 sm:px-6 lg:px-8">
                        <p class="text-sm font-medium leading-6 text-muted-foreground">
                            Human Engine Hit
                        </p>
                        <p class="mt-2 flex items-baseline gap-x-2">
                    <span class="text-xl font-semibold tracking-tight text-primary"
                    >Score: {general.threat_score}</span>
                        </p>
                    </div>
                {/if}
                {#if general.action?.length !== 0}
                    <div class="px-4 py-6 sm:px-6 lg:px-8">
                        <p class="text-sm font-medium leading-6 text-muted-foreground">
                            Action
                        </p>
                        <p class="mt-2 flex items-baseline gap-x-2">
                    <span class="text-xl font-semibold tracking-tight text-primary overflow-wrap overflow-hidden"
                    >{displayAction(general)}</span>
                        </p>
                    </div>
                {/if}
                {#if general.rproxy_bytes_written != 0}
                    <div class="px-4 py-6 sm:px-6 lg:px-8">
                        <p class="text-sm font-medium leading-6 text-muted-foreground">
                            Bytes written from backend
                        </p>
                        <p class="mt-2 flex items-baseline gap-x-2">
                    <span class="text-xl font-semibold tracking-tight text-primary"
                    >{general.rproxy_bytes_written}</span>
                        </p>
                    </div>
                {/if}
            </div>
        {/if}
        <p on:click={() => general.expanded = !general.expanded}
           class="text-muted-foreground text-center text-sm py-1">
            {general.expanded ? "Collapse" : "Expand"}
        </p>
    </div>
    {/each}

    {#if searchQuery === ""}
        <p on:click={balls} class="text-center py-2">Click to include more rows</p>
    {:else}
        <p on:click={() => countThreshold += 10} class="text-center py-2">Click to include more rows</p>
    {/if}
</div>
