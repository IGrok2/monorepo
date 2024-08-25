<script>
    import { page } from "$app/stores";
    import { onMount } from "svelte";
    import { toast } from "svelte-sonner";
    import { fade, fly, slide, draw } from "svelte/transition";
    import { Button } from "$lib/components/ui/button";
    import * as Card from "$lib/components/ui/card";
    import { Switch } from "$lib/components/ui/switch";
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
    import { Input } from "$lib/components/ui/input";
    import { Label } from "$lib/components/ui/label";
    import ClearCache from "./(components)/ClearCache.svelte";
    import { getCookie } from "$lib/utils/auth";
    import { ChevronDown } from "lucide-svelte";
    import * as Drawer from "$lib/components/ui/drawer";

    import { PUBLIC_API } from "$env/static/public";
    import Breadcrumbs from "$lib/components/Breadcrumbs.svelte";
    import {beautiful_date} from "$lib/utils/beautiful_date.js";
    import {beautifulBytes} from "$lib/utils/beautiful_bytes.js";

    /** @type {import('./$types').PageData} */
    export let data;

    // For loading the dashboard
    let domain_info = data.resp.domain.domain;
    let glance = data.resp.domain.glances.cache;
    let loaded = false;
    let time_based;
    let override = false;
    let override_popup = false;
    let cacheDialog;

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

    // optimization specific options
    let cache_enabled_switch;
    let default_ttl;
    let default_cache_level;

    // for default cache level changes

    $: default_cache_level, letSaveChanges();
    $: cache_enabled_switch, letSaveChanges();
    $: default_ttl, letSaveChanges();

    $: console.log(`default_cache_level: ${default_cache_level}`);

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

                domain_info = body.domain;

                const cache_settings = { ...domain_info.caching_settings };

                default_ttl = Number(cache_settings.default_ttl);
                default_cache_level = cache_settings.default_cache_level;
                cache_enabled_switch = cache_settings.enabled;

                console.log("test")

                time_based = Date.now();

                loaded = true;
            });
        } catch (err) {
            console.log(err);

            toast.error("Error loading user profile");

            document.location.href = "/i/auth/login";
        }
    };

    onMount(async () => {
        const cache_settings = { ...domain_info.caching_settings };

        default_ttl = Number(cache_settings.default_ttl);
        default_cache_level = cache_settings.default_cache_level;
        cache_enabled_switch = cache_settings.enabled;

        time_based = Date.now();

        loaded = true;
    });

    const saveChanges = async () => {
        // save changes to bot settings
        if (able_to_save === true) {
            let success = false;
            saving = true;
            save_button_success = true;

            try {
                const slug = $page.params.slug;

                const token = getCookie("jwt");

                console.log(`cache enabled switch: ${cache_enabled_switch}`);

                (async () => {
                    await fetch(`${PUBLIC_API}/@/project/${$page.params.project}/domain/${$page.params.id}/caching/update`, {
                        method: "POST",
                        headers: new Headers({
                            "content-type": "application/json",
                            Authorization: token,
                        }),
                        body: JSON.stringify({
                            domain: slug,
                            caching_settings: {
                                enabled: cache_enabled_switch,
                                default_ttl: Number(default_ttl),
                                default_cache_level: default_cache_level,
                            },
                            time: time_based,
                            __foo_confirm: true,
                        }),
                    })
                        .then(async (res) => {
                            let response = await res.json();
                            if (res.status !== 200) {
                                if (
                                    response.message ===
                                    "Changes have been made since data last checked. Confirm changes? Open new changes in a new tab?"
                                ) {
                                    override_popup = true;
                                } else {
                                    saving = false;
                                    show_staged_changes = false;
                                    save_button_success = false;
                                    toast.error(`Failed to update: ${response.message}`);
                                }
                            } else {
                                success = true;
                            }
                        })
                        .catch(async (err) => {
                            saving = false;
                            show_staged_changes = false;
                            save_button_success = false;
                            toast.error(`Failed to update: ${err}`);
                        });
                })().then(async () => {
                    if (success) {
                        saving = false;
                        show_staged_changes = false;
                        save_button_success = false;
                        await load();
                        letSaveChanges();
                        toast.success(`Pushed changes to edge`);
                    }
                });
            } catch (err) {
                saving = false;
                show_staged_changes = false;
                save_button_success = false;
                toast.error(`Failed to update: ${err}`);
            }
        }
    };

    const letSaveChanges = () => {
        if (domain_info) {
            // modifies able_to_save based on params
            console.log(`letSaveChanges called, cache_enabled_switch: ${cache_enabled_switch} vs ${domain_info.caching_settings.enabled}`);
            able_to_save =
                domain_info.caching_settings.enabled !== cache_enabled_switch ||
                domain_info.caching_settings.default_ttl !== default_ttl ||
                domain_info.caching_settings.default_cache_level !==
                    default_cache_level;

            able_to_save
                ? (saved_class = "bg-fuchsia-500")
                : (saved_class = "cursor-not-allowed bg-gray-400");
        }
    };

    const confirmChange = async (title, message, func) => {
        confirmation_title = title;
        confirmation_message = message;
        confirmation_function = func;
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
</script>

<svelte:window on:beforeunload={beforeUnload} />

<ClearCache bind:dialog={cacheDialog} />

<div class="flex h-full w-full">
    <!-- <Menu optimization={true} api_engine_class={api_engine} page_rules_class={page_rules} optimization_class={cache_engine} bot_engine_class={bot_engine} /> -->

    {#if loaded}
        <div
                class="w-full p-5 rounded-b-2xl shadow"
        >
            <!-- <div>
                <p class="uppercase text-gray-300">at a glance</p>
                <p class="text-slate-200 text-xl">API engine has handled 239,392 requests</p>
            </div> -->
            <div class="md:flex items-center justify-between">
                <div class="uppercase text-muted-foreground px-4 py-4">
                    Cache & optimize
                    {#if glance.cached_reqs}
                        <a class="md:flex items-center align-middle underline decoration-dashed text-sm text-primary normal-case" href="./" target="_blank">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-4 h-4">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M7.5 14.25v2.25m3-4.5v4.5m3-6.75v6.75m3-9v9M6 20.25h12A2.25 2.25 0 0 0 20.25 18V6A2.25 2.25 0 0 0 18 3.75H6A2.25 2.25 0 0 0 3.75 6v12A2.25 2.25 0 0 0 6 20.25Z" />
                            </svg>

                            last 24 hours -
                            {glance.cached_reqs} cached reqs, {beautifulBytes(glance.data_transferred_cache)} transferred
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
                                                {#if cache_enabled_switch !== domain_info.caching_settings.enabled}
                                                    {#if cache_enabled_switch === true}
                                                        <li>
                                                            <a class="text-green-500"
                                                            >Enable</a
                                                            > cache & optimize stack
                                                        </li>
                                                    {:else}
                                                        <li>
                                                            <a class="text-red-500"
                                                            >Disable</a
                                                            > cache & optimize stack
                                                        </li>
                                                    {/if}
                                                {/if}

                                                {#if default_ttl !== domain_info.caching_settings.default_ttl}
                                                    <li>
                                                        Change default time-to-live of
                                                        cached objects from {domain_info
                                                        .caching_settings.default_ttl} seconds
                                                        to
                                                        <a class="text-green-500"
                                                        >{default_ttl} seconds</a
                                                        >
                                                    </li>
                                                {/if}

                                                {#if default_cache_level !== domain_info.caching_settings.default_cache_level}
                                                    <li>
                                                        Change default cache level of cached
                                                        objects from {domain_info
                                                        .caching_settings
                                                        .default_cache_level} to
                                                        <a class="text-green-500"
                                                        >{default_cache_level}</a
                                                        >
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
                    <Button class="my-1.5 md:my-0" href="/docs/optimization" variant="secondary"
                        >Documentation</Button
                    >
                    <Button
                            class="my-1.5 md:my-0"
                            on:click={() => cacheDialog.showModal()}
                            variant="destructive"
                    >
                        Clear cache
                    </Button>
                </div>
            </div>
            <div class="mt-4 flex justify-end">
                <p class="p-4">
                    We optimize your site by storing temporary copies around the world of the content your backend uses. We love these win-win situations: our
                    servers don't have to ask yours for content
                    that isn't changing, your bills are lower,
                    your site is more secure against capacity
                    issues, and your users receive content
                    quicker.
                    <br>
                    <br>
                    Caching misconfigured can produce security risks. It's important to configure caching so it doesn't apply to sensitive data, like API responses.
                    You can override any of these values for a specific condition using <a class="underline" target="_blank" href="./page_rules">Page Rules</a>.
                </p>
            </div>
            <div class="my-4 space-y-4">
                <Card.Root>
                    <Card.Header>
                        <div
                            class="flex justify-between items-center space-x-8"
                        >
                            <div>
                                <Card.Title>Cache stack enabled</Card.Title>
                                <Card.Description
                                    >Our cache stack stores temporary copies of
                                    your content on our servers around the
                                    world. We love these win-win situations: our
                                    servers don't have to ask yours for content
                                    that isn't changing, your bills are lower,
                                    your site is more secure against capacity
                                    issues, and your users receive content
                                    quicker.</Card.Description
                                >
                            </div>
                            <Switch
                                on:click={async () => {
                                    await letSaveChanges();
                                }}
                                bind:checked={cache_enabled_switch}
                            ></Switch>
                        </div>
                    </Card.Header>
                </Card.Root>
                <Card.Root>
                    <Card.Header>
                        <div
                            class="md:flex md:justify-between md:items-center md:space-x-8"
                        >
                            <div>
                                <Card.Title>Default cache level</Card.Title>
                                <Card.Description
                                    >Choose how we decide if content is
                                    cache-able or not. Aggressive means we'll go
                                    after everything. Standard means that we'll
                                    rely on basic indicators (like headers your
                                    backend returns that let us know the content
                                    isn't changing). When query strings are
                                    used, we won't cache the content. Ignore
                                    query string means we'll go the standard
                                    route but ignore query strings and cache and
                                    return cached objects regardless of query.
                                    None isn't recommended - it means we'll stop
                                    caching objects, but we'll continue to serve
                                    cached objects until their time-to-live runs
                                    out. <br><br>By the way, you can override any of
                                    these values for a specific condition using
                                    <a class="underline" target="_blank" href="./page_rules">Page Rules</a>.</Card.Description
                                >
                                <Card.Description class="text-destructive font-semibold"
                                    >It's important to note that Aggressive caching
                                    shouldn't be applied to your entire site unless your content never changes.</Card.Description
                                >
                            </div>
                            <div class="my-1.5 md:my-0">
                                <DropdownMenu.Root>
                                    <DropdownMenu.Trigger asChild let:builder>
                                      <Button variant="outline" builders={[builder]}>{default_cache_level} <ChevronDown /></Button>
                                    </DropdownMenu.Trigger>
                                    <DropdownMenu.Content class="w-56">
                                      <DropdownMenu.Label>Default Cache Level</DropdownMenu.Label>
                                      <DropdownMenu.Separator />
                                      <DropdownMenu.RadioGroup bind:value={default_cache_level}>
                                        <DropdownMenu.RadioItem value="None">None</DropdownMenu.RadioItem>
                                        <DropdownMenu.RadioItem value="Standard">Standard</DropdownMenu.RadioItem>
                                        <DropdownMenu.RadioItem value="IgnoreQueryString">IgnoreQueryString</DropdownMenu.RadioItem>
                                        <DropdownMenu.RadioItem value="Aggressive">Aggressive</DropdownMenu.RadioItem>
                                      </DropdownMenu.RadioGroup>
                                    </DropdownMenu.Content>
                                  </DropdownMenu.Root>
                            </div>
                        </div>
                    </Card.Header>
                </Card.Root>
                <Card.Root>
                    <Card.Header>
                        <div
                            class="md:flex md:justify-between md:items-center md:space-x-8"
                        >
                            <div>
                                <Card.Title>Default cache TTL</Card.Title>
                                <Card.Description
                                    >How long, by default, we should maintain
                                    cached objects. If your origin becomes
                                    unresponsive, we may continue serving cached
                                    content past this TTL.</Card.Description
                                >
                            </div>
                            <div class="flex flex-col w-full max-w-sm gap-1.5 my-3 md:my-0">
                                <Label for="default_ttl">Default TTL (in seconds)</Label>
                                <Input
                                    type="number"
                                    id="default_ttl"
                                    name="default_ttl"
                                    bind:value={default_ttl}
                                />
                                <p class="pt-1">{beautiful_date(default_ttl)}</p>
                            </div>
                        </div>
                    </Card.Header>
                </Card.Root>
            </div>
        </div>
    {/if}
</div>
