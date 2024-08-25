<script>
    import {fade} from "svelte/transition";
    export let override_popup;
    export let show_staged_changes;
    export let domain_info;
    export let override;
    export let saveChanges;
    export let save_button_success;
</script>

{#if override_popup}
    <div in:fade class="relative z-30" aria-labelledby="modal-title" role="dialog" aria-modal="true">
        <div class="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity"></div>
        <div class="fixed inset-0 z-10 overflow-y-auto">
            <div class="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0">
                <div class="relative transform overflow-hidden rounded-lg bg-background px-4 pb-4 pt-5 text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-lg sm:p-6">
                    <div class="sm:flex sm:items-start">
                        <button on:click={() => show_staged_changes = undefined} class="mx-auto flex h-12 w-12 flex-shrink-0 items-center justify-center rounded-full bg-fuchsia-500 sm:mx-0 sm:h-10 sm:w-10">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6 text-white">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M9 8.25H7.5a2.25 2.25 0 00-2.25 2.25v9a2.25 2.25 0 002.25 2.25h9a2.25 2.25 0 002.25-2.25v-9a2.25 2.25 0 00-2.25-2.25H15M9 12l3 3m0 0l3-3m-3 3V2.25" />
                            </svg>
                        </button>
                        <div class="mt-3 text-center sm:ml-4 sm:mt-0 sm:text-left">
                            <h3 class="text-base font-semibold leading-6 text-gray-900" id="modal-title">Changes have been made since you've opened this page. Your staged changes may override these changes. Continue?</h3>
                        </div>
                    </div>
                    <div class="mt-5 sm:ml-10 sm:mt-4 sm:flex sm:pl-4">
                        <button on:click={async () => { override_popup = false; override = true; await saveChanges() }} type="button" class="mt-3 inline-flex w-full justify-center rounded-md bg-background px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50 sm:ml-3 sm:mt-0 sm:w-auto">Confirm my changes</button>
                        <a href="/i/dash/domains/{domain_info._id}/page_rules" target="_blank" type="button" class="mt-3 inline-flex w-full justify-center rounded-md bg-background px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50 sm:ml-3 sm:mt-0 sm:w-auto">See changes in a new tab</a>
                        <button on:click={() => { override_popup = false; show_staged_changes = false; save_button_success = false; saving = false; }} type="button" class="mt-3 inline-flex w-full justify-center rounded-md bg-background px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50 sm:ml-3 sm:mt-0 sm:w-auto">Cancel (this doesn't lose your changes)</button>
                    </div>
                </div>
            </div>
        </div>
    </div>
{/if}