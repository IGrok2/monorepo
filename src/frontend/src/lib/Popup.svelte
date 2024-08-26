<script>
    import { fade } from 'svelte/transition';

    export let bg_pattern = "hideout.svg";

    export let bg_color = "bg-black";

    export let no_repeat = true;
</script>

<div transition:fade class="relative z-20" aria-labelledby="modal-title" role="dialog" aria-modal="true">
    <div class="fixed inset-0 bg-gray-950 bg-opacity-60 transition-opacity"></div>

    <div class="fixed inset-0 z-10 overflow-y-auto">
        <div class="flex min-h-full items-end justify-center text-center sm:items-center sm:p-0 overflow-hidden">
            <div class="sm:my-8 sm:w-full sm:max-w-4xl sm:p-6">
                <div class="{bg_color}">
                    <div class="bg-gray-900 bg-opacity-50">
                        <div class="p-4 relative transform rounded-lg px-4 pb-4 pt-5 text-left shadow-xl transition-all">
                            <div class="sm:flex sm:items-start lg:bg-[url('/{bg_pattern}')] {(no_repeat === true) ? 'bg-no-repeat' : `bg-repeat`} bg-center">
                                <div class="mt-3 text-center sm:ml-4 sm:mt-0 sm:text-left">
                                    <slot />
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>
