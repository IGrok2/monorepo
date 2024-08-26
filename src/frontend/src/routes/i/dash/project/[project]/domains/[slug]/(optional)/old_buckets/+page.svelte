<script>
    import { page } from '$app/stores';
    import {onMount} from "svelte";
    import { fade, fly, slide, draw } from 'svelte/transition';
    import { getCookie } from "$lib/utils/auth";
    import Options from "$lib/Options.svelte";
    import Menu from "$lib/Menu.svelte";
    import BasicSwitch from "$lib/base/BasicSwitch.svelte";
    import DocsLink from "$lib/base/DocsLink.svelte";
    import BasicLink from "$lib/base/BasicLink.svelte";
    import NewEditPopup from "$lib/base/NewEditPopup.svelte";
    import OverridePopup from "$lib/base/OverridePopup.svelte";

    // For loading the dashboard
    let domain_info;
    let loaded = false;

    // Basic saving class properties
    let saved_class = "cursor-not-allowed bg-gray-400";
    let saving = false;
    let able_to_save = false;
    let confirmation_title;
    let confirmation_message;
    let confirmation_function;

    // For CSS classes
    let api_engine = "";
    let cache_engine = "";
    let bot_engine = "";
    let page_rules = "";

    // Bucket specific settings
    let buckets = [];

    // Bucket helpers
    let new_bucket_popup = false;
    let edit_bucket_popup = false;

    let new_bucket_name;
    let new_bucket_threshold;
    let new_bucket_secs;
    let edit_bucket_id;

    const load = async () => {
        try {
            loaded = false;

            const slug = $page.params.slug;

            const token = getCookie("jwt")

            await fetch("https://iucuwv-ip-24-189-107-128.tunnelmole.net/@/domain", {
                method: "POST",
                headers: new Headers({'content-type': 'application/json', 'Authorization': token}),
                body: JSON.stringify({
                    domain: slug
                })
            }).then(async (rawBody) => {
                let body = await rawBody.json();

                time_based = Date.now();

                domain_info = body.domain;

                if (domain_info.api_engine.enabled) {
                    api_engine = "from-green-500";
                } else {
                    api_engine = "from-red-500";
                };

                if (domain_info.caching_settings.enabled) {
                    cache_engine = "from-green-500";
                } else {
                    cache_engine = "from-red-500";
                };

                if (domain_info.bot_management.enabled) {
                    bot_engine = "from-green-500";
                } else {
                    bot_engine = "from-red-500";
                };

                if (domain_info.rules.enabled) {
                    page_rules = "from-green-500";
                } else {
                    page_rules = "from-red-500";
                };

                buckets = domain_info.ratelimit_buckets;

                for (let i = 0; i < buckets.length; ++i) {
                    for (let v = 0; v < body.buckets.length; v++) {
                        if (body.buckets[v].name === buckets[i]._id) {
                            buckets[i].passed = body.buckets[v].passed;
                            buckets[i].rl = body.buckets[v].rl;
                        }
                    }

                    buckets[i].changed = {};
                }

                loaded = true;
            })
        } catch (err) {
            console.log(err);

            await error_notification("Error loading user profile", "Redirecting to login page ...")

            document.location.href = "/i/auth/login";
        }
    }

    onMount(load);

    const deleteBucket = async (name) => {
        for (let i = 0; i < buckets.length; i++) {
            if (buckets[i]._id === name) {
                // was the bucket staged, if so, slice
                if (buckets[i].changed.new_staged) {
                    await confirmChange("Delete staged bucket?", "Because this bucket was staged, nothing will differ in your production environment", () => {
                        buckets.splice(i, 1);
                        buckets = buckets;
                    })
                } else {
                    await confirmChange("Delete production bucket?", "This bucket will be staged for deletion", () => {
                        buckets[i].changed.being_deleted = true;
                        buckets = buckets;
                    })
                }
            }
        }
    }

    const getNewBuckets = () => {
        let new_buckets = [];

        for (let i = 0; i < buckets.length; i++) {
            if (buckets[i].changed.new_staged) {
                new_buckets.push(buckets[i])
            }
        }

        return new_buckets;
    }

    const getDeletedBuckets = () => {
        let deleted_buckets = [];

        for (let i = 0; i < buckets.length; i++) {
            if (buckets[i].changed.being_deleted) {
                deleted_buckets.push(buckets[i])
            }
        }

        return deleted_buckets;
    }

    const getChangedBuckets = () => {
        let changed_buckets = [];

        for (let i = 0; i < buckets.length; i++) {
            if (buckets[i].changed.existed_modified) {
                changed_buckets.push(buckets[i]);
            }
        }

        return changed_buckets
    }

    const letSaveChanges = () => {
        let local_able_to_save = getChangedBuckets().length !== 0 || getNewBuckets().length !== 0
                || getDeletedBuckets().length !== 0;

        if (local_able_to_save) {
            able_to_save = true;
            saved_class = "bg-fuchsia-500";
        } else {
            able_to_save = false;
            saved_class = "cursor-not-allowed bg-gray-400"
        }
    }

    const newBucket = () => {
        if (!new_bucket_name || new_bucket_name.length <= 3 || !new_bucket_threshold || new_bucket_threshold > 1000 || !new_bucket_secs || new_bucket_secs > 86400) {
            error_notification("Couldn't stage new bucket", "Didn't have critical info, or info was extreme. Make sure the name is at least 3 characters, the threshold is less than 1000, and the seconds is less than 86400")
            return;
        }

        // see if the name has already been used
        for (let i = 0; i < buckets.length; i++) {
            if (buckets[i]._id === new_bucket_name) {
                error_notification("Couldn't stage new bucket", "There is a bucket with the same name")
                return;
            }
        }

        new_bucket_popup = false;

        buckets.push({
            changed: {
                new_staged: true
            },
            _id: new_bucket_name,
            threshold: new_bucket_threshold,
            secs: new_bucket_secs
        })

        buckets = buckets;

        letSaveChanges()
    }

    const editBucket = () => {
        if (!new_bucket_name || new_bucket_name.length <= 3 || !new_bucket_threshold || new_bucket_threshold > 1000 || !new_bucket_secs || new_bucket_secs > 86400) {
            error_notification("Couldn't change bucket", "Didn't have critical info, or info was extreme. Make sure the name is at least 3 characters, the threshold is less than 1000, and the seconds is less than 86400")
            return;
        }

        // see if the name has already been used
        for (let i = 0; i < buckets.length; i++) {
            if (buckets[i]._id === new_bucket_name && edit_bucket_id !== i) {
                error_notification("Couldn't change bucket", "There is a bucket with the same name")
                return;
            }
        }

        edit_bucket_popup = false;

        buckets.splice(edit_bucket_id, 1);

        buckets.push({
            changed: {
                existed_modified: true,
            },
            _id: new_bucket_name,
            threshold: new_bucket_threshold,
            secs: new_bucket_secs
        })

        buckets = buckets;

        letSaveChanges()
    }

    const setupEdit = (index) => {
        const bucket = buckets[index];

        new_bucket_name = bucket._id;
        new_bucket_secs = bucket.secs;
        new_bucket_threshold = bucket.threshold;
        edit_bucket_id = index;

        edit_bucket_popup = true;
    }

    let time_based;
    let override;
    let override_popup;

    const saveChanges = async () => {
        // save changes to bot settings
        if (able_to_save === true) {
            let success = false;
            saving = true;
            save_button_success = true;

            try {
                const slug = $page.params.slug;

                const token = getCookie("jwt");

                for (let i = 0; i < buckets.length; ++i) {
                    delete buckets[i].passed;
                    delete buckets[i].rl;

                    if (buckets[i].changed.being_deleted) {
                        buckets.splice(i, 1);
                        i--;
                    } else {
                        delete buckets[i].changed;
                    }
                }

                (async () => {
                    await fetch("https://iucuwv-ip-24-189-107-128.tunnelmole.net/@/domain/buckets/update", {
                        method: "POST",
                        headers: new Headers({'content-type': 'application/json', 'Authorization': token}),
                        body: JSON.stringify({
                            domain: slug,
                            buckets,
                            time: time_based,
                            __foo_confirm: override
                        }),
                    })
                        .then(async (res) => {
                            let response = await res.json();
                            if (res.status !== 200) {
                                if (response.message === "Changes have been made since data last checked. Confirm changes? Open new changes in a new tab?") {
                                    override_popup = true;
                                } else {
                                    await error_notification("Failed to update settings", `Error: ${response.message}`);
                                }
                            } else {
                                success = true;
                            }
                        }).catch(async (err) => {
                            saving = false;
                            show_staged_changes = false;
                            save_button_success = false;
                            await error_notification("Failed to update settings", `Try refreshing the page? ${err}`)
                        })

                })().then(async () => {
                    if (success) {
                        saving = false;
                        show_staged_changes = false;
                        save_button_success = false;
                        await load();
                        letSaveChanges();
                        await notification("Updated settings", "Pushed changes to edge")
                    }
                })

            } catch (err) {
                saving = false;
                show_staged_changes = false;
                save_button_success = false;
                await error_notification("Failed to update settings", `Try refreshing the page? ${err}`)
            }
        }
    }


/*
    const saveChanges = async () => {
        if (able_to_save === true) {
            saving = true;
            save_button_success = true;
            try {
                const slug = $page.params.slug;

                const token = getCookie("jwt");

                (async () => {
                    const changed_buckets = getChangedBuckets();
                    const new_buckets = getNewBuckets();
                    const deleted_buckets = getDeletedBuckets();

                    if (deleted_buckets.length !== 0) {
                        for (let i = 0; i < deleted_buckets.length; i++) {
                            await fetch("https://iucuwv-ip-24-189-107-128.tunnelmole.net/@/domain/buckets/delete", {
                                method: "POST",
                                headers: new Headers({'content-type': 'application/json', 'Authorization': token}),
                                body: JSON.stringify({
                                    domain: slug,
                                    id: deleted_buckets[i]._id
                                }),
                            })
                                .then(async (res) => {
                                    let response = await res.json();
                                    if (res.status !== 200) {
                                        await error_notification("Failed to update settings", `Error: ${response.message}`);
                                        return;
                                    }
                                }).catch(async (err) => {
                                    saving = false;
                                    show_staged_changes = false;
                                    save_button_success = false;
                                    await error_notification("Failed to update settings", `Try refreshing the page? ${err}`)
                                    return;
                                })
                        }
                    }

                    if (new_buckets.length !== 0) {
                        for (let i = 0; i < new_buckets.length; i++) {
                            await fetch("https://iucuwv-ip-24-189-107-128.tunnelmole.net/@/domain/buckets/new", {
                                method: "POST",
                                headers: new Headers({'content-type': 'application/json', 'Authorization': token}),
                                body: JSON.stringify({
                                    domain: slug,
                                    id: new_buckets[i]._id,
                                    threshold: new_buckets[i].threshold,
                                    secs: new_buckets[i].secs
                                }),
                            })
                                .then(async (res) => {
                                    let response = await res.json();
                                    if (res.status !== 200) {
                                        await error_notification("Failed to update settings", `Error: ${response.message}`);
                                        return;
                                    }
                                }).catch(async (err) => {
                                    saving = false;
                                    show_staged_changes = false;
                                    save_button_success = false;
                                    await error_notification("Failed to update settings", `Try refreshing the page? ${err}`);
                                    return;
                                })
                        }
                    }


                    if (changed_buckets.length !== 0) {
                        for (let i = 0; i < changed_buckets.length; i++) {
                            await fetch("https://iucuwv-ip-24-189-107-128.tunnelmole.net/@/domain/buckets/update", {
                                method: "POST",
                                headers: new Headers({'content-type': 'application/json', 'Authorization': token}),
                                body: JSON.stringify({
                                    domain: slug,
                                    id: changed_buckets[i]._id,
                                    threshold: changed_buckets.changed.threshold ? changed_buckets[i].changed.threshold : changed_buckets[i].threshold,
                                    secs: changed_buckets.changed.secs ? changed_buckets[i].changed.secs : changed_buckets[i].secs
                                }),
                            })
                                .then(async (res) => {
                                    let response = await res.json();
                                    if (res.status !== 200) {
                                        await error_notification("Failed to update settings", `Error: ${response.message}`);
                                        return;
                                    }
                                }).catch(async (err) => {
                                    saving = false;
                                    show_staged_changes = false;
                                    save_button_success = false;
                                    await error_notification("Failed to update settings", `Try refreshing the page? ${err}`);
                                    return
                                })
                        }
                    }
                })().then(async () => {
                    saving = false;
                    show_staged_changes = false;
                    save_button_success = false;
                    able_to_save = false;
                    await notification("Updated settings", "Pushed changes to edge")
                    if (!able_to_save) {
                        location.reload()
                    }
                })
            } catch (err) {
                saving = false;
                show_staged_changes = false;
                save_button_success = false;
                await error_notification("Failed to update settings", `Try refreshing the page? ${err}`)
            }
        } else {
            show_staged_changes = false;

            await error_notification("Wasn't able to save changes", "Nothing was changed")
        }
    }

 */

    let classes;
    let message;
    let submessage;

    let error;
    let error_submessage;

    const notification = async (newMsg, sub) => {
        classes = "transform ease-out duration-300 transition translate-y-0 opacity-100 sm:translate-x-0"

        message = newMsg;
        submessage = sub;

        await new Promise(resolve => setTimeout(resolve, 3000));

        classes = "transition ease-in duration-100 opacity-0";

        message = undefined;
    }

    const error_notification = async (newMsg, sub) => {
        classes = "transform ease-out duration-300 transition translate-y-0 opacity-100 sm:translate-x-0"

        error = newMsg;
        error_submessage = sub;

        await new Promise(resolve => setTimeout(resolve, 7000));

        classes = "transition ease-in duration-100 opacity-0";

        error = undefined;
    }

    const confirmChange = async (title, message, func) => {
        confirmation_title = title;
        confirmation_message = message;
        confirmation_function = func;
    }

    let show_staged_changes = false;
    let save_button_success = false;

    const beforeUnload = () => {
        if (able_to_save === true) {
            // Cancel the event as stated by the standard.
            event.preventDefault();
            // Chrome requires returnValue to be set.
            event.returnValue = "There are staged changes not pushed";
            // more compatibility
            return "There are staged changes not pushed";
        }
    }
</script>

<svelte:window on:beforeunload={beforeUnload}/>

<OverridePopup bind:override_popup={override_popup} bind:show_staged_changes={show_staged_changes} bind:domain_info={domain_info}
               bind:override={override} saveChanges={saveChanges} bind:save_button_success={save_button_success} />

{#if show_staged_changes}
    <div in:fade class="relative z-20" aria-labelledby="modal-title" role="dialog" aria-modal="true">
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
                            <h3 class="text-base font-semibold leading-6 text-gray-900" id="modal-title">Push staged changes to edge</h3>
                            <div class="mt-2">
                                <p class="text-sm text-gray-500">Changes:</p>
                                <ol class="list-disc">
                                    {#if getDeletedBuckets().length !== 0}
                                        <li><a class="text-red-500">Delete</a> buckets:</li>
                                        <ol class="list-disc pl-6">
                                            {#each getDeletedBuckets() as bucket}
                                                <li class="font-light">{bucket._id}</li>
                                            {/each}
                                        </ol>
                                    {/if}

                                    {#if getChangedBuckets().length !== 0}
                                        <li><a class="text-yellow-500">Modify</a> buckets:</li>
                                        <ol class="list-disc pl-6">
                                            {#each getChangedBuckets() as bucket}
                                                <li class="font-light">{bucket._id}</li>
                                                <div class="pl-8">
                                                    {#if bucket.changed.threshold}
                                                        <p class="text-yellow-600">Threshold: {bucket.threshold}</p>
                                                    {:else if bucket.changed.secs}
                                                        <p class="text-yellow-600">Every: {bucket.secs}</p>
                                                    {/if}
                                                </div>
                                            {/each}
                                        </ol>
                                    {/if}

                                    {#if getNewBuckets().length !== 0}
                                        <li><a class="text-green-500">Create</a> buckets:</li>
                                        <ol class="list-disc pl-6">
                                            {#each getNewBuckets() as bucket}
                                                <li class="font-light">{bucket._id}</li>
                                                <div class="pl-8">
                                                    <p class="text-gray-700">Threshold: {bucket.threshold}</p>
                                                    <p class="text-gray-700">Every: {bucket.secs}</p>
                                                </div>
                                            {/each}
                                        </ol>
                                    {/if}
                                </ol>
                            </div>
                        </div>
                    </div>
                    <div class="mt-5 sm:ml-10 sm:mt-4 sm:flex sm:pl-4">
                        {#if save_button_success}
                            <button transition:fly="{{ x: 0, y: 0 }}" type="button" class="bg-green-700 inline-flex w-full justify-center rounded-md px-1.5 py-2 text-sm font-semibold text-white shadow-sm">Blasting off ...</button>
                        {:else}
                            <button on:click={saveChanges} type="button" class="hover:cursor-[url(/svelte.config.cur),_copy] inline-flex w-full justify-center rounded-md bg-gradient-to-br from-blue-500 via-gray-700 to-fuchsia-500 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:from-blue-500 hover:via-amber-500 hover:to-fuchsia-500 sm:w-auto font-krona">Blast off: push changes to edge</button>
                            <button on:click={() => show_staged_changes = false} type="button" class="mt-3 inline-flex w-full justify-center rounded-md bg-background px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50 sm:ml-3 sm:mt-0 sm:w-auto">Cancel (this doesn't lose your changes)</button>
                        {/if}
                    </div>
                </div>
            </div>
        </div>
    </div>
{/if}

{#if confirmation_title}
    <div transition:fade class="relative z-20" aria-labelledby="modal-title" role="dialog" aria-modal="true">
        <div class="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity"></div>

        <div class="fixed inset-0 z-10 overflow-y-auto">
            <div class="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0">
                <div class="relative transform overflow-hidden rounded-lg bg-background px-4 pb-4 pt-5 text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-lg sm:p-6">
                    <div class="sm:flex sm:items-start">
                        <button on:click={() => confirmation_title = undefined} class="mx-auto flex h-12 w-12 flex-shrink-0 items-center justify-center rounded-full bg-red-100 sm:mx-0 sm:h-10 sm:w-10">
                            <svg class="h-6 w-6 text-red-600" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z" />
                            </svg>
                        </button>
                        <div class="mt-3 text-center sm:ml-4 sm:mt-0 sm:text-left">
                            <h3 class="text-base font-semibold leading-6 text-gray-900" id="modal-title">{confirmation_title}</h3>
                            <div class="mt-2">
                                <p class="text-sm text-gray-500">{confirmation_message}</p>
                            </div>
                        </div>
                    </div>
                    <div class="mt-5 sm:ml-10 sm:mt-4 sm:flex sm:pl-4">
                        <button on:click={async () => {
                            confirmation_function();
                            confirmation_title = undefined;
                            await letSaveChanges()
                        }} type="button" class="inline-flex w-full justify-center rounded-md bg-red-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-red-500 sm:w-auto">Yes, I'd like to do this</button>
                        <button on:click={() => confirmation_title = undefined} type="button" class="mt-3 inline-flex w-full justify-center rounded-md bg-background px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50 sm:ml-3 sm:mt-0 sm:w-auto">Cancel</button>
                    </div>
                </div>
            </div>
        </div>
    </div>
{/if}

<div aria-live="assertive" class="z-50 pointer-events-none fixed inset-0 flex items-end px-4 py-6 sm:items-start sm:p-6">
    <div class="flex w-full flex-col items-center space-y-4 sm:items-end transition ease-in duration-100 ${classes}">

        {#if message}
            <div transition:fade class="{classes} pointer-events-auto w-full max-w-sm overflow-hidden rounded-lg bg-background shadow-lg ring-1 ring-black ring-opacity-5">
                <div class="p-4">
                    <div class="flex items-start">
                        <div class="flex-shrink-0">
                            <svg class="h-6 w-6 text-green-400" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M9 12.75L11.25 15 15 9.75M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                            </svg>
                        </div>
                        <div class="ml-3 w-0 flex-1 pt-0.5">
                            <p class="text-sm font-medium text-gray-900">{message}</p>
                            <p class="mt-1 text-sm text-gray-500">{submessage}</p>
                        </div>
                        <div class="ml-4 flex flex-shrink-0">
                            <button on:click={() => { message = undefined }} type="button" class="inline-flex rounded-md bg-background text-gray-400 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2">
                                <span class="sr-only">Close</span>
                                <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                    <path d="M6.28 5.22a.75.75 0 00-1.06 1.06L8.94 10l-3.72 3.72a.75.75 0 101.06 1.06L10 11.06l3.72 3.72a.75.75 0 101.06-1.06L11.06 10l3.72-3.72a.75.75 0 00-1.06-1.06L10 8.94 6.28 5.22z" />
                                </svg>
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        {/if}

        {#if error}
            <div transition:fade class="{classes} pointer-events-auto w-full max-w-sm overflow-hidden rounded-lg bg-background shadow-lg ring-1 ring-black ring-opacity-5">
                <div class="p-4">
                    <div class="flex items-start">
                        <div class="flex-shrink-0 text-red-600">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z" />
                            </svg>
                        </div>
                        <div class="ml-3 w-0 flex-1 pt-0.5">
                            <p class="text-sm font-medium text-gray-900">{error}</p>
                            <p class="mt-1 text-sm text-gray-500">{error_submessage}</p>
                        </div>
                        <div class="ml-4 flex flex-shrink-0">
                            <button on:click={() => { error = undefined }} type="button" class="inline-flex rounded-md bg-background text-gray-400 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2">
                                <span class="sr-only">Close</span>
                                <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                    <path d="M6.28 5.22a.75.75 0 00-1.06 1.06L8.94 10l-3.72 3.72a.75.75 0 101.06 1.06L10 11.06l3.72 3.72a.75.75 0 101.06-1.06L11.06 10l3.72-3.72a.75.75 0 00-1.06-1.06L10 8.94 6.28 5.22z" />
                                </svg>
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        {/if}
    </div>
</div>

<div transition:fade class="flex h-full w-full">
    <Menu buckets={true} api_engine_class={api_engine} page_rules_class={page_rules} optimization_class={cache_engine} bot_engine_class={bot_engine} />

    <NewEditPopup bind:popup={new_bucket_popup} callback={newBucket} bind:edit_popup={edit_bucket_popup} edit_callback={editBucket} name="origin setting">
        <p>
            <label for="rule-path" class="block font-medium text-slate-100">Name</label>
            <a class="text-gray-300 text-sm tracking-tight">This is how you'll identify the bucket. It must be unique across this domain, and cannot contain spaces or special characters.</a>
            <input bind:value={new_bucket_name} type="text" name="setting-name" id="rule-path" class="mt-1 focus:ring-indigo-500 focus:border-indigo-500 block w-full shadow-sm sm:text-sm border-fuchsia-500 rounded-md" placeholder="hey-jude" required>
        </p>

        <p>
            <label for="rule-path" class="block font-medium text-slate-100 pt-6">Threshold</label>
            <a class="text-gray-300 text-sm tracking-tight">A 1-1000 value that represents the amount of requests a key (such as an IP address) before being blocked.</a>
            <input bind:value={new_bucket_threshold} type="number" min="1" max="1000" name="setting-name" id="rule-order" class="mt-1 focus:ring-indigo-500 focus:border-indigo-500 block w-full shadow-sm sm:text-sm border-fuchsia-500 rounded-md" placeholder="10 requests" required>
        </p>

        <p>
            <label for="rule-path" class="block font-medium text-slate-100 pt-6">Interval</label>
            <a class="text-gray-300 text-sm tracking-tight">A 1-86400 (one second to one day) value that represents the time limit the requests can be made in.</a>
            <input bind:value={new_bucket_secs} type="number" min="1" max="86400" name="setting-name" id="rule-order" class="mt-1 focus:ring-indigo-500 focus:border-indigo-500 block w-full shadow-sm sm:text-sm border-fuchsia-500 rounded-md" placeholder="10 seconds" required>
        </p>


        {#if new_bucket_name && new_bucket_threshold && new_bucket_secs}
            <div class="flex justify-center pt-8">
                <button transition:fade type="submit" class="mt-3 inline-flex w-full items-center justify-center rounded-md bg-blue-500 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-fuchsia-500 duration-100 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 sm:ml-3 sm:mt-0 sm:w-auto">Stage bucket with name {new_bucket_name}, that allows {new_bucket_threshold} requests every {new_bucket_secs} seconds.</button>
            </div>
        {:else}
            <div class="flex justify-center pt-8">
                <p class="mt-3 inline-flex w-full items-center justify-center rounded-md text-sm font-semibold text-white shadow-sm hover:bg-red-500 duration-100 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 sm:ml-3 sm:mt-0 sm:w-auto">All fields must be filled in to create this bucket.</p>
            </div>
        {/if}
        <p class="text-sm text-slate-200">
            <br><br>If you have any questions, our <a href="/support" class="text-indigo-500 hover:text-indigo-400">support team</a> stands by, ready to lend a hand if you have any questions. It is staffed by the same people who built Packetware.
            You can also check the <a href="/docs" class="text-indigo-500 hover:text-indigo-400">documentation</a> on buckets.
        </p>
    </NewEditPopup>



    {#if loaded}
        <div transition:fade class="w-full p-5 pb-28 backdrop-blur-lg rounded-b-2xl shadow">
            <!-- <div>
                <p class="uppercase text-gray-300">at a glance</p>
                <p class="text-slate-200 text-xl">API engine has handled 239,392 requests</p>
            </div> -->
            <div class="flex items-center justify-between">
                <p class="uppercase text-gray-300">Ratelimiting (bucket-based) settings - {domain_info._id}</p>
                <div class="flex flex-1 justify-end">
                    <button on:click={() => {if (able_to_save === true) show_staged_changes = true}} class="{saved_class} text-white px-3 py-1.5 mx-3 rounded text-sm">
                        {#if saving}
                            Saving...
                        {:else}
                            {#if able_to_save}
                                View staged changes
                            {:else}
                                No changes made yet
                            {/if}
                        {/if}
                    </button>
                    <a href="/docs/buckets" class="uppercase text-indigo-200 hover:text-indigo-100 py-1.5">documentation</a>
                </div>
            </div>
            <div>
                <div class="h-full">
                    <div class="flex items-center justify-between py-5">
                        <p class="uppercase text-gray-300">Buckets</p>
                        <div class="flex flex-1 justify-end">
                            <button on:click={() => { new_bucket_name = undefined; new_bucket_threshold = undefined; new_bucket_secs = undefined; new_bucket_popup = true }} class="bg-fuchsia-500 text-white px-3 py-1.5 mx-3 rounded text-sm">
                                + New bucket
                            </button>
                        </div>
                    </div>

                    <div class="pb-5">
                        {#each buckets as bucket, index}
                                <div class="pt-5">
                                    <div class="shadow bg-opacity-90 bg-amber-100">
                                        <div class="px-4 py-5 sm:p-6">
                                            <div class="sm:flex sm:flex-1 sm:w-full sm:justify-between">
                                                <div class="justify-start sm:flex sm:flex-1">
                                                    <h3 class="text-base font-semibold leading-6 text-gray-900 py-0.5 pr-3" id="api-strict-mode-enabled">{bucket._id}</h3>
                                                    {#if bucket.changed?.new_staged}
                                                        <span class="inline-flex flex-shrink-0 items-center rounded-full bg-blue-50 px-1.5 py-0.5 font-medium text-blue-700 ring-1 ring-inset ring-green-600/20">staged</span>
                                                    {:else if bucket.changed?.existed_modified}
                                                        <span class="inline-flex flex-shrink-0 items-center rounded-full bg-yellow-50 px-1.5 py-0.5 font-medium text-yellow-700 ring-1 ring-inset ring-yellow-600/20">modified</span>
                                                    {:else if bucket.changed?.being_deleted}
                                                        <span class="inline-flex flex-shrink-0 items-center rounded-full bg-red-50 px-1.5 py-0.5 font-medium text-red-700 ring-1 ring-inset ring-red-600/20">staged to be deleted</span>
                                                    {/if}
                                                </div>
                                                <div class="justify-end flex flex-1 items-center">
                                                    <div class="flex-col pr-5">
                                                        <p class="text-gray-400 uppercase text-sm">24h passed / rl</p>
                                                        <p class="text-gray-600">{bucket.passed} / {bucket.rl}</p>
                                                    </div>
                                                    <button on:click={() => { setupEdit(index) }} class="border-r-2 text-gray-600 pr-3 mr-3 rounded text-sm">
                                                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                                            <path stroke-linecap="round" stroke-linejoin="round" d="M16.862 4.487l1.687-1.688a1.875 1.875 0 112.652 2.652L6.832 19.82a4.5 4.5 0 01-1.897 1.13l-2.685.8.8-2.685a4.5 4.5 0 011.13-1.897L16.863 4.487zm0 0L19.5 7.125" />
                                                        </svg>
                                                    </button>
                                                    <button on:click={async () => deleteBucket(bucket._id)} class="text-red-500 hover:text-red-700 rounded text-sm">
                                                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                                            <path stroke-linecap="round" stroke-linejoin="round" d="M14.74 9l-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 01-2.244 2.077H8.084a2.25 2.25 0 01-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 00-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 013.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 00-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 00-7.5 0" />
                                                        </svg>
                                                    </button>
                                                </div>
                                            </div>
                                            <div class="mt-2 sm:flex sm:items-start sm:justify-between">
                                                <div class="max-w-xl text-sm text-gray-500">
                                                    <p>{bucket.threshold} requests every {bucket.secs} seconds</p>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                        {/each}
                    </div>
                </div>
            </div>
        </div>
        {/if}
</div>
