<script>
    import {draw} from "svelte/transition";
    import Notifications from "$lib/components/notifications/Notifications.svelte";

    let popup = false;

    export let selected;

    export let options;
</script>

<div>
    <button on:click={() => popup = !popup} type="button" class="inline-flex w-full justify-center md:gap-x-1.5 rounded-md bg-background px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50" id="menu-button" aria-expanded="true" aria-haspopup="true">
        <p class="relative">
            {#if !selected}
                Set a match type
            {:else}
                {selected}
            {/if}
        </p>
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
    <div class="absolute z-10 mt-2 w-58 origin-top-right rounded-md bg-background shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none" role="menu" aria-orientation="vertical" aria-labelledby="menu-button" tabindex="-1">
        <div class="py-1" role="none">
            {#each options as option}
                {#if option === selected}
                    <button on:click={() => { selected = ""; popup = false }} class="flex flex-1 justify-center items-center w-full bg-gray-300 text-gray-700 px-4 py-2 text-sm" role="menuitem" tabindex="-1" id="menu-item-0">
                        {option}
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                            <path in:draw stroke-linecap="round" stroke-linejoin="round" d="M4.5 12.75l6 6 9-13.5" />
                        </svg>
                    </button>
                {:else}
                    <button on:click={() => { selected = option; popup = false }} class="w-full hover:bg-gray-300 duration-100 text-gray-700 block px-4 py-2 text-sm" role="menuitem" tabindex="-1" id="menu-item-0">{option}</button>
                {/if}
            {/each}
        </div>
    </div>
{/if}