<script>
    import Options from "$lib/Options.svelte";
    import { fade, draw } from "svelte/transition";
    import OneOption from "$lib/OneOption.svelte";

    export let domain_info;

    export let settings;

    export let new_rule_popup;
    export let new_rule_setting_index;
    export let new_rule_path;
    export let new_rule_match_type_popup;
    export let new_rule_cache_popup;
    export let new_rule_ratelimit_popup;
    export let new_rule_match_type;
    export let new_rule_order;
    export let new_rule_allow_query_string;
    export let new_rule_ws_methods;
    export let new_rule_http_methods;
    export let new_rule_actions;
    export let new_rule_cache_level;
    export let new_rule_cache_ttl;
    export let new_rule_bucket;
</script>

{#if new_rule_popup}
    <div class="bg-gray-900 p-6 rounded-xl">
        <div class="flex items-start justify-between">
            <div class="text-slate-100">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke-width="1.5"
                    stroke="currentColor"
                    class="w-6 h-6"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        d="M21 7.5l-2.25-1.313M21 7.5v2.25m0-2.25l-2.25 1.313M3 7.5l2.25-1.313M3 7.5l2.25 1.313M3 7.5v2.25m9 3l2.25-1.313M12 12.75l-2.25-1.313M12 12.75V15m0 6.75l2.25-1.313M12 21.75V19.5m0 2.25l-2.25-1.313m0-16.875L12 2.25l2.25 1.313M21 14.25v2.25l-2.25 1.313m-13.5 0L3 16.5v-2.25"
                    />
                </svg>
            </div>
            <h2
                class="font-semibold leading-6 text-slate-100 text-xl"
                id="slide-over-title"
            >
                Modify API Engine rule
            </h2>
            <div class="ml-3 flex h-7 items-center">
                <button
                    on:click={() => {
                        new_rule_popup = false;
                    }}
                    type="button"
                    class="rounded-md text-slate-100 hover:text-slate-400 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
                >
                    <span class="sr-only">Close panel</span>
                    <svg
                        class="h-6 w-6"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke-width="1.5"
                        stroke="currentColor"
                        aria-hidden="true"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            d="M6 18L18 6M6 6l12 12"
                        />
                    </svg>
                </button>
            </div>
        </div>

        <div class="lg:grid lg:grid-cols-2 lg:gap-4">
            <p>
                <label for="rule-path" class="block font-medium text-slate-100"
                    >Specific API path</label
                >
                <a class="text-gray-300 text-sm tracking-tight"
                    >The path to invoke this rule</a
                >
                <input
                    bind:value={new_rule_path}
                    type="text"
                    name="setting-name"
                    id="rule-path"
                    class="mt-1 focus:ring-indigo-500 focus:border-indigo-500 block w-full shadow-sm sm:text-sm border-fuchsia-500 rounded-md"
                    placeholder="/auth/login or /auth/*"
                    required
                />
            </p>

            <div>
                <label
                    for="rule-path"
                    class="block font-medium text-slate-100 pt-6"
                    >Match type</label
                >
                <a class="text-gray-300 text-sm tracking-tight"
                    >How to match the API path</a
                >
                <div>
                    <div>
                        <button
                            on:click={() =>
                                (new_rule_match_type_popup =
                                    !new_rule_match_type_popup)}
                            type="button"
                            class="inline-flex w-full justify-center gap-x-1.5 rounded-md bg-background px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50"
                            id="menu-button"
                            aria-expanded="true"
                            aria-haspopup="true"
                        >
                            {#if !new_rule_match_type}
                                Set a match type
                            {:else}
                                {new_rule_match_type}
                            {/if}

                            <OneOption
                                options={[
                                    "Exact",
                                    "Contains",
                                    "StartsWith",
                                    "UseStar",
                                ]}
                                bind:selected={new_rule_match_type}
                            />
                        </button>
                    </div>
                </div>
            </div>

            <p>
                <label
                    for="rule-path"
                    class="block font-medium text-slate-100 pt-6">Order</label
                >
                <a class="text-gray-300 text-sm tracking-tight"
                    >A 1-20 value that represents the order API rules are
                    processed in.</a
                >
                <input
                    bind:value={new_rule_order}
                    type="number"
                    min="1"
                    max="20"
                    name="setting-name"
                    id="rule-order"
                    class="mt-1 focus:ring-indigo-500 focus:border-indigo-500 block w-full shadow-sm sm:text-sm border-fuchsia-500 rounded-md"
                    placeholder="7"
                    required
                />
            </p>

            <div>
                <label class="block font-medium text-slate-100 pt-6"
                    >HTTP methods allowed</label
                >
                <a class="text-gray-300 text-sm tracking-tight"
                    >HTTP methods to be allowed for this endpoint</a
                >
                <Options
                    name={"HTTP methods"}
                    options={["GET", "POST", "DELETE", "OPTIONS"]}
                    bind:chosen_data={new_rule_http_methods}
                />
            </div>

            <div>
                <label class="block font-medium text-slate-100 pt-6"
                    >WebSocket methods allowed</label
                >
                <a class="text-gray-300 text-sm tracking-tight"
                    >WebSocket methods to be allowed for this endpoint. If no
                    methods are set, WebSockets will not be allowed to connect.</a
                >
                <Options
                    name={"WebSocket methods"}
                    options={["TEXT", "BINARY", "PING", "CLOSE"]}
                    bind:chosen_data={new_rule_ws_methods}
                />
            </div>

            <div class="flex flex-1">
                <div>
                    <label class="block font-medium text-slate-100 pt-6"
                        >Allow query string</label
                    >
                    <a class="text-gray-300 text-sm tracking-tight"
                        >Allow query string in requests, such as /public/stats<a
                            class="underline">?query=500</a
                        ></a
                    >
                </div>
                {#if !new_rule_allow_query_string}
                    <div
                        class="mt-5 sm:ml-6 sm:mt-0 sm:flex sm:flex-shrink-0 sm:items-center"
                    >
                        <!-- Enabled: "bg-indigo-600", Not Enabled: "bg-gray-200" -->
                        <button
                            on:click={async () =>
                                (new_rule_allow_query_string = true)}
                            type="button"
                            class="bg-gray-400 relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-600 focus:ring-offset-2"
                            role="switch"
                            aria-checked="false"
                            aria-labelledby="renew-subscription-label"
                            aria-describedby="renew-subscription-description"
                        >
                            <!-- Enabled: "translate-x-5", Not Enabled: "translate-x-0" -->
                            <span
                                aria-hidden="true"
                                class="translate-x-0 inline-block h-5 w-5 transform rounded-full bg-background shadow ring-0 transition duration-200 ease-in-out"
                            ></span>
                        </button>
                    </div>
                {:else}
                    <div
                        class="mt-5 sm:ml-6 sm:mt-0 sm:flex sm:flex-shrink-0 sm:items-center"
                    >
                        <!-- Enabled: "bg-indigo-600", Not Enabled: "bg-gray-200" -->
                        <button
                            on:click={async () =>
                                (new_rule_allow_query_string = false)}
                            type="button"
                            class="bg-indigo-600 relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-600 focus:ring-offset-2"
                            role="switch"
                            aria-checked="false"
                            aria-labelledby="renew-subscription-label"
                            aria-describedby="renew-subscription-description"
                        >
                            <!-- Enabled: "translate-x-5", Not Enabled: "translate-x-0" -->
                            <span
                                aria-hidden="true"
                                class="translate-x-5 inline-block h-5 w-5 transform rounded-full bg-background shadow ring-0 transition duration-200 ease-in-out"
                            ></span>
                        </button>
                    </div>
                {/if}
            </div>

            <div>
                <label class="block font-medium text-slate-100 pt-6"
                    >Actions</label
                >
                <a class="text-gray-300 text-sm tracking-tight"
                    >Optional and configurable actions</a
                >
                <Options
                    name={"actions"}
                    options={["Microcache", "Use ratelimit bucket"]}
                    bind:chosen_data={new_rule_actions}
                />
            </div>

            {#if new_rule_actions.includes("Microcache")}
                <div>
                    <label class="block font-medium text-slate-100 pt-6"
                        >Caching settings</label
                    >
                    <a class="text-gray-300 text-sm tracking-tight"
                        >For microcaching public endpoints on your API</a
                    >
                    <div>
                        <div>
                            <button
                                on:click={() =>
                                    (new_rule_cache_popup =
                                        !new_rule_cache_popup)}
                                type="button"
                                class="inline-flex w-full justify-center gap-x-1.5 rounded-md bg-background px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50"
                                id="menu-button"
                                aria-expanded="true"
                                aria-haspopup="true"
                            >
                                {#if !new_rule_cache_level}
                                    Set a cache level
                                {:else}
                                    {new_rule_cache_level}
                                {/if}
                                {#if new_rule_cache_popup}
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        fill="none"
                                        viewBox="0 0 24 24"
                                        stroke-width="1.5"
                                        stroke="currentColor"
                                        class="w-6 h-6"
                                    >
                                        <path
                                            in:draw
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            d="M6 18L18 6M6 6l12 12"
                                        />
                                    </svg>
                                {:else}
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        fill="none"
                                        viewBox="0 0 24 24"
                                        stroke-width="1.5"
                                        stroke="currentColor"
                                        class="w-6 h-6"
                                    >
                                        <path
                                            in:draw
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            d="M3 4.5h14.25M3 9h9.75M3 13.5h9.75m4.5-4.5v12m0 0l-3.75-3.75M17.25 21L21 17.25"
                                        />
                                    </svg>
                                {/if}
                            </button>
                        </div>

                        {#if new_rule_cache_popup}
                            <div
                                class="absolute z-10 mt-2 w-56 origin-top-right rounded-md bg-background shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none"
                                role="menu"
                                aria-orientation="vertical"
                                aria-labelledby="menu-button"
                                tabindex="-1"
                            >
                                <div class="py-1" role="none">
                                    <OneOption
                                        options={[
                                            "None",
                                            "Standard",
                                            "IgnoreQueryString",
                                            "Aggressive",
                                        ]}
                                        bind:selected={new_rule_cache_level}
                                    />
                                </div>
                            </div>
                        {/if}
                    </div>
                    <input
                        bind:value={new_rule_cache_ttl}
                        type="number"
                        min="5"
                        max="604800"
                        name="setting-name"
                        id="rule-order"
                        class="mt-1 focus:ring-indigo-500 focus:border-indigo-500 block w-full shadow-sm sm:text-sm border-fuchsia-500 rounded-md"
                        placeholder="Cache time-to-live"
                        required
                    />
                </div>
            {/if}

            {#if new_rule_actions.includes("Use ratelimit bucket")}
                <div>
                    <label class="block font-medium text-slate-100 pt-6"
                        >Ratelimiting</label
                    >
                    <a class="text-gray-300 text-sm tracking-tight"
                        >Use buckets to ratelimit IPs against this endpoint</a
                    >
                    <div>
                        <div>
                            <div>
                                <button
                                    on:click={() =>
                                        (new_rule_ratelimit_popup =
                                            !new_rule_ratelimit_popup)}
                                    type="button"
                                    class="inline-flex w-full justify-center gap-x-1.5 rounded-md bg-background px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50"
                                    id="menu-button"
                                    aria-expanded="true"
                                    aria-haspopup="true"
                                >
                                    {#if !new_rule_bucket}
                                        Set a bucket
                                    {:else}
                                        {new_rule_bucket}
                                    {/if}
                                    {#if new_rule_ratelimit_popup}
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            fill="none"
                                            viewBox="0 0 24 24"
                                            stroke-width="1.5"
                                            stroke="currentColor"
                                            class="w-6 h-6"
                                        >
                                            <path
                                                in:draw
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                d="M6 18L18 6M6 6l12 12"
                                            />
                                        </svg>
                                    {:else}
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            fill="none"
                                            viewBox="0 0 24 24"
                                            stroke-width="1.5"
                                            stroke="currentColor"
                                            class="w-6 h-6"
                                        >
                                            <path
                                                in:draw
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                d="M3 4.5h14.25M3 9h9.75M3 13.5h9.75m4.5-4.5v12m0 0l-3.75-3.75M17.25 21L21 17.25"
                                            />
                                        </svg>
                                    {/if}
                                </button>
                            </div>

                            {#if new_rule_ratelimit_popup}
                                <div
                                    class="absolute z-10 mt-2 w-56 origin-top-right rounded-md bg-background shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none"
                                    role="menu"
                                    aria-orientation="vertical"
                                    aria-labelledby="menu-button"
                                    tabindex="-1"
                                >
                                    <div class="py-1" role="none">
                                        {#if domain_info.ratelimit_buckets.length > 0}
                                            {#each domain_info.ratelimit_buckets as bucket}
                                                <button
                                                    on:click={() => {
                                                        new_rule_bucket =
                                                            bucket._id;
                                                        new_rule_ratelimit_popup = false;
                                                    }}
                                                    class="w-full hover:bg-gray-300 duration-100 text-gray-700 block px-4 py-2 text-sm"
                                                    role="menuitem"
                                                    tabindex="-1"
                                                    id="menu-item-0"
                                                    >{bucket._id} - {bucket.threshold}
                                                    per {bucket.secs}</button
                                                >
                                            {/each}
                                        {:else}
                                            <a
                                                href="/i/dash/domains/{domain_info._id}/buckets"
                                                target="_blank"
                                                class="w-full hover:bg-gray-300 duration-100 text-gray-700 block px-4 py-2 text-sm"
                                                role="menuitem"
                                                tabindex="-1"
                                                id="menu-item-0"
                                                >no buckets available - create
                                                one in a new tab</a
                                            >
                                        {/if}
                                    </div>
                                </div>
                            {/if}
                        </div>
                    </div>
                </div>
            {/if}
        </div>

        {#if new_rule_path && new_rule_order && new_rule_match_type}
            <div class="flex justify-center pt-8">
                <button
                    transition:fade
                    type="submit"
                    class="mt-3 inline-flex w-full items-center justify-center rounded-md bg-blue-500 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-fuchsia-500 duration-100 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 sm:ml-3 sm:mt-0 sm:w-auto"
                    >Stage rule that applies on {settings[
                        new_rule_setting_index
                    ]._id}{new_rule_path}</button
                >
            </div>
        {:else}
            <div class="flex justify-center pt-8">
                <p
                    class="mt-3 inline-flex w-full items-center justify-center rounded-md text-sm font-semibold text-white shadow-sm hover:bg-red-500 duration-100 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 sm:ml-3 sm:mt-0 sm:w-auto"
                >
                    Fill in at least path, match type & order to create rule
                </p>
            </div>
        {/if}
        <p class="text-sm text-slate-200">
            <br /><br />If you have any questions, our
            <a href="/support" class="text-indigo-500 hover:text-indigo-400"
                >support team</a
            >
            stands by, ready to lend a hand if you have any questions. It is staffed
            by the same people who built Packetware. You can also check the
            <a href="/docs" class="text-indigo-500 hover:text-indigo-400"
                >documentation</a
            > on API engine settings.
        </p>
    </div>
{/if}
