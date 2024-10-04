<script>
    import { page } from "$app/stores";
    import DisplayPageRule from "./(components)/DisplayPageRule.svelte";

    import { Button } from "$lib/components/ui/button";
    import * as Dialog from "$lib/components/ui/dialog";
    import {Plus} from "lucide-svelte";
    import * as Drawer from "$lib/components/ui/drawer";
    import {beautifulBytes} from "$lib/utils/beautiful_bytes.js";
    import Breadcrumbs from "$lib/components/Breadcrumbs.svelte";
    import {onMount} from "svelte";
    import {getCookie} from "$lib/utils/auth.js";
    import {PUBLIC_API} from "$env/static/public";
    import {toast} from "svelte-sonner";

    /** @type {import('./$types').PageData} */
    export let data;

    let domain = data.resp.domain.domain;

    let pageRules = domain.rules;

    const ruleExample = {
        order: 1,
        enabled: true,
        trigger: {
            matchType: [{
                trigger: {
                    ip: "127.0.0.1",
                },
                match_type: "Exact",
                inversed: false,
                required: true,
            }],
            trigger_requirement: "All",
            inversed: false,
        },
        action: {
            monopoly: "Block",
            trustbusting: ["Ratelimit", "Redirect"],
            bucket_name: "cache-everything",
            cache_level: "Standard",
            cache_level_ttl: 300,
            redirect: `{domain._id}/redirect`,
            backend_host: "0.0.0.0",
        },
    }

    let rule; // Placeholder for the rule that is being created or edited.

    let createRuleDialog = false; // When the dialog with the form should be open.
    let able_to_save = true;
    let saving = false;

    let domain_info = domain;
    let loaded = false;

    const mounter = async () => {
        for (let i = 0; i < domain_info.rules.rules.length; i++) {
            // go through each of the match types on the rule
            for (let j = 0; j < domain_info.rules.rules[i].trigger.match_type.length; j++) {
                // set the editing property to false
                // domain_info.rules.rules[i].trigger.match_type[j].trigger;

                // temp trigger
                let temp_trigger = {};

                for (let key of Object.keys(domain_info.rules.rules[i].trigger.match_type[j].trigger)) {
                    temp_trigger.key = key;
                    temp_trigger.value = domain_info.rules.rules[i].trigger.match_type[j].trigger[key];
                }

                domain_info.rules.rules[i].trigger.match_type[j].trigger = temp_trigger;
            }
        }

        loaded = true;
    }

    onMount(mounter);

    // get changes function
    const saveChanges = async () => {
        // let changes = [];

        // check if the rule is enabled
        for (let i = 0; i < domain_info.rules.rules.length; i++) {
            // go through each of the match types on the rule
            for (let j = 0; j < domain_info.rules.rules[i].trigger.match_type.length; j++) {
                // set the editing property to false
                // domain_info.rules.rules[i].trigger.match_type[j].trigger;
                delete domain_info.rules.rules[i].trigger.match_type[j].editing;

                if (domain_info.rules.rules[i].trigger.match_type[j].trigger.key?.value) {
                    // it has been changed
                    domain_info.rules.rules[i].trigger.match_type[j].trigger[domain_info.rules.rules[i].trigger.match_type[j].trigger.key.value] = domain_info.rules.rules[i].trigger.match_type[j].trigger.value;
                    delete domain_info.rules.rules[i].trigger.match_type[j].trigger.key;
                    delete domain_info.rules.rules[i].trigger.match_type[j].trigger.value;
                } else {
                    // it has not been changed
                    domain_info.rules.rules[i].trigger.match_type[j].trigger[domain_info.rules.rules[i].trigger.match_type[j].trigger.key] = domain_info.rules.rules[i].trigger.match_type[j].trigger.value;
                    delete domain_info.rules.rules[i].trigger.match_type[j].trigger.key;
                    delete domain_info.rules.rules[i].trigger.match_type[j].trigger.value;
                }
            }

            // if the action is Block, change the monopoly to true
            if (domain_info.rules.rules[i].action.trustbusting.find((element) => element === "Block")) {
                domain_info.rules.rules[i].action.monopoly = "Block";
                domain_info.rules.rules[i].action.trustbusting = [];
            }
        }

        // now that the rules have been prepped, save changes
        let success = false;
        saving = true;

        try {
            const slug = $page.params.slug;

            const token = getCookie("jwt");

            (async () => {
                await fetch(`${PUBLIC_API}/@/project/${$page.params.project}/domain/${$page.params.id}/rules/update`, {
                    method: "POST",
                    headers: new Headers({
                        "content-type": "application/json",
                        Authorization: token,
                    }),
                    body: JSON.stringify({
                        domain: slug,
                        rules: domain_info.rules.rules,
                        enabled: domain_info.rules.enabled,
                        time: new Date(),
                        __foo_confirm: true,
                    }),
                })
                    .then(async (res) => {
                        let response = await res.json();
                        if (res.status !== 200) {
                            saving = false;
                            await saveFailed();
                            toast.error(`Failed to update: ${response.message}`);
                        } else {
                            success = true;
                        }
                    })
                    .catch(async (err) => {
                        saving = false;
                        await saveFailed();
                        toast.error(`Failed to update: ${err}`);
                    });
            })().then(async () => {
                if (success) {
                    saving = false;
                    // await load();
                    toast.success(`Pushed changes to edge`);

                    location.reload();
                }
            });
        } catch (err) {
            saving = false;
            await saveFailed();
            toast.error(`Failed to update: ${err}`);
        }
    };

    async function saveFailed() {
        // if the save failed, set the saving variable to false
        await mounter();
    }

    function getHighestPossibleOrder() {
        // iterate through the rules and get the highest order available
        let highest = 1;

        for (let i = 0; i < domain_info.rules.rules.length; i++) {
            if (domain_info.rules.rules[i].order > highest) {
                highest = domain_info.rules.rules[i].order;
            }
        }

        return highest;
    }
</script>

{#if loaded}
<!-- Create Rule -->
<Dialog.Root bind:open={createRuleDialog}>
    <Dialog.Content>
        <Dialog.Header>
            <Dialog.Title>New Page Rule</Dialog.Title>
            <Dialog.Description>
                This form allows you to create a new page rule for the domain.
            </Dialog.Description>
        </Dialog.Header>

    </Dialog.Content>
</Dialog.Root>

<!-- <div>
            <p class="uppercase text-gray-300">at a glance</p>
            <p class="text-slate-200 text-xl">API engine has handled 239,392 requests</p>
        </div> -->
<div
        class="w-full p-5 rounded-b-2xl shadow"
>
<div class="flex flex-wrap items-center justify-between">
    <div class="uppercase text-muted-foreground px-4">
        Page Rules
    </div>
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
        <Button class="my-1.5 md:my-0" href="/docs/optimization" variant="secondary"
        >Documentation</Button
        >
    </div>
</div>

<div class="mt-4 flex flex-wrap justify-end">
    <p class="p-4">
        Page Rules can be thought of as a system where inputs from the request, like IP address, path, or host, are matched against values
        you specify. When triggered, the rule will apply actions you define, like ratelimiting or modifying cache settings.
        <br>
        <br>
        Examples of Page Rules could be whitelisting IP addresses to administrative paths, blocking requests from certain countries,
        disabling integrity checks for specific networks, redirecting requests with a specific path, or modifying cache settings for API endpoints
    </p>

    <Button
            on:click={() => pageRules.rules = [{
                enabled: true,
                order: getHighestPossibleOrder(),
                trigger: {
                    match_type: [],
                    trigger_requirement: "One",
                    inversed: false
                },
                action: {
                    trustbusting: [],
                },
            }, ...pageRules.rules]}
            variant="primary"
    >
        Create new rule
    </Button>
</div>

<div class="my-4 space-y-4">
    {#each pageRules.rules as pageRule}
        <DisplayPageRule rule={pageRule} domain_info={domain} />
    {/each}
</div>
</div>
    {/if}

