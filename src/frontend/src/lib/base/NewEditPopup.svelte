<script>
import DocsLink from "$lib/base/DocsLink.svelte";
import BasicSwitch from "$lib/base/BasicSwitch.svelte";
import BasicLink from "$lib/base/BasicLink.svelte";
import {fade, slide} from "svelte/transition";

export let popup = false;
export let edit_popup = false;
export let name;
export let callback;
export let edit_callback;

</script>

{#if popup}
    <div in:fade out:slide|global class="relative z-40" aria-labelledby="slide-over-title" role="dialog" aria-modal="true">
        <div on:click={() => popup = false} class="fixed inset-0 bg-gray-700 bg-opacity-60 transition-opacity bg-no-repeat bg-center"></div>
        <div class="fixed inset-0 overflow-hidden">
            <div class="absolute inset-0 overflow-hidden">
                <div class="pointer-events-none fixed inset-y-0 right-0 flex max-w-full pl-10">
                    <div class="pointer-events-auto w-screen max-w-md">
                        <div class="flex h-full flex-col overflow-y-scroll bg-gray-700 py-6 shadow-xl">
                            <div class="px-4 sm:px-6">
                                <div class="flex items-start justify-between">
                                    <div class="text-slate-100">
                                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M21 7.5l-2.25-1.313M21 7.5v2.25m0-2.25l-2.25 1.313M3 7.5l2.25-1.313M3 7.5l2.25 1.313M3 7.5v2.25m9 3l2.25-1.313M12 12.75l-2.25-1.313M12 12.75V15m0 6.75l2.25-1.313M12 21.75V19.5m0 2.25l-2.25-1.313m0-16.875L12 2.25l2.25 1.313M21 14.25v2.25l-2.25 1.313m-13.5 0L3 16.5v-2.25" />
                                        </svg>
                                    </div>
                                    <h2 class="font-semibold leading-6 text-slate-100 text-xl" id="slide-over-title">Create an {name}</h2>
                                    <div class="ml-3 flex h-7 items-center">
                                        <button on:click={() => { popup = false }} type="button" class="rounded-md text-slate-100 hover:text-slate-400 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2">
                                            <span class="sr-only">Close panel</span>
                                            <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                                            </svg>
                                        </button>
                                    </div>
                                </div>
                            </div>
                            <form class="relative mt-6 flex-1 px-4 sm:px-6" on:submit|preventDefault={callback}>
                                <slot />
                            </form>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
{/if}

{#if edit_popup}
    <div in:fade out:slide|global class="relative z-40" aria-labelledby="slide-over-title" role="dialog" aria-modal="true">
        <div on:click={() => edit_popup = false} class="fixed inset-0 bg-gray-700 bg-opacity-60 transition-opacity bg-no-repeat bg-center"></div>
        <div class="fixed inset-0 overflow-hidden">
            <div class="absolute inset-0 overflow-hidden">
                <div class="pointer-events-none fixed inset-y-0 right-0 flex max-w-full pl-10">
                    <div class="pointer-events-auto w-screen max-w-md">
                        <div class="flex h-full flex-col overflow-y-scroll bg-gray-700 py-6 shadow-xl">
                            <div class="px-4 sm:px-6">
                                <div class="flex items-start justify-between">
                                    <div class="text-slate-100">
                                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M21 7.5l-2.25-1.313M21 7.5v2.25m0-2.25l-2.25 1.313M3 7.5l2.25-1.313M3 7.5l2.25 1.313M3 7.5v2.25m9 3l2.25-1.313M12 12.75l-2.25-1.313M12 12.75V15m0 6.75l2.25-1.313M12 21.75V19.5m0 2.25l-2.25-1.313m0-16.875L12 2.25l2.25 1.313M21 14.25v2.25l-2.25 1.313m-13.5 0L3 16.5v-2.25" />
                                        </svg>
                                    </div>
                                    <h2 class="font-semibold leading-6 text-slate-100 text-xl" id="slide-over-title">Edit an {name}</h2>
                                    <div class="ml-3 flex h-7 items-center">
                                        <button on:click={() => { edit_popup = false }} type="button" class="rounded-md text-slate-100 hover:text-slate-400 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2">
                                            <span class="sr-only">Close panel</span>
                                            <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                                            </svg>
                                        </button>
                                    </div>
                                </div>
                            </div>
                            <form class="relative mt-6 flex-1 px-4 sm:px-6" on:submit|preventDefault={edit_callback}>
                                <slot />
                            </form>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
{/if}
