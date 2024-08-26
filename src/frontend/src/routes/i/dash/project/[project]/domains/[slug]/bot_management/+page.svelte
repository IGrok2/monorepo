<script>
    import { page } from "$app/stores";
    import { onMount } from "svelte";
    import { fade, fly, slide, draw } from "svelte/transition";
    import Options from "$lib/Options.svelte";
    import { Button } from "$lib/components/ui/button";
    import * as Card from "$lib/components/ui/card";
    import { Switch } from "$lib/components/ui/switch";
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
    import { ChevronDown } from "lucide-svelte";
    import { getCookie } from "$lib/utils/auth";
    import * as Drawer from "$lib/components/ui/drawer";

    import { PUBLIC_API } from "$env/static/public";
    import {toast} from "svelte-sonner";
    import Breadcrumbs from "$lib/components/Breadcrumbs.svelte";

    // For loading the dashboard
    let loaded = false;

    /** @type {import('./$types').PageData} */
    export let data;

    console.log(data);

    const domain_info = data.resp.domain.domain;

    // Basic saving class properties
    let saved_class = "cursor-not-allowed bg-gray-400";
    let saving = false;
    let able_to_save = false;
    let confirmation_title;
    let confirmation_message;
    let confirmation_function;
    let time_based;
    let override = false;

    // bot management specific settings
    let bot_management_enabled_switch = domain_info.bot_management.enabled;
    let bots = domain_info.bot_management.bots_allowed;
    let formatted_bot_string = "";

    // format the bots we get onMount
    onMount(async () => {
        let formatted_bots = [];

        console.log(`onMount bots: ${JSON.stringify(bots)}`);

        for (const bot of bots) {
            formatted_bots.push({
                _id: bot,
                new_staged: false,
                being_deleted: false,
            });
        }

        bots = formatted_bots;

        displayBots(bots);

        time_based = Date.now();
    });

    loaded = true;

    // when this function is called, this is what we expect
    // bot doesn't exist --> create bot
    // bot does exist and is staged or exists --> remove bot
    // bot does exist and is being deleted --> unmark bot as being deleted
    const addBot = (name) => {
        let found = false;
        let can_delete = false;

        for (let i = 0; i < bots.length; i++) {
            if (bots[i]._id === name) {
                found = true;
                can_delete = true;

                if (bots[i].being_deleted) {
                    bots[i].being_deleted = false;
                    can_delete = false;
                }

                break;
            }
        }

        if (!found) {
            bots.push({
                _id: name,
                new_staged: true,
                being_deleted: false,
            });
        } else if (can_delete) {
            deleteBot(name);
        }

        console.log(`added: ${JSON.stringify(bots)}`);

        letSaveChanges();
    };

    const deleteBot = (name) => {
        for (let i = 0; i < bots.length; i++) {
            if (bots[i]._id === name) {
                if (bots[i].new_staged) {
                    bots.splice(i, 1);
                } else bots[i].being_deleted = true;

                break;
            }
        }

        console.log(`delete ${name}`);

        letSaveChanges();
    };

    const newBots = () => {
        let newBots = [];

        for (let i = 0; i < bots.length; i++) {
            if (bots[i].new_staged === true) {
                newBots.push(bots[i]);
            }
        }

        return newBots;
    };

    const deletedBots = () => {
        let deletedBots = [];

        for (let i = 0; i < bots.length; i++) {
            if (bots[i].being_deleted === true) {
                deletedBots.push(bots[i]);
            }
        }

        return deletedBots;
    };

    // display the bots as a string
    const displayBots = () => {
        let display = "";

        for (let i = 0; i < bots.length; i++) {
            if (bots[i].being_deleted === false) {
                if (i === bots.length - 1) {
                    display += `${bots[i]._id}`;
                } else {
                    display += `${bots[i]._id}, `;
                }
            }
        }

        formatted_bot_string = display;
    };

    // if the response was a success
    let success = false;

    const saveChanges = async () => {
        // save changes to bot settings
        if (able_to_save === true) {
            saving = true;
            save_button_success = true;

            try {
                const slug = $page.params.slug;

                const token = getCookie("jwt");

                (async () => {
                    // get the name of each bot
                    let formatted_bots = [];

                    for (const bot of bots) {
                        if (!bot.being_deleted) formatted_bots.push(bot._id);
                    }

                    await fetch(
                        `${PUBLIC_API}/@/project/${$page.params.project}/domain/${$page.params.id}/bot_management/update`,
                        {
                            method: "POST",
                            headers: new Headers({
                                "content-type": "application/json",
                                Authorization: token,
                            }),
                            body: JSON.stringify({
                                domain: slug,
                                enabled: bot_management_enabled_switch,
                                bots_allowed: formatted_bots,
                                time: time_based,
                                __foo_confirm: true,
                            }),
                        },
                    )
                        .then(async (res) => {
                            let response = await res.json();

                            console.log(`resposne code: ${res.status}`);

                            if (res.status !== 200) {
                                console.log(`response status was not 200`)
                                saving = false;
                                show_staged_changes = false;
                                save_button_success = false;

                                await toast.error(
                                    `Failed to update settings: ${response.message}`,
                                );
                            } else {
                                success = true;
                            }
                        })
                        .catch(async (err) => {
                            saving = false;
                            show_staged_changes = false;
                            save_button_success = false;

                            await toast.error(`Failed to update settings: try refreshing the page: ${err}`);
                        });
                })().then(async () => {
                    if (success) {
                        saving = false;
                        show_staged_changes = false;
                        save_button_success = false;
                        able_to_save = false;
                        success = false;

                        await toast.success("Pushed changes to edge");

                        location.reload();
                    }
                });
            } catch (err) {
                saving = false;
                show_staged_changes = false;
                save_button_success = false;

                await toast.error(`Failed to update settings: try refreshing the page: ${err}`);
            }
        }
    };

    const letSaveChanges = () => {
        displayBots(bots);
        console.log(
            `bot_management_enabled_switch: ${bot_management_enabled_switch}`,
        );
        // modifies able_to_save based on params
        able_to_save =
            domain_info.bot_management.enabled !==
                bot_management_enabled_switch ||
            newBots().length !== 0 ||
            deletedBots().length !== 0;

        able_to_save
            ? (saved_class = "bg-fuchsia-500")
            : (saved_class = "cursor-not-allowed bg-gray-400");
    };

    // register variable listeners
    $: bot_management_enabled_switch, letSaveChanges();

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

<div class="flex h-full w-full">
    {#if loaded}
        <div
            class="w-full p-5 rounded-b-2xl shadow"
        >
            <!-- <div>
                <p class="uppercase text-gray-300">at a glance</p>
                <p class="text-slate-200 text-xl">API engine has handled 239,392 requests</p>
            </div> -->
            <div class="flex items-center justify-between">
                <p class="uppercase text-muted-foreground">Bot engine settings</p>
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
                                                {#if bot_management_enabled_switch !== domain_info.bot_management.enabled}
                                                    {#if bot_management_enabled_switch === true}
                                                        <li>
                                                            <a class="text-green-500"
                                                            >Enable</a
                                                            > bot engine (will activate whitelisted
                                                            bots)
                                                        </li>
                                                    {:else}
                                                        <li>
                                                            <a class="text-red-500"
                                                            >Disable</a
                                                            > bot engine (whitelisted bots will
                                                            be paused)
                                                        </li>
                                                    {/if}
                                                {/if}

                                                {#if newBots().length !== 0}
                                                    <li>
                                                        <a class="text-green-500">Allow</a> bots:
                                                    </li>
                                                    <ol class="list-disc pl-6">
                                                        {#each bots as bot}
                                                            {#if bot.new_staged}
                                                                <li class="font-light">
                                                                    {bot._id}
                                                                </li>
                                                            {/if}
                                                        {/each}
                                                    </ol>
                                                {/if}

                                                {#if deletedBots().length !== 0}
                                                    <li>
                                                        <a class="text-red-500">Remove</a> bots:
                                                    </li>
                                                    <ol class="list-disc pl-6">
                                                        {#each bots as bot}
                                                            {#if bot.being_deleted}
                                                                <li class="font-light">
                                                                    {bot._id}
                                                                </li>
                                                            {/if}
                                                        {/each}
                                                    </ol>
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
                    <Button href="/docs/api_engine" class="my-1.5 md:my-0" variant="secondary"
                        >Documentation</Button
                    >
                </div>
            </div>
            <div>
                <div class="h-full">
                    <div class="md:flex items-center justify-between py-5">
                        <p class="">
                            Bot engine allows specific good bots to bypass Human Engine and API Engine.
                            We have high standards for the bots we allow, and we monitor them for abuse.
                            <br><br>
                            You have control which bots that should be allowed to visit your site.
                            Use <a class="underline" target="_blank" href="./page_rules">Page Rules</a> for more granular control.
                        </p>
                    </div>
                </div>

                <div class="my-4 space-y-4">
                    <Card.Root>
                        <Card.Header>
                            <div
                                class="flex justify-between items-center space-x-8"
                            >
                                <div>
                                    <Card.Title>Bot management enabled</Card.Title>
                                    <Card.Description
                                        >Bot management allows specific bots to be
                                        allowed past Human Engine, such as search
                                        engine crawlers such Googlebot or uptime
                                        trackers such as UptimeRobot.</Card.Description
                                    >
                                </div>
                                <Switch
                                    bind:checked={bot_management_enabled_switch}
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
                                    <Card.Title>Bots allowed</Card.Title>
                                    <Card.Description
                                        >Specify which bots to allow past Human
                                        Engine. Bots must still comply with
                                        user-defined rules, and bot activity is
                                        automatically monitored for any signs of
                                        abuse.</Card.Description
                                    >
                                </div>
                                <div class="my-3 md:my-0">
                                    <DropdownMenu.Root>
                                        <DropdownMenu.Trigger asChild let:builder>
                                            <Button
                                                variant="outline"
                                                builders={[builder]}
                                                >{formatted_bot_string} <ChevronDown /></Button
                                            >
                                        </DropdownMenu.Trigger>
                                        <DropdownMenu.Content class="w-56">
                                            <DropdownMenu.Label>Bots</DropdownMenu.Label
                                            >
                                            <DropdownMenu.Separator />
                                            <DropdownMenu.CheckboxItem
                                                    on:click={() => {
                                                        addBot("UptimeRobot");
                                                    }}
                                                value="UptimeRobot"
                                                >UptimeRobot</DropdownMenu.CheckboxItem
                                            >
                                            <DropdownMenu.CheckboxItem
                                                    on:click={() => {
                                                        addBot("Bingbot");
                                                    }}
                                                    value="Bingbot"
                                                >Bingbot</DropdownMenu.CheckboxItem
                                            >
                                            <DropdownMenu.CheckboxItem
                                                    on:click={() => {
                                                        addBot("Googlebot");
                                                    }}
                                                    value="Googlebot"
                                                >Googlebot</DropdownMenu.CheckboxItem
                                            >
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
</div>
