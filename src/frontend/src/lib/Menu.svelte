<script>
    import { fade } from "svelte/transition";
    import {page} from "$app/stores";

    export let glance = false;
    //export let api_engine = false;
    export let page_rules = false;
    export let optimization = false;
    export let bot_engine = false;
    export let human_engine = false;
    export let origin_settings = false;
    export let buckets = false;
    export let analytics = false;
    export let general_settings = false;

    let api_engine_class;
    let optimization_class;
    let bot_engine_class;
    let page_rules_class;

    export let domain_info;
    /*
    if (domain_info) {
        api_engine_class = domain_info.api_engine.enabled
            ? "from-green-500"
            : "from-red-500";
        optimization_class = domain_info.caching_settings.enabled
            ? "from-green-500"
            : "from-red-500";
        bot_engine_class = domain_info.bot_management.enabled
            ? "from-green-500"
            : "from-red-500";
        page_rules_class = domain_info.rules.enabled
            ? "from-green-500"
            : "from-red-500";
    }*/
</script>

<div in:fade class="w-1/5 z-10 p-2">
    <p class="uppercase text-slate-300 backdrop-blur-sm md:p-2 hidden md:visible">settings</p>
    <ul role="list" class="flex flex-1 flex-col gap-y-4">
        {#if glance}
            <a
                role="list"
                href="/i/dash/domains/{$page.params.slug}/"
                class="hover:italic bg-fuchsia-500 text-white group flex md:gap-x-3 rounded-md md:p-2 text-sm leading-6 font-semibold border-2"
            >
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    class="w-6 h-6"
                >
                    <path
                        fill-rule="evenodd"
                        d="M14.615 1.595a.75.75 0 01.359.852L12.982 9.75h7.268a.75.75 0 01.548 1.262l-10.5 11.25a.75.75 0 01-1.272-.71l1.992-7.302H3.75a.75.75 0 01-.548-1.262l10.5-11.25a.75.75 0 01.913-.143z"
                        clip-rule="evenodd"
                    />
                </svg>

                at a glance
            </a>
        {:else}
            <a
                role="list"
                href="/i/dash/domains/{$page.params.slug}/"
                class="hover:italic bg-fuchsia-500 text-white group flex md:gap-x-3 rounded-md md:p-2 text-sm leading-6 font-semibold border-2 border-dotted hover:border-dashed hover:bg-fuchsia-600"
            >
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    class="w-6 h-6"
                >
                    <path
                        fill-rule="evenodd"
                        d="M14.615 1.595a.75.75 0 01.359.852L12.982 9.75h7.268a.75.75 0 01.548 1.262l-10.5 11.25a.75.75 0 01-1.272-.71l1.992-7.302H3.75a.75.75 0 01-.548-1.262l10.5-11.25a.75.75 0 01.913-.143z"
                        clip-rule="evenodd"
                    />
                </svg>

                at a glance
            </a>
        {/if}
        <!--
        {#if api_engine}
        <a role="list" class="hover:italic bg-gradient-to-r {api_engine_class} to-fuchsia-700 text-white group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold border-2"
           href="./api_engine">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                <path stroke-linecap="round" stroke-linejoin="round" d="M21 7.5l-2.25-1.313M21 7.5v2.25m0-2.25l-2.25 1.313M3 7.5l2.25-1.313M3 7.5l2.25 1.313M3 7.5v2.25m9 3l2.25-1.313M12 12.75l-2.25-1.313M12 12.75V15m0 6.75l2.25-1.313M12 21.75V19.5m0 2.25l-2.25-1.313m0-16.875L12 2.25l2.25 1.313M21 14.25v2.25l-2.25 1.313m-13.5 0L3 16.5v-2.25" />
            </svg>

            api engine
        </a>
        {:else}
            <a role="list" class="hover:italic bg-gradient-to-r {api_engine_class} to-fuchsia-700 text-white group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold border-2 border-dotted hover:border-dashed hover:bg-fuchsia-600"
               href="./api_engine">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M21 7.5l-2.25-1.313M21 7.5v2.25m0-2.25l-2.25 1.313M3 7.5l2.25-1.313M3 7.5l2.25 1.313M3 7.5v2.25m9 3l2.25-1.313M12 12.75l-2.25-1.313M12 12.75V15m0 6.75l2.25-1.313M12 21.75V19.5m0 2.25l-2.25-1.313m0-16.875L12 2.25l2.25 1.313M21 14.25v2.25l-2.25 1.313m-13.5 0L3 16.5v-2.25" />
                </svg>

                api engine
            </a>
        {/if}
         
        {#if page_rules}
            <a role="list" class="hover:italic bg-gradient-to-r {page_rules_class} to-fuchsia-700 text-white group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold border-2"
               href="./page_rules">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M16.862 4.487l1.687-1.688a1.875 1.875 0 112.652 2.652L6.832 19.82a4.5 4.5 0 01-1.897 1.13l-2.685.8.8-2.685a4.5 4.5 0 011.13-1.897L16.863 4.487zm0 0L19.5 7.125" />
                </svg>

                page rules
            </a>
        {:else}
            <a role="list" class="hover:italic bg-gradient-to-r {page_rules_class} to-fuchsia-700 text-white group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold border-2 border-dotted hover:border-dashed hover:bg-fuchsia-600"
               href="./page_rules">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M16.862 4.487l1.687-1.688a1.875 1.875 0 112.652 2.652L6.832 19.82a4.5 4.5 0 01-1.897 1.13l-2.685.8.8-2.685a4.5 4.5 0 011.13-1.897L16.863 4.487zm0 0L19.5 7.125" />
                </svg>

                page rules
            </a>
        {/if}

        {#if optimization}
            <a role="list" class="hover:italic bg-gradient-to-r {optimization_class} to-fuchsia-700 text-white group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold border-2"
               href="./optimization">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M9.348 14.651a3.75 3.75 0 010-5.303m5.304 0a3.75 3.75 0 010 5.303m-7.425 2.122a6.75 6.75 0 010-9.546m9.546 0a6.75 6.75 0 010 9.546M5.106 18.894c-3.808-3.808-3.808-9.98 0-13.789m13.788 0c3.808 3.808 3.808 9.981 0 13.79M12 12h.008v.007H12V12zm.375 0a.375.375 0 11-.75 0 .375.375 0 01.75 0z" />
                </svg>

                cache & optimization
            </a>
        {:else}
            <a role="list" class="hover:italic bg-gradient-to-r {optimization_class} to-fuchsia-700 text-white group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold border-2 border-dotted hover:border-dashed hover:bg-fuchsia-600"
               href="./optimization">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M9.348 14.651a3.75 3.75 0 010-5.303m5.304 0a3.75 3.75 0 010 5.303m-7.425 2.122a6.75 6.75 0 010-9.546m9.546 0a6.75 6.75 0 010 9.546M5.106 18.894c-3.808-3.808-3.808-9.98 0-13.789m13.788 0c3.808 3.808 3.808 9.981 0 13.79M12 12h.008v.007H12V12zm.375 0a.375.375 0 11-.75 0 .375.375 0 01.75 0z" />
                </svg>

                cache & optimization
            </a>
        {/if}

        {#if bot_engine}
            <a role="list" class="hover:italic bg-gradient-to-r {bot_engine_class} to-fuchsia-700 text-white group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold border-2"
               href="./bot_management">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M6.75 7.5l3 2.25-3 2.25m4.5 0h3m-9 8.25h13.5A2.25 2.25 0 0021 18V6a2.25 2.25 0 00-2.25-2.25H5.25A2.25 2.25 0 003 6v12a2.25 2.25 0 002.25 2.25z" />
                </svg>

                bot engine
            </a>
        {:else}
            <a role="list" class="hover:italic bg-gradient-to-r {bot_engine_class} to-fuchsia-700 text-white group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold border-2 border-dotted hover:border-dashed hover:bg-fuchsia-600"
               href="./bot_management">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M6.75 7.5l3 2.25-3 2.25m4.5 0h3m-9 8.25h13.5A2.25 2.25 0 0021 18V6a2.25 2.25 0 00-2.25-2.25H5.25A2.25 2.25 0 003 6v12a2.25 2.25 0 002.25 2.25z" />
                </svg>

                bot engine
            </a>
        {/if}
        -->
        {#if human_engine}
            <a
                role="list"
                class="hover:italic bg-fuchsia-500 text-white group flex md:gap-x-3 rounded-md md:p-2 text-sm leading-6 font-semibold border-2 hover:bg-fuchsia-600"
                href="/i/dash/domains/{$page.params.slug}/human_engine"
            >
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
                        d="M16.5 10.5V6.75a4.5 4.5 0 10-9 0v3.75m-.75 11.25h10.5a2.25 2.25 0 002.25-2.25v-6.75a2.25 2.25 0 00-2.25-2.25H6.75a2.25 2.25 0 00-2.25 2.25v6.75a2.25 2.25 0 002.25 2.25z"
                    />
                </svg>

                human engine
            </a>
        {:else}
            <a
                role="list"
                class="hover:italic bg-fuchsia-500 text-white group flex md:gap-x-3 rounded-md md:p-2 text-sm leading-6 font-semibold border-2 border-dotted hover:border-dashed hover:bg-fuchsia-600"
                href="/i/dash/domains/{$page.params.slug}/human_engine"
            >
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
                        d="M16.5 10.5V6.75a4.5 4.5 0 10-9 0v3.75m-.75 11.25h10.5a2.25 2.25 0 002.25-2.25v-6.75a2.25 2.25 0 00-2.25-2.25H6.75a2.25 2.25 0 00-2.25 2.25v6.75a2.25 2.25 0 002.25 2.25z"
                    />
                </svg>

                human engine
            </a>
        {/if}

        {#if origin_settings}
            <a
                role="list"
                class="hover:italic bg-fuchsia-500 text-white group flex md:gap-x-3 rounded-md md:p-2 text-sm leading-6 font-semibold border-2 hover:bg-fuchsia-600"
                href="/i/dash/domains/{$page.params.slug}/origins"
            >
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
                        d="M6.75 7.5l3 2.25-3 2.25m4.5 0h3m-9 8.25h13.5A2.25 2.25 0 0021 18V6a2.25 2.25 0 00-2.25-2.25H5.25A2.25 2.25 0 003 6v12a2.25 2.25 0 002.25 2.25z"
                    />
                </svg>

                origin settings
            </a>
        {:else}
            <a
                role="list"
                class="hover:italic bg-fuchsia-500 text-white group flex md:gap-x-3 rounded-md md:p-2 text-sm leading-6 font-semibold border-2 border-dotted hover:border-dashed hover:bg-fuchsia-600"
                href="/i/dash/domains/{$page.params.slug}/origins"
            >
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
                        d="M6.75 7.5l3 2.25-3 2.25m4.5 0h3m-9 8.25h13.5A2.25 2.25 0 0021 18V6a2.25 2.25 0 00-2.25-2.25H5.25A2.25 2.25 0 003 6v12a2.25 2.25 0 002.25 2.25z"
                    />
                </svg>

                origin settings
            </a>
        {/if}
        <!-- 
        {#if old_buckets}
            <a role="list" class="hover:italic bg-fuchsia-500 text-white group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold border-2 hover:bg-fuchsia-600"
               href="./old_buckets">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M6.75 7.5l3 2.25-3 2.25m4.5 0h3m-9 8.25h13.5A2.25 2.25 0 0021 18V6a2.25 2.25 0 00-2.25-2.25H5.25A2.25 2.25 0 003 6v12a2.25 2.25 0 002.25 2.25z" />
                </svg>

                old_buckets
            </a>
        {:else}
            <a role="list" class="hover:italic bg-fuchsia-500 text-white group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold border-2 border-dotted hover:border-dashed hover:bg-fuchsia-600"
               href="./old_buckets">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M6.75 7.5l3 2.25-3 2.25m4.5 0h3m-9 8.25h13.5A2.25 2.25 0 0021 18V6a2.25 2.25 0 00-2.25-2.25H5.25A2.25 2.25 0 003 6v12a2.25 2.25 0 002.25 2.25z" />
                </svg>

                old_buckets
            </a>
        {/if}
            
        {#if analytics}
            <a
                role="list"
                class="hover:italic bg-fuchsia-500 text-white group flex md:gap-x-3 rounded-md md:p-2 text-sm leading-6 font-semibold border-2 hover:bg-fuchsia-600"
                href="/i/dash/domains/{$page.params.slug}/analytics"
            >
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
                        d="M3 13.125C3 12.504 3.504 12 4.125 12h2.25c.621 0 1.125.504 1.125 1.125v6.75C7.5 20.496 6.996 21 6.375 21h-2.25A1.125 1.125 0 013 19.875v-6.75zM9.75 8.625c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125v11.25c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V8.625zM16.5 4.125c0-.621.504-1.125 1.125-1.125h2.25C20.496 3 21 3.504 21 4.125v15.75c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V4.125z"
                    />
                </svg>

                analytics
            </a>
        
        {:else}
            <a
                role="list"
                class="hover:italic bg-fuchsia-500 text-white group flex md:gap-x-3 rounded-md md:p-2 text-sm leading-6 font-semibold border-2 border-dotted hover:border-dashed hover:bg-fuchsia-600"
                href="/i/dash/domains/{$page.params.slug}/analytics"
            >
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
                        d="M3 13.125C3 12.504 3.504 12 4.125 12h2.25c.621 0 1.125.504 1.125 1.125v6.75C7.5 20.496 6.996 21 6.375 21h-2.25A1.125 1.125 0 013 19.875v-6.75zM9.75 8.625c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125v11.25c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V8.625zM16.5 4.125c0-.621.504-1.125 1.125-1.125h2.25C20.496 3 21 3.504 21 4.125v15.75c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V4.125z"
                    />
                </svg>

                analytics
            </a>
        {/if}
            -->
        <a
            role="list"
            class="hover:italic bg-fuchsia-500 text-white group flex md:gap-x-3 rounded-md md:p-2 text-sm leading-6 font-semibold border-2 border-dotted hover:border-dashed hover:bg-fuchsia-600"
            href="/i/dash/domains/{$page.params.slug}/settings"
        >
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
                    d="M4.5 12a7.5 7.5 0 0015 0m-15 0a7.5 7.5 0 1115 0m-15 0H3m16.5 0H21m-1.5 0H12m-8.457 3.077l1.41-.513m14.095-5.13l1.41-.513M5.106 17.785l1.15-.964m11.49-9.642l1.149-.964M7.501 19.795l.75-1.3m7.5-12.99l.75-1.3m-6.063 16.658l.26-1.477m2.605-14.772l.26-1.477m0 17.726l-.26-1.477M10.698 4.614l-.26-1.477M16.5 19.794l-.75-1.299M7.5 4.205L12 12m6.894 5.785l-1.149-.964M6.256 7.178l-1.15-.964m15.352 8.864l-1.41-.513M4.954 9.435l-1.41-.514M12.002 12l-3.75 6.495"
                />
            </svg>

            general settings
        </a>
    </ul>
</div>
