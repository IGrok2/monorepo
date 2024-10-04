<script>
    import { X } from "lucide-svelte";
    export let dialog;

    export let title;

    import { fade } from "svelte/transition";
</script>

<dialog transition:fade bind:this={dialog} on:close>
    <div
        class="relative z-20 my-8 rounded-lg"
        aria-labelledby="modal-title"
        role="dialog"
        aria-modal="true"
    >
        <div
            on:click={() => dialog.close()}
            class="fixed inset-0 bg-background/80 backdrop-blur-sm"
        ></div>
        <div class="fixed inset-0 z-50 overflow-y-auto">
            <div
                class="flex min-h-full items-end justify-center p-2 py-10 text-center sm:items-center"
            >
                <div class="bg-background rounded-lg">
                    <div
                        class="relative transform overflow-hidden rounded-lg px-4 pb-4 pt-5 text-left shadow-xl transition-all sm:w-full sm:max-w-lg sm:py-8 sm:p-6"
                    >
                        <div class="flex items-start justify-between">
                            <div class="text-primary">
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
                                class="font-semibold leading-6 text-primary text-xl"
                                id="slide-over-title"
                            >
                                {title}
                            </h2>
                            <div class="ml-3 flex h-7 items-center">
                                <button
                                    on:click={() => {
                                        dialog.close();
                                    }}
                                    type="button"
                                    class="rounded-md text-primary focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
                                >
                                    <span class="sr-only">Close panel</span>
                                    <X />
                                </button>
                            </div>
                        </div>
                        <slot />
                    </div>
                </div>
            </div>
        </div>
    </div>
</dialog>
