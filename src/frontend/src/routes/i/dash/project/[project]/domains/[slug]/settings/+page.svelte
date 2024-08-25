<script>
    import { enhance } from "$app/forms";

    import OneOption from "$lib/OneOption.svelte";
    import DeleteDomain from "./(components)/DeleteDomain.svelte";

    /** @type {import('./$types').PageData} */
    export let data;

    const domain = data.resp.domain.domain;

    let templates = ["None", "Pterodactyl", "Wordpress"];
    let selectedTemplate = "None";

    let deleteDomainDialog;
</script>

<DeleteDomain bind:show={deleteDomainDialog} />

<div class="grid gap-y-8">
    <div>
        <h2 class="text-base font-semibold leading-7 text-white">
            (Alpha) Application Template
        </h2>
        <p class="mt-1 mb-2 text-sm leading-6 text-gray-400">
            Overwrite all settings, optimizations, and rules for a domain to be
            geared for a specific application.
        </p>
        <OneOption bind:options={templates} bind:selected={selectedTemplate} />
    </div>

    <div class="flex justify-between">
        <div>
            <h2 class="text-base font-semibold leading-7 text-white">
                Delete Domain
            </h2>
            <p class="mt-1 text-sm leading-6 text-gray-400">
                Perminantly remove this domain and all assiated information or
                settings.
            </p>
        </div>
        <div class="mt-8 flex">
            <form
                method="POST"
                action="?/deleteDomain"
                use:enhance={({ cancel }) => {
                    deleteDomainDialog.show();
                    // prevent the unconfirmed form submission
                    cancel();
                }}
            >
                <button
                    type="submit"
                    class="rounded-md bg-red-500 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-red-400 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-red-500"
                    >Delete</button
                >
            </form>
        </div>
    </div>
</div>
