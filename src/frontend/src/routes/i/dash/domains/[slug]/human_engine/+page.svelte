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
    import { ChevronDown } from "lucide-svelte";
    import {toast} from "svelte-sonner";
    import Breadcrumbs from "$lib/components/Breadcrumbs.svelte";

    /** @type {import('./$types').PageData} */
    export let data;

    // For loading the dashboard
    let domain_info = data.resp.domain.domain;
    let glance = data.resp.domain.glances.human;
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
            able_to_save =
                human_engine_mode !== domain_info.human_engine.mode;
        }
    };

    const saveChanges = async () => {
        // save changes to bot settings
        if (able_to_save === true) {
            saving = true;
            save_button_success = true;

            try {
                const slug = $page.params.slug;

                const token = getCookie("jwt");

                const request = await fetch(
                    `${PUBLIC_API}/@/project/${$page.params.project}/domain/${$page.params.id}/human-engine/update`,
                    {
                        method: "POST",
                        headers: new Headers({
                            "content-type": "application/json",
                            Authorization: token,
                        }),
                        body: JSON.stringify({
                            domain: slug,
                            human_engine: {
                                mode: human_engine_mode,
                            },
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
        <p class="uppercase text-muted-foreground">Human Engine</p>
    <div class="md:flex md:items-center md:justify-between">
        <div class="uppercase text-muted-foreground py-4">
            {#if glance?.bot_ratio !== null}
                <a class="md:flex items-center align-middle underline decoration-dashed text-sm text-primary lowercase" href="./" target="_blank">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-4 h-4">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M7.5 14.25v2.25m3-4.5v4.5m3-6.75v6.75m3-9v9M6 20.25h12A2.25 2.25 0 0 0 20.25 18V6A2.25 2.25 0 0 0 18 3.75H6A2.25 2.25 0 0 0 3.75 6v12A2.25 2.25 0 0 0 6 20.25Z" />
                    </svg>

                    Last 24 hours -
                    for every human request there were {glance.bot_ratio} bot requests
                </a>
            {/if}
        </div>
        <div class="flex flex-wrap flex-1 justify-end space-x-4">
            {#if able_to_save && !saving}
                <Drawer.Root>
                    <Drawer.Trigger asChild let:builder>
                        <Button builders={[builder]}>Review staged changes</Button>
                    </Drawer.Trigger>
                    <Drawer.Content>
                        <div class="mx-auto w-full max-w-sm">
                            <Drawer.Header>
                                <Drawer.Title>Review & confirm</Drawer.Title>
                                <Drawer.Description>
                                    <ol class="list-disc">
                                        {#if human_engine_mode !== domain_info.human_engine.mode}
                                            <li>
                                                <a class="text-yellow-500">Change</a
                                                >
                                                human engine mode: {human_engine_mode}
                                            </li>
                                        {/if}
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
            {:else}
            <Button
                on:click={() => {
                    if (able_to_save === true) show_staged_changes = true;
                }}
                variant="outline"
            >
                {#if saving}
                    Saving...
                {:else}
                    No changes made yet
                {/if}
            </Button>
            {/if}
            <Button href="/docs/buckets" class="my-1.5 md:my-0" variant="secondary"
                >Documentation</Button
            >
        </div>
    </div>
    </div>
    <div>
        <div class="h-full">
            <div class="md:flex items-center justify-between py-5">
                <p class="">
                    Human Engine is an advanced system that deters bad bots from causing issues to your site,
                    such as denial of service attacks and comment spam. Using a modern and traditional blend of heuristics,
                    machine learning, and static techniques. Its work is invisibly in the background so your users are never
                    impacted with bad UX. Additionally, it can be combined with API Engine to validate human traffic to specific endpoints,
                    like an authentication API.
                    <br><br>
                    The core protection principle of Packetware involves splitting
                    traffic into two categories: what should be human and what should be allowed to be bot.
                    Human Engine is the system that is designed to be applied to intended human traffic.
                </p>
            </div>
        </div>

        <div class="my-4 space-y-4">
            <Card.Root>
                <Card.Header>
                    <div
                        class="md:flex justify-between items-center md:space-x-8"
                    >
                        <div>
                            <Card.Title>Human Engine setting</Card.Title>
                            <Card.Description
                                >Use this to customize the strictness of the
                                human engine. Use Chill for high flexibility
                                and tolerance for more leakage. Switch to Standard
                                for a more balanced approach, when bots shouldn't be visiting your site.
                                When the most sophisticated attacks face you, or your site has little tolerance for excess traffic,
                                Standard+ will pull all the stops.

                                <a class="md:flex items-center align-middle underline decoration-dashed text-sm text-primary lowercase" href="./" target="_blank">
                                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-4 h-4">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M7.5 14.25v2.25m3-4.5v4.5m3-6.75v6.75m3-9v9M6 20.25h12A2.25 2.25 0 0 0 20.25 18V6A2.25 2.25 0 0 0 18 3.75H6A2.25 2.25 0 0 0 3.75 6v12A2.25 2.25 0 0 0 6 20.25Z" />
                                    </svg>

                                    Last 24 hours -
                                    human engine served {glance?.smart_challenged_reqs} evaluations, {glance?.challenge_completed} were completed
                                </a>

                            </Card.Description
                            >
                        </div>
                        <div class="pt-5">
                            <DropdownMenu.Root>
                                <DropdownMenu.Trigger asChild let:builder>
                                  <Button variant="outline" builders={[builder]}>{human_engine_mode} <ChevronDown /></Button>
                                </DropdownMenu.Trigger>
                                <DropdownMenu.Content class="w-56">
                                  <DropdownMenu.Label>Human Engine Strictness</DropdownMenu.Label>
                                  <DropdownMenu.Separator />
                                  <DropdownMenu.RadioGroup bind:value={human_engine_mode}>
                                    <DropdownMenu.RadioItem value="Chill">Chill</DropdownMenu.RadioItem>
                                    <DropdownMenu.RadioItem value="Standard">Standard</DropdownMenu.RadioItem>
                                    <DropdownMenu.RadioItem value="Standard+">Standard+</DropdownMenu.RadioItem>
                                  </DropdownMenu.RadioGroup>
                                </DropdownMenu.Content>
                            </DropdownMenu.Root>
                        </div>
                    </div>
                </Card.Header>
            </Card.Root>
        </div>
    </div>
    </div>
{/if}
