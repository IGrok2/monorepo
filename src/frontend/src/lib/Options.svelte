<script>
    import {draw} from "svelte/transition";

    export let name;
    // contains all the possible options
    export let options;
    // contains the selected options
    export let chosen_data;
    // optional callbacks on change
    export let delete_callback;
    export let add_callback;

    let popup;
</script>

<div>
    <button on:click={() => popup = !popup} type="button" class="z-10 inline-flex w-full justify-center gap-x-1.5 rounded-md bg-background px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50" id="menu-button" aria-expanded="true" aria-haspopup="true">
        {#if chosen_data.length === 0}
            Set {name}
        {:else}
            {#if chosen_data.length === 1}
                {chosen_data[0]}
            {:else if chosen_data.length === 2}
                {chosen_data[0]} and {chosen_data[1]}
            {:else if chosen_data.length === options.length}
                All {name}
            {:else}
                {#each chosen_data as method}
                    <!-- add comma if not last item -->
                    {#if method !== chosen_data[chosen_data.length - 1]}
                        {method},&nbsp;
                    {:else}
                        and {method}
                    {/if}
                {/each}
            {/if}
        {/if}
        {#if popup}
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                <path in:draw stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
            </svg>
        {:else}
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                <path in:draw stroke-linecap="round" stroke-linejoin="round" d="M3 4.5h14.25M3 9h9.75M3 13.5h9.75m4.5-4.5v12m0 0l-3.75-3.75M17.25 21L21 17.25" />
            </svg>
        {/if}
    </button>
</div>

{#if popup}
    <div on:click={() => popup = false} class="fixed inset-0"></div>
    <div class="absolute z-10 mt-2 w-56 origin-top-right rounded-md bg-background shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none" role="menu" aria-orientation="vertical" aria-labelledby="menu-button" tabindex="-1">
        <div class="py-1" role="none">
            <!-- Active: "bg-gray-100 text-gray-900", Not Active: "text-gray-700" -->
            {#each options as option}
                {#if chosen_data.includes(option)}
                    <!-- contains the method -->
                    <button on:click={() => { chosen_data = chosen_data.filter(item => item !== option); delete_callback(option) }} class="flex flex-1 justify-center items-center w-full bg-gray-300 text-gray-700 px-4 py-2 text-sm" role="menuitem" tabindex="-1" id="menu-item-0">
                        {option}
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                            <path in:draw stroke-linecap="round" stroke-linejoin="round" d="M4.5 12.75l6 6 9-13.5" />
                        </svg>
                    </button>
                {:else}
                    <!-- doesn't contain the method -->
                    <button on:click={() => { chosen_data = [...chosen_data, option]; add_callback(option) }} class="w-full hover:bg-gray-300 duration-100 text-gray-700 block px-4 py-2 text-sm" role="menuitem" tabindex="-1">
                        {option}
                    </button>
                {/if}
            {/each}
        </div>
    </div>
{/if}