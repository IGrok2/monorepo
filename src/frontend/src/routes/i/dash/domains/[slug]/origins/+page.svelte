<script>
    import { page } from "$app/stores";

    import { enhance } from "$app/forms";

    import { Button } from "$lib/components/ui/button";
    import * as Card from "$lib/components/ui/card";
    import DisplayOriginSetting from "./(components)/DisplayOriginSetting.svelte";
    import { Lock, Plus, Pencil, Trash2 } from "lucide-svelte";
    import { toast } from "svelte-sonner";
    import Dialog from "$lib/components/Dialog.svelte";
    import EditOrigin from "./(components)/EditOrigin.svelte";
    /*import {
        newOrigin,
        editOrigin,
        deleteOrigin,
    } from "$lib/utils/origins";*/
    import Breadcrumbs from "$lib/components/Breadcrumbs.svelte";
    import { fade } from "svelte/transition";
    import APIClient from "$lib/utils/api";
    import deepClone from "$lib/utils/deep_clone";

    /** @type {import('./$types').PageData} */
    export let data;

    const domain = data.resp.domain.domain;

    const glance = data.resp.domain.glances.origins;

    let origin_settings = domain.origin_settings;

    let newOriginDialog = false;

    let editOriginId = "";

    const default_origin = {
        _id: "",
        ssl: false,
        http2: true,
        timeout: 10,
        ip_data: false,
        origins: [
            {
                url: "",
                weight: 1,
            },
        ],
        origin_failover: false,
    };

    let editOrigin = default_origin;
    let newOrigin = default_origin;

    const newOriginAction = async () => {
        toast.loading("Processing new origin request...");

        let domain_info = deepClone(domain);
        console.log("domain_info: ", domain_info);
        // Remove _id field from each origin in domain_info.origin_settings.origins.
        for (let i = 0; i < domain_info.origin_settings.length; i++) {
            domain_info.origin_settings[i].http2 = false;
            for (
                let j = 0;
                j < domain_info.origin_settings[i].origins.length;
                j++
            ) {
                delete domain_info.origin_settings[i].origins[j]._id;
            }
        }

        // Combine all to update.
        if (domain_info.origin_settings) {
            domain_info.origin_settings.push(newOrigin);
        } else {
            domain_info.origin_settings = [newOrigin];
        }

        const reqBody = {
            domain: domain_info._id,
            origin_settings: domain_info.origin_settings,
            time: Date.now(),
            __foo_confirm: true,
        };

        console.log(reqBody);

        try {
            const res = await APIClient.post(
                `@/project/${$page.params.project}/domain/${$page.params.slug}/origins/update`,
                reqBody,
            );

            toast.success(res.message);
        } catch (error) {
            console.log("Error status", error.response.status);
            console.log("Response text", error.response.data);
            toast.error(error.response.data.message);
        }
    };

    // Generic function always used to manipulate origins.
    const updateOriginAction = async () => {
        toast.loading("Processing edit origin request...");

        let domain_info = deepClone(domain);
        // Replaced the old origin with the edited data.
        for (let i = 0; i < domain_info.origin_settings.length; i++) {
            if (domain_info.origin_settings[i]._id === editOrigin._id) {
                domain_info.origin_settings[i] = editOrigin;
            }
            domain_info.origin_settings[i].http2 = false;
        }

        // Remove _id field from each origin in domain_info.origin_settings.origins.
        for (let i = 0; i < domain_info.origin_settings.length; i++) {
            for (
                let j = 0;
                j < domain_info.origin_settings[i].origins.length;
                j++
            ) {
                delete domain_info.origin_settings[i].origins[j]._id;
            }
        }

        const reqBody = {
            domain: domain_info._id,
            origin_settings: domain_info.origin_settings,
            time: Date.now(),
            __foo_confirm: true,
        };

        console.log(reqBody);

        try {
            const res = await APIClient.post(
                `@/project/${$page.params.project}/domain/${$page.params.slug}/origins/update`,
                reqBody,
            );

            toast.success(res.message);
        } catch (error) {
            console.log("Error status", error.response.status);
            console.log("Response text", error.response.data);
            toast.error(error.response.data.message);
        }
    };

    // Generic function always used to manipulate origins.
    const deleteOriginAction = async () => {
        toast.loading("Processing delete origin request...");

        let domain_info = deepClone(domain);
        // Remove _id field from each origin in domain_info.origin_settings.origins.
        for (let i = 0; i < domain_info.origin_settings.length; i++) {
            for (
                let j = 0;
                j < domain_info.origin_settings[i].origins.length;
                j++
            ) {
                delete domain_info.origin_settings[i].origins[j]._id;
            }
        }

        // Delete the origin to be deleted.
        for (let i = 0; i < domain_info.origin_settings.length; i++) {
            if (domain_info.origin_settings[i]._id === deletedOrigin._id) {
                domain_info.origin_settings.splice(i, 1);
            }
        }

        const reqBody = {
            domain: domain_info._id,
            origin_settings: domain_info.origin_settings,
            time: Date.now(),
            __foo_confirm: true,
        };

        console.log(reqBody);

        try {
            const res = await APIClient.post(
                `@/project/${$page.params.project}/domain/${$page.params.slug}/origins/update`,
                reqBody,
            );

            toast.success(res.message);
        } catch (error) {
            console.log("Error status", error.response.status);
            console.log("Response text", error.response.data);
            toast.error(error.response.data.message);
        }
    };
    /*
    const newOriginAction = async () => {
        toast.loading("Processing new origin request...");

        let action;
        try {
            action = await newOrigin(origin, domain);
            console.log("action", action);
            origin_settings = action.origin_settings;
            newOriginDialog = false;

            toast.success(action.message);
        } catch (err) {
            console.log("action", action);
            toast.error(action.message);
        }
    };

    const editOriginAction = async () => {
        toast.loading("Processing edit origin request...");

        let action;
        try {
            action = await editOrigin(origin, domain);
            console.log("action", action);
            origin_settings = action.origin_settings;
            editOriginId = "";

            toast.success(action.message);
        } catch (err) {
            toast.error(action.message);
        }
    };

    const modifyOriginAction = async () => {
        toast.loading("Processing edit origin request...");

        let domain_info = Object.assign({}, domain);
        // Replaced the old origin with the edited data.
        for (let i = 0; i < domain_info.origin_settings.length; i++) {
            if (domain_info.origin_settings[i]._id === origin._id) {
                domain_info.origin_settings[i] = origin;
            }
            domain_info.origin_settings[i].http2 = false;
        }

        // Remove _id field from each origin in domain_info.origin_settings.origins.
        for (let i = 0; i < domain_info.origin_settings.length; i++) {
            for (
                let j = 0;
                j < domain_info.origin_settings[i].origins.length;
                j++
            ) {
                delete domain_info.origin_settings[i].origins[j]._id;
            }
        }

        let res = await updateOrigin(domain_info);
        console.log(res);

        if (res.success) {
            toast.success(res.message);
        } else {
            toast.error(res.message);
        }
    };

    const deleteOriginAction = async () => {
        toast.loading("Processing delete origin request...");
        let action;
        try {
            action = await deleteOrigin(origin, domain);
            console.log("action", action);
            origin_settings = action.origin_settings;
            editOriginId = "";

            toast.success(action.message);
        } catch (err) {
            console.log(err);
            toast.error(action.message);
        }
    };*/

    console.log(domain);
</script>

<!--
<Dialog
    bind:dialog
    on:close
>
    <NewOrigin domain_info={domain} />
</Dialog>
-->

<div class="w-full p-5 rounded-b-2xl shadow">
    <div class="flex items-center justify-between mx-2">
        <p class="uppercase md:text-base text-muted-foreground py-4">Origins</p>
        <div class="flex flex-wrap flex-1 justify-end space-x-4">
            <Button
                on:click={() => {
                    newOriginDialog = !newOriginDialog;
                    newOrigin = default_origin; // reset just in case prior was edited.
                }}
            >
                {#if newOriginDialog}
                    Close
                {:else}
                    <Plus class="mr-2" />
                    New Origin Setting
                {/if}
            </Button>
            <Button
                class="my-1.5 md:my-0"
                href="/docs/domains/what-are-origins"
                variant="secondary">Documentation</Button
            >
        </div>
    </div>
    <div>
        <div class="h-full mx-2">
            <div class="flex items-center justify-between py-5">
                <p class="pr-2">
                    Origins are the servers that serve your content. You can add
                    multiple origins to your domain to serve your content from
                    different locations. You can also set up failover origins to
                    ensure your content is always available.
                </p>
            </div>
            <div class="pb-4 space-y-4">
                <form on:submit|preventDefault={newOriginAction}>
                    <EditOrigin
                        bind:show={newOriginDialog}
                        domain_info={domain}
                        bind:origin={newOrigin}
                    />
                </form>

                {#each origin_settings as origin_setting, index}
                    {#if editOriginId === origin_setting._id}
                        <form on:submit|preventDefault={updateOriginAction}>
                            <EditOrigin
                                bind:show={editOriginId}
                                domain_info={domain}
                                bind:origin={editOrigin}
                            />
                        </form>
                    {:else}
                        <Card.Root>
                            <Card.Header>
                                <div class="flex justify-between">
                                    <div>
                                        <Card.Title
                                            >{origin_setting._id}</Card.Title
                                        >
                                        <Card.Description>
                                            <a
                                                class="flex items-center align-middle underline decoration-dashed"
                                                href="./"
                                                target="_blank"
                                            >
                                                <svg
                                                    xmlns="http://www.w3.org/2000/svg"
                                                    fill="none"
                                                    viewBox="0 0 24 24"
                                                    stroke-width="1.5"
                                                    stroke="currentColor"
                                                    class="w-4 h-4"
                                                >
                                                    <path
                                                        stroke-linecap="round"
                                                        stroke-linejoin="round"
                                                        d="M7.5 14.25v2.25m3-4.5v4.5m3-6.75v6.75m3-9v9M6 20.25h12A2.25 2.25 0 0 0 20.25 18V6A2.25 2.25 0 0 0 18 3.75H6A2.25 2.25 0 0 0 3.75 6v12A2.25 2.25 0 0 0 6 20.25Z"
                                                    />
                                                </svg>
                                                ok:
                                                {glance?.find(
                                                    (editOrigin) =>
                                                        editOrigin.name ===
                                                        origin_setting._id,
                                                )?.origin_success}, errors:
                                                {glance?.find(
                                                    (editOrigin) =>
                                                        editOrigin.name ===
                                                        origin_setting._id,
                                                )?.origin_failures}
                                            </a>
                                        </Card.Description>
                                    </div>
                                    <div class="flex items-center">
                                        <Button
                                            variant="ghost"
                                            on:click={() => {
                                                editOriginId =
                                                    origin_setting._id;
                                                editOrigin = origin_setting;
                                            }}
                                        >
                                            <Pencil />
                                        </Button>
                                        <Button
                                            variant="ghost"
                                            on:click={async () => {
                                                editOrigin = origin_setting;
                                                await deleteOriginAction();
                                                editOrigin = null;
                                            }}
                                        >
                                            <Trash2 color="red" />
                                        </Button>
                                    </div>
                                </div>
                            </Card.Header>
                            <Card.Content>
                                <DisplayOriginSetting
                                    setting={origin_setting}
                                />
                            </Card.Content>
                        </Card.Root>
                    {/if}
                {/each}
            </div>
        </div>
    </div>
</div>
