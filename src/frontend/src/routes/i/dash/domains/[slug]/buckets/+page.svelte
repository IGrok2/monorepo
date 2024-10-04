<!--
    mode: z.enum(["Chill", "Standard", "Standard+"]),
    turbo_mode_enabled: z.boolean(),
    traffic_level: z.enum(["Less", "Decent", "High", "Very high"]),
    /*notifications: z.array(z.object({
        webhook: z.string().max(150).pattern("^(http|https)://"),
        email: z.boolean()
    }))*/
-->

<script>
    import { page } from "$app/stores";
    import { onMount } from "svelte";
    import { fade, fly, slide, draw } from "svelte/transition";
    import Confirmation from "$lib/base/Confirmation.svelte";
    import OneOption from "$lib/OneOption.svelte";
    import OverridePopup from "$lib/base/OverridePopup.svelte";
    import { PUBLIC_API } from "$env/static/public";
    import Notifications from "$lib/components/notifications/Notifications.svelte";
    import { getCookie } from "$lib/utils/auth";

    import { Button } from "$lib/components/ui/button";
    import * as Drawer from "$lib/components/ui/drawer";
    import * as Card from "$lib/components/ui/card";
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
    import { Input } from "$lib/components/ui/input";
    import {ChevronDown, Pencil, Save, Trash2} from "lucide-svelte";
    import {toast} from "svelte-sonner";
    import Breadcrumbs from "$lib/components/Breadcrumbs.svelte";
    import {beautiful_date} from "$lib/utils/beautiful_date.js";
    import * as domain from "domain";

    /** @type {import('./$types').PageData} */
    export let data;

    // For loading the dashboard
    let domain_info = data.resp.domain.domain;
    let glance = data.resp.domain.glances.buckets;
    let buckets = [...domain_info.ratelimit_buckets]
    // $: buckets, letSaveChanges();
    let loaded = false;
    let time_based;
    let override;
    let override_popup;

    // Basic saving class properties
    let saving = false;
    let able_to_save = false;
    let confirmation_title;
    let confirmation_message;
    let confirmation_function;

    // human engine helpers
    let human_engine_mode;
    let human_engine_turbo = false;
    // these will be TODO, because I haven't established when to notify people because mitigation is mostly static
    let human_engine_traffic_level; // TODO
    let human_engine_notification;

    const load = async () => {
        try {
            override = false;

            const slug = $page.params.slug;

            const token = getCookie("jwt");

            await fetch(`${PUBLIC_API}/@/project/${$page.params.project}/domain/${$page.params.id}`, {
                method: "POST",
                headers: new Headers({
                    "content-type": "application/json",
                    Authorization: token,
                }),
                body: JSON.stringify({
                    domain: slug,
                }),
            }).then(async (rawBody) => {
                let body = await rawBody.json();

                if (rawBody.status === 200) {
                    domain_info = body.domain;

                    const foo_he = domain_info.human_engine;

                    foo_he.changed = { ssl: false };

                    human_engine_mode = foo_he.mode;
                    human_engine_turbo = foo_he.turbo_mode_enabled;
                    human_engine_traffic_level = foo_he.traffic_level;

                    // verifier for the load function making sure we've got the correct data
                    if (human_engine_mode === undefined || human_engine_turbo === undefined || human_engine_traffic_level === undefined) {
                        toast.error(`Failed to load human engine settings for ${slug} - team has been notified`);
                        throw new Error(`Failed to load human engine settings: ${slug} - ${JSON.stringify(foo_he)}}`);
                    } else {
                        // get the time we got the changes at so we can track any changes
                        time_based = Date.now();

                        loaded = true;
                    }
                } else {
                    document.location.href = `/i/dash/domains/${$page.params.slug}`;
                }
            });
        } catch (err) {
            toast.error(`Redirecting for authentication - failed to load human engine settings for ${$page.params.slug}`);

            // redirect to login after 3 seconds
            await new Promise(() => { setTimeout(() => {
                document.location.href = "/i/auth/login";
            }, 3000) });
        }
    };

    onMount(async () => {
        const foo_he = domain_info.human_engine;

        foo_he.changed = { ssl: false };

        human_engine_mode = foo_he.mode;
        human_engine_turbo = foo_he.turbo_mode_enabled;
        // human_engine_traffic_level = foo_he.traffic_level;

        // verifier for the load function making sure we've got the correct data
        if (human_engine_mode === undefined || human_engine_turbo === undefined) {
            toast.error(`Failed to load human engine settings for ${domain_info._id} - team has been notified`);
            throw new Error(`Failed to load human engine settings: ${domain_info._id} - ${JSON.stringify(foo_he)}}`);
        } else {
            // get the time we got the changes at so we can track any changes
            time_based = Date.now();

            loaded = true;
        }
    });

    const letSaveChanges = () => {
        if (domain_info) {
            if (domain_info.ratelimit_buckets.length !== buckets.length) {
                able_to_save = true;
            }

            for (let i = 0; i < buckets.length; i++) {
                // iterate through the buckets and see if the fields match up
                let og = domain_info.ratelimit_buckets[i];
                let nw = buckets[i];

                if (og.threshold !== nw.threshold || og.secs !== nw.secs) {
                    able_to_save = true;
                }
            }
        }
    };

    const saveChanges = async () => {
        // save changes to bot settings
        saving = true;
        save_button_success = true;

        // modify the buckets
        for (let i = 0; i < buckets.length; i++) {
            delete buckets[i].editing;
        }

        try {
            const slug = $page.params.slug;

            const token = getCookie("jwt");

            const request = await fetch(
                `${PUBLIC_API}/@/project/${$page.params.project}/domain/${$page.params.id}/buckets/update`,
                {
                    method: "POST",
                    headers: new Headers({
                        "content-type": "application/json",
                        Authorization: token,
                    }),
                    body: JSON.stringify({
                        domain: slug,
                        buckets,
                        time: time_based,
                        __foo_confirm: true,
                    }),
                },
            );

            const response = await request.json();

            if (request.status !== 200) {
                if (
                    response.message ===
                    "Changes have been made since data last checked. Confirm changes? Open new changes in a new tab?"
                ) {
                    override_popup = true;
                } else {
                    toast.error(`Response: "${response.message}" - please reach out if in error`);
                    throw new Error(response.message);
                }
            } else {
                saving = false;
                show_staged_changes = false;
                save_button_success = false;
                await load();
                letSaveChanges();
                await toast.success("Pushed settings to edge");
            }
        } catch (err) {
            saving = false;
            show_staged_changes = false;
            save_button_success = false;
            await toast.error(`Failed to push settings to edge: ${err}`);
        }
    };

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
    };

    $: human_engine_mode, letSaveChanges();
</script>

<svelte:window on:beforeunload={beforeUnload} />

<Confirmation
        bind:confirmation_title
        bind:confirmation_message
        bind:confirmation_function
        letSaveChangesFunc={letSaveChanges}
/>

<OverridePopup
        bind:override_popup
        bind:show_staged_changes
        bind:domain_info
        bind:override
        {saveChanges}
        bind:save_button_success
/>

{#if loaded}
    <div
            class="w-full p-5 rounded-b-2xl shadow"
    >
    <!-- <div>
                <p class="uppercase text-gray-300">at a glance</p>
                <p class="text-slate-200 text-xl">API engine has handled 239,392 requests</p>
            </div> -->
            
    <div class="flex items-center justify-between">
        <p class="uppercase text-muted-foreground">Ratelimiting Buckets</p>
        <div class="flex flex-wrap flex-1 justify-end space-x-4">
            {#if saving}
                <Button variant="outline" disabled>
                    Saving...
                </Button>
            {:else}
                <Drawer.Root>
                    <Drawer.Trigger asChild let:builder>
                        <Button builders={[builder]}>Push config to edge</Button>
                    </Drawer.Trigger>
                    <Drawer.Content>
                        <div class="mx-auto w-full max-w-sm">
                            <Drawer.Header>
                                <Drawer.Title>Review & confirm</Drawer.Title>
                                <Drawer.Description>
                                    <ol class="list-disc">
                                        Push the current configuration to the edge.
                                    </ol>
                                </Drawer.Description>
                            </Drawer.Header>
                            <Drawer.Footer>
                                <Drawer.Close asChild let:builder>
                                    <Button on:click={saveChanges} builders={[builder]}>Ship it</Button>
                                </Drawer.Close>
                                <Drawer.Close asChild let:builder>
                                    <Button builders={[builder]} variant="outline">Cancel</Button>
                                </Drawer.Close>
                            </Drawer.Footer>
                        </div>
                    </Drawer.Content>
                </Drawer.Root>
            {/if}
            <Button href="/docs/buckets" variant="secondary"
                    class="my-1.5 md:my-0"
            >Documentation</Button
            >
        </div>
    </div>
    <div>
        <div class="h-full flex flex-wrap justify-end">
            <div class="md:flex items-center justify-between py-5">
                <p class="">
                    The core protection principle of Packetware involves splitting
                    traffic into two categories: what should be human and what should be allowed to be bot.
                    Human Engine is the system that is designed to be applied to intended human traffic.
                </p>
            </div>

            <Button
                    class="my-3"
                    on:click={() => buckets = [{
                        _id: `${domain_info._id + "_bucket_" + (buckets.length + 1)}`,
                        threshold: 100,
                        secs: 10,
                        editing: true,
            }, ...buckets]}
                    variant="primary"
            >
                Create new bucket
            </Button>
        </div>

        <div class="md:grid md:grid-cols-2 space-x-6">
            {#each buckets as bucket}
                {#if bucket.editing}
                    <Card.Root>
                        <Card.Header>
                            <div
                                    class="flex justify-between items-center w-full"
                            >
                                <div>
                                    <Card.Title>{bucket._id}</Card.Title>
                                    <Card.Description class="w-full">
                                        <div class="flex w-full justify-between">
                                            <div class="">
                                                <a class="md:flex items-center md:align-middle underline decoration-dashed text-sm text-primary lowercase" href="./" target="_blank">
                                                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-4 h-4">
                                                        <path stroke-linecap="round" stroke-linejoin="round" d="M7.5 14.25v2.25m3-4.5v4.5m3-6.75v6.75m3-9v9M6 20.25h12A2.25 2.25 0 0 0 20.25 18V6A2.25 2.25 0 0 0 18 3.75H6A2.25 2.25 0 0 0 3.75 6v12A2.25 2.25 0 0 0 6 20.25Z" />
                                                    </svg>

                                                    <p>
                                                    Last 24 hours -
                                                    bucket allowed {glance.find(b => b.name === bucket._id) ? glance.find(b => b.name === bucket._id).bucket_passed : 0} and ratelimited {glance.find(b => b.name === bucket._id) ? glance.find(b => b.name === bucket._id).bucket_ratelimited : 0}
                                                    </p>
                                                </a>

                                                <p class="pt-5">
                                                    Allows <Input bind:value={bucket.threshold}/> requests per <Input bind:value={bucket.secs}/> seconds ({beautiful_date(bucket.secs)})
                                                </p>
                                            </div>

                                            <div class="pl-3 flex justify-end">
                                                <Button
                                                        variant="ghost"
                                                        on:click={() => {
                                                                    // set match_type to editing
                                                                    bucket.editing = !bucket.editing;
                                                            }}
                                                >
                                                    {#if bucket.editing}
                                                        <Save size=16 strokeWidth=2 />
                                                    {:else}
                                                        <Pencil size=16 strokeWidth=2 />
                                                    {/if}
                                                </Button>
                                                <Button
                                                        variant="ghost"
                                                        size=12
                                                        strokeWidth=1
                                                        on:click={async () => {
                                                            domain_info.ratelimit_buckets = domain_info.ratelimit_buckets.filter(b => b._id !== bucket._id);
                                                                    }}
                                                >
                                                    <Trash2 size=16 strokeWidth=2 color="red" />
                                                </Button>
                                            </div>
                                        </div>
                                    </Card.Description>
                                </div>
                            </div>
                        </Card.Header>
                    </Card.Root>
                {:else}
                    <Card.Root>
                        <Card.Header>
                            <div
                                    class="flex justify-between items-center space-x-8 w-full"
                            >
                                <div>
                                    <Card.Title>{bucket._id}</Card.Title>
                                    <Card.Description class="w-full">
                                        <div class="flex w-full justify-between">
                                            <div class="">
                                                <a class="flex items-center align-middle underline decoration-dashed text-sm text-primary lowercase" href="./" target="_blank">
                                                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-4 h-4">
                                                        <path stroke-linecap="round" stroke-linejoin="round" d="M7.5 14.25v2.25m3-4.5v4.5m3-6.75v6.75m3-9v9M6 20.25h12A2.25 2.25 0 0 0 20.25 18V6A2.25 2.25 0 0 0 18 3.75H6A2.25 2.25 0 0 0 3.75 6v12A2.25 2.25 0 0 0 6 20.25Z" />
                                                    </svg>

                                                    Last 24 hours -
                                                    bucket allowed {glance.find(b => b.name === bucket._id) ? glance.find(b => b.name === bucket._id).bucket_passed : 0} and ratelimited {glance.find(b => b.name === bucket._id) ? glance.find(b => b.name === bucket._id).bucket_ratelimited : 0}
                                                </a>

                                                <p class="pt-5">
                                                    Allows {bucket.threshold} requests per {beautiful_date(bucket.secs)}
                                                </p>
                                            </div>

                                            <div class="pl-3 flex justify-end">
                                                <Button
                                                        variant="ghost"
                                                        on:click={() => {
                                                                    // set match_type to editing
                                                                    bucket.editing = !bucket.editing;
                                                            }}
                                                >
                                                    {#if bucket.editing}
                                                        <Save size=16 strokeWidth=2 />
                                                    {:else}
                                                        <Pencil size=16 strokeWidth=2 />
                                                    {/if}
                                                </Button>
                                                <Button
                                                        variant="ghost"
                                                        size=12
                                                        strokeWidth=1
                                                        on:click={async () => {
                                                            domain_info.ratelimit_buckets = domain_info.ratelimit_buckets.filter(b => b._id !== bucket._id);
                                                                    }}
                                                >
                                                    <Trash2 size=16 strokeWidth=2 color="red" />
                                                </Button>
                                            </div>
                                        </div>
                                    </Card.Description>
                                </div>
                            </div>
                        </Card.Header>
                    </Card.Root>
                {/if}
            {/each}
        </div>
    </div>
    </div>
{/if}
