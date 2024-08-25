<script>
    import { fade } from 'svelte/transition';


</script>

{#if show_staged_changes}
    <div
            in:fade
            class="relative z-20"
            aria-labelledby="modal-title"
            role="dialog"
            aria-modal="true"
    >
        <div
                class="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity"
        ></div>
        <div class="fixed inset-0 z-10 overflow-y-auto">
            <div
                    class="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0"
            >
                <div
                        class="relative transform overflow-hidden rounded-lg bg-background px-4 pb-4 pt-5 text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-lg sm:p-6"
                >
                    <div class="sm:flex sm:items-start">
                        <button
                                on:click={() => (show_staged_changes = undefined)}
                                class="mx-auto flex h-12 w-12 flex-shrink-0 items-center justify-center rounded-full bg-fuchsia-500 sm:mx-0 sm:h-10 sm:w-10"
                        >
                            <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    stroke-width="1.5"
                                    stroke="currentColor"
                                    class="w-6 h-6 text-white"
                            >
                                <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        d="M9 8.25H7.5a2.25 2.25 0 00-2.25 2.25v9a2.25 2.25 0 002.25 2.25h9a2.25 2.25 0 002.25-2.25v-9a2.25 2.25 0 00-2.25-2.25H15M9 12l3 3m0 0l3-3m-3 3V2.25"
                                />
                            </svg>
                        </button>
                        <div
                                class="mt-3 text-center sm:ml-4 sm:mt-0 sm:text-left"
                        >
                            <h3
                                    class="text-base font-semibold leading-6 text-gray-900"
                                    id="modal-title"
                            >
                                Push staged changes to edge
                            </h3>
                            <div class="mt-2">
                                <p class="text-sm text-gray-500">Changes:</p>
                                <ol class="list-disc">
                                    {#if human_engine_mode !== domain_info.human_engine.mode}
                                        <li>
                                            <a class="text-yellow-500">Change</a
                                            >
                                            human engine mode: {human_engine_mode}
                                        </li>
                                    {/if}
                                </ol>
                            </div>
                        </div>
                    </div>
                    <div class="mt-5 sm:ml-10 sm:mt-4 sm:flex sm:pl-4">
                        {#if save_button_success}
                            <button
                                    transition:fly={{ x: 0, y: 0 }}
                                    type="button"
                                    class="bg-green-700 inline-flex w-full justify-center rounded-md px-1.5 py-2 text-sm font-semibold text-white shadow-sm"
                            >Blasting off ...</button
                            >
                        {:else}
                            <button
                                    on:click={saveChanges}
                                    type="button"
                                    class="hover:cursor-[url(/svelte.config.cur),_copy] inline-flex w-full justify-center rounded-md bg-gradient-to-br from-blue-500 via-gray-700 to-fuchsia-500 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:from-blue-500 hover:via-amber-500 hover:to-fuchsia-500 sm:w-auto font-krona"
                            >Blast off: push changes to edge</button
                            >
                            <button
                                    on:click={() => (show_staged_changes = false)}
                                    type="button"
                                    class="mt-3 inline-flex w-full justify-center rounded-md bg-background px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50 sm:ml-3 sm:mt-0 sm:w-auto"
                            >Cancel (this doesn't lose your changes)</button
                            >
                        {/if}
                    </div>
                </div>
            </div>
        </div>
    </div>
{/if}
