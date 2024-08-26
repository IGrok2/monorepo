<script>
    import { page } from "$app/stores";
    import Dialog from "$lib/components/Dialog.svelte";
    import Dropdown from "$lib/components/Dropdown.svelte";
    import BasicSwitch from "$lib/base/BasicSwitch.svelte";
    import { getCookie } from "$lib/utils/auth";

    import { PUBLIC_API } from "$env/static/public";
    import { toast } from "svelte-sonner";
    import OneOption from "$lib/OneOption.svelte";
    import Popup from "$lib/Popup.svelte";

    export let dialog;

    let selected = "";

    let matches = ["Exact", "Contains", "StartsWith", "EndsWith"];

    const createBody = () => {
        if (selected === "Clear all cache") {
            return {
                domain: $page.params.slug,
                all: true,
                time: new Date(),
            }
        }

        return {
            domain: $page.params.slug,
            path: { match_type: match, paths: paths },
            time: new Date(),
        }
    };

    const clearCache = async () => {
        toast.success("Attempting to clear cache...");
        try {
            const slug = $page.params.slug;

            const token = getCookie("jwt");

            await fetch(`${PUBLIC_API}/@/project/${$page.params.project}/domain/${$page.params.id}/caching/clear`, {
                method: "POST",
                headers: new Headers({
                    "content-type": "application/json",
                    Authorization: token,
                }),
                body: JSON.stringify(
                    createBody()
                ),
            }).then(async (rawBody) => {
                let body = await rawBody.json();

                if (rawBody.status !== 200) {
                    toast.error(`Failed to clear cache: ${body.message}`);
                } else {
                    toast.success(`Successfully cleared cache!`);
                }

                console.log(body);
            });
        } catch (err) {
            console.log(err);

            toast.error(`Failed: ${err}`);
        }
    };
</script>

<Dialog title="Add Your First Basic Origin">
    <div class="text-slate-300">
        Clearing the cache will remove cached resources from the CDN globally.
        <br>
        You can choose to clear all cache or clear cache by path.
    </div>
    <div class="flex justify-between my-6">
        <div class="flex items-center">
            <OneOption bind:selected={selected} options={["Clear all cache", "Clear cache by path"]} />
        </div>
        {#if selected === "Clear cache by path"}
            <button
                type="button"
                on:click={() => {
                   if (paths.length > 10) {
                       toast.error("You can only have 10 paths");
                   } else {
                    paths.push("");
                    paths = paths;
                   }
                }}
                class="bg-fuchsia-500 text-white px-3 py-1.5 mx-3 rounded text-sm"
            >
                +
            </button>
        {/if}
    </div>
    {#if selected === "Clear cache by path"}
        <p class="text-slate-300">Select your paths, then pick a match type. For example, if you wanted to remove everything from the subdomain
            api.{$page.params.slug}, then enter that as the path and select StartsWith for the match type. Another example:
            if you want to
        clear the cache for all PNGs, then enter .png and EndsWith as the match style.</p>
        {#each paths as path, index}
            <div class="mt-4 bg-gray-800 rounded-md p-4">
                <div class="flex justify-end">
                    <button
                        on:click={async () => {
                            if (paths.length > 1) {
                                paths.splice(index, 1);
                                paths = [...paths];
                            }
                        }}
                        class="text-xl text-red-500 cursor-grab"
                    >
                        𝕏
                    </button>
                </div>
                <p>
                    <label
                        for="origin-setting-host"
                        class="block font-medium text-slate-100">Path</label
                    >

                    <a class="text-gray-300 text-sm tracking-tight"
                        >The path where you'd like to alter the cache</a
                    >
                    <input
                        bind:value={path}
                        type="text"
                        name="setting-name"
                        id="origin-setting-host"
                        placeholder="example: {$page.params.slug}/image.png"
                        class="mt-1 focus:ring-indigo-500 focus:border-indigo-500 block w-full shadow-sm sm:text-sm border-fuchsia-500 rounded-md"
                        required
                    />
                </p>
            </div>
        {/each}
        <div class="mt-4 bg-gray-600 rounded-md p-4">
            <p class="mt-2">
                <label
                    for="origin-setting-host"
                    class="block font-medium text-slate-100">Match</label
                >
                <a class="text-gray-300 text-sm tracking-tight"
                    >How would you like to match cached resources</a
                >
                <Dropdown bind:tabs={matches} bind:activeTab={match} />
            </p>
        </div>
    {/if}
    {#if selected === "Clear all cache"}
        <p class="text-slate-300">Clearing all cache will remove all cached resources from the CDN globally.</p>
    {/if}

    {#if selected === "Clear cache by path" || selected === "Clear all cache"}
    <div class="flex justify-center pt-8">
        <button
            type="submit"
            class="mt-3 inline-flex w-full items-center justify-center rounded-md bg-blue-500 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-fuchsia-500 duration-100 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 sm:ml-3 sm:mt-0 sm:w-auto"
            on:click={() => {
                clearCache();
                dialog.close();
            }}>Clear Cache</button
        >
    </div>
        {/if}
</Dialog>
