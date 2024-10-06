<script>
    import { enhance } from "$app/forms";

    import Dialog from "$lib/components/Dialog.svelte";
    export let show;
</script>

<Dialog bind:dialog={show} title="Delete Domain">
    <!-- header, content, etc. -->

    <form method="POST" action="?/deleteDomain" use:enhance>
        <p class="mt-1 mb-2 text-sm leading-6 text-gray-400">
            Perminantly remove this domain and all assiated information or
            settings. Procede with caution.
        </p>
        <div class="flex justify-end">
            <button
                type="submit"
                class="rounded-md bg-red-500 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-red-400 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-red-500"
                >Confirm</button
            >
        </div>
    </form>
</Dialog>
