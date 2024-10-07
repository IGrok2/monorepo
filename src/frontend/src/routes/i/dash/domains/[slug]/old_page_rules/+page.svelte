<script>
    import { page } from '$app/stores';
    import {onMount} from "svelte";
    import { fade, fly, slide, draw } from 'svelte/transition';
    import Options from "$lib/Options.svelte";
    import OneOption from "$lib/OneOption.svelte";
    import Input from "$lib/Input.svelte";
    import Menu from "$lib/Menu.svelte";
    import {Confetti} from "svelte-confetti";
    import { getCookie } from "$lib/utils/auth.js";

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

    // API Engine specific settings
    let page_rules_enabled_switch;
    // rule specific stuff
    let rules = [];

    // rules
    let new_rule_popup;
    let new_rule_setting;

    let new_rule_matches = [{
        triggerSelected: null,
        trigger: { query: {}, headers: {} },
        match_type: "Exact",
        inversed: false,
        required: false
    }];
    let new_rule_trigger_requirement = "One";
    let new_rule_origin_popup = false;
    let new_rule_ratelimit_popup = false;
    let new_rule_order;
    let new_rule_allow_query_string;
    let new_rule_ws_methods = [];
    let new_rule_http_methods = [];
    let new_rule_actions = [];
    let new_rule_actiontype = "";
    let new_rule_monopoly = "";
    let new_rule_trustbust = [];
    let new_rule_cache_settings = {
        level: null,
        ttl: null
    }; // level and ttl
    let new_rule_bucket;
    let new_rule_redirect;
    let new_rule_backend_host;

    onMount(async () => {
        try {
            const slug = $page.params.slug;

            const token = getCookie("jwt")

            await fetch("http://localhost:3030/@/project/${$page.params.project}/domain/${$page.params.id}", {
                method: "POST",
                headers: new Headers({'content-type': 'application/json', 'Authorization': token}),
                body: JSON.stringify({
                    domain: slug
                })
            }).then(async (rawBody) => {
                let body = await rawBody.json();

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

                page_rules_enabled_switch = domain_info.rules.enabled;
                rules = domain_info.rules.rules;

                console.log(rules)

                loaded = true;
            })
        } catch (err) {
            console.log(err);

            await error_notification("Error loading user profile", "Redirecting to login page ...")

            document.location.href = "/i/auth/login";
        }
    })

    const letSaveChanges = async () => {
        if (!(await checkForEditedRules())
            && page_rules_enabled_switch === domain_info.rules.enabled) {

            saved_class = "cursor-not-allowed bg-gray-400";
            able_to_save = false

        } else {
            saved_class = "bg-fuchsia-500";
            able_to_save = true;
        }
    }

    // check if there are any staged or new rules
    const checkForEditedRules = async () => {
        for (const rule of rules) {
            if (rule.new_staged === true || rule.existed_modified === true || rule.being_deleted) {
                return true;
            }
        }

        return false;
    }

    // for enable button
    const enabledButtonClick = async () => {
        page_rules_enabled_switch = !page_rules_enabled_switch
        await letSaveChanges()
    }

    const formatOriginIps = (origin_settings) => {
        let formatted = [];
        for (let i = 0; i < origin_settings.origins.length; i++) {
            formatted.push(origin_settings.origins[i].url)
        }
        return formatted
    }

    // sort page rules by order
    const sortRules = async () => {
        rules.sort((a, b) => {
            return a.order - b.order;
        })
    }

    /*
    Things to check
- Is first match filled out
- Are all matches selected filled out
- Origin setting, redirect, cache, ratelimit if selected, is option selected too
- Order filled out
- Continent (is two characters?)
- Country (is two characters)
- ASN (AS400495)
- IP address (does it match IP address schema)
     */
    const newPageRule = async () => {
        if (new_rule_matches.length === 0) {
            await error_notification("Couldn't stage new page rule", "The first rule wasn't filled out")
            return;
        }

        for (let i = 0; i < new_rule_matches.length; i++) {
            if (new_rule_matches[i].triggerSelected === null || new_rule_matches[i].match_type === null) {
                await error_notification("Couldn't stage new page rule", `Match ${i + 1} wasn't filled out`)
                return;
            }

            if (new_rule_matches[i].triggerSelected === "Query string") {
                if (Object.keys(new_rule_matches[i].trigger.query).length === 0) {
                    await error_notification("Couldn't stage new page rule", `Match ${i + 1} wasn't filled out`)
                    return;
                }
            } else if (new_rule_matches[i].triggerSelected === "Header") {
                if (Object.keys(new_rule_matches[i].trigger.headers).length === 0) {
                    await error_notification("Couldn't stage new page rule", `Match ${i + 1} wasn't filled out`)
                    return;
                }
            } else if (new_rule_matches[i].triggerSelected === "IP address") {
                // check if IP is filled out, then check if it's a valid IP
                if (!new_rule_matches[i].trigger.ip) {
                    await error_notification("Couldn't stage new page rule", `Match ${i + 1} wasn't filled out`)
                    return;
                }

                const reg = /^(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$/;

                if (!reg.test(new_rule_matches[i].trigger.ip)) {
                    await error_notification("Couldn't stage new page rule", `Match ${i + 1} had an invalid IP address`)
                    return;
                }
            } else if (new_rule_matches[i].triggerSelected === "HTTP Path") {
                // make sure it contains a slash
                if (!new_rule_matches[i].trigger.path || !new_rule_matches[i].trigger.path.includes("/")) {
                    await error_notification("Couldn't stage new page rule", `Match ${i + 1} didn't have a correct path`)
                    return;
                }
            } else if (new_rule_matches[i].triggerSelected === "ASN") {
                // make sure it's a number
                if (!new_rule_matches[i].trigger.asn || isNaN(new_rule_matches[i].trigger.asn)) {
                    await error_notification("Couldn't stage new page rule", `Match ${i + 1} didn't have a correct ASN. Should be a number!`)
                    return;
                }
            } else if (new_rule_matches[i].triggerSelected === "Country") {
                // make sure it's two characters
                if (!new_rule_matches[i].trigger.country || new_rule_matches[i].trigger.country.length !== 2) {
                    await error_notification("Couldn't stage new page rule", `Match ${i + 1} didn't have a correct country`)
                    return;
                }
            } else if (new_rule_matches[i].triggerSelected === "Continent") {
                // make sure it's two characters
                if (!new_rule_matches[i].trigger.continent || new_rule_matches[i].trigger.continent.length !== 2) {
                    await error_notification("Couldn't stage new page rule", `Match ${i + 1} didn't have a correct continent`)
                    return;
                }
            }
        }

        // now, check triggers. first, make sure there's at least one
        if (!new_rule_actiontype) {
            await error_notification("Couldn't stage new page rule", `No action type was selected`)
            return;
        } else if (new_rule_actiontype === "Monopoly" && !new_rule_monopoly) {
            await error_notification("Couldn't stage new page rule", `Monopoly action was indicated, but no monopoly was selected`)
            return;
        } else if (new_rule_actiontype === "Trustbust" && (!new_rule_trustbust || new_rule_trustbust.length === 0)) {
            await error_notification("Couldn't stage new page rule", `Trustbust action was indicated, but no trustbust was selected`)
            return;
        }

        if (true) {
            new_rule_popup = false;

            rules = [...rules, {
                new_staged: true,
                existed_modified: false,
                being_deleted: false,
                order: new_rule_order,
                match: new_rule_matches,
                actions: new_rule_actions,
                action_type: new_rule_actiontype,
                monopoly: new_rule_monopoly,
                trustbust: new_rule_trustbust,
                cache_settings: new_rule_cache_settings,
                bucket: new_rule_bucket,
                redirect: new_rule_redirect,
                backend_host: new_rule_backend_host
            }]

            await sortRules()

            await letSaveChanges()
        } else {
            await error_notification("Couldn't stage new API Engine rule", "Didn't include any paths")
        }
    }


    const deletePageRule = async (rule, index) => {
        await confirmChange(`Delete ${rule.new_staged ? `staged` : `existing`} page rule?`, `${rule.new_staged ? "Because this rule was staged, nothing will change in your production environment" : "This rule will be staged for deletion"}`,
            () => { // TODO: deleting the setting doesn't delete the rules yet
                if (rule.new_staged) {
                    rules.splice(index, 1);
                    rules = rules;
                } else {
                    rule.being_deleted = true;
                }
            })
    }

    // get new rulses
    const getNewRules = () => {
        let new_rules = [];

        for (let i = 0; i < rules.length; i++) {
            let rule = rules[i];

            if (rule.new_staged === true) {
                new_rules.push(rule)
            }
        }

        return new_rules
    }

    // get changed rules
    const getChangedRules = () => {
        let changed_rules = [];

        for (let i = 0; i < rules.length; i++) {
            let rule = rules[i];

            if (rule.existed_modified === true) {
                changed_rules.push(rule)
            }
        }

        return changed_rules
    }

    // get deleted settings
    const getDeletedRules = () => {
        let deleted_rules = [];

        for (let i = 0; i < rules.length; i++) {
            let rule = rules[i];

            if (rule.being_deleted === true) {
                deleted_rules.push(rule)
            }
        }

        return deleted_rules
    }


    const saveChanges = async () => {
        if (able_to_save === true) {
            saving = true;
            save_button_success = true;
            try {
                const slug = $page.params.slug;

                const token = getCookie("jwt");

                (async () => {
                    if (rules.length !== 0) {
                        for (let i = 0; i < rules.length; i++) {
                            if (rules[i].new_staged) {
                                rules[i].new_staged = undefined;
                                rules[i].existed_modified = undefined;
                                rules[i].being_deleted = undefined;

                                await fetch("https://iucuwv-ip-24-189-107-128.tunnelmole.net/@/domain/api-engine/add-setting", {
                                    method: "POST",
                                    headers: new Headers({'content-type': 'application/json', 'Authorization': token}),
                                    body: JSON.stringify({
                                        domain: slug,
                                        _id: rules[i]._id,
                                        // TODO add the rest of the properties
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
                                    })
                            } else if (rules[i].existed_modified && rules[i].changed) {
                                rules[i].new_staged = undefined;
                                rules[i].existed_modified = undefined;
                                rules[i].being_deleted = undefined;

                                const c = rules[i].changed;

                                console.log("here like actually", c);

                                if (c.whitelist_factors.new_ips) {
                                    console.log("wtf");

                                    await fetch("https://iucuwv-ip-24-189-107-128.tunnelmole.net/@/domain/api-engine/setting/add-whitelistfactor", {
                                        method: "POST",
                                        headers: new Headers({'content-type': 'application/json', 'Authorization': token}),
                                        body: JSON.stringify({
                                            domain: slug,
                                            setting_id: rules[i]._id,
                                            whitelist_factors: {
                                                ips: c.whitelist_factors.new_ips
                                            }
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
                                        })
                                }
                            }
                        }
                    }

                    if (page_rules_enabled_switch !== domain_info.rules.enabled) {
                        await fetch("https://iucuwv-ip-24-189-107-128.tunnelmole.net/@/domain/api-engine/update-basic-settings", {
                            method: "POST",
                            headers: new Headers({
                                'content-type': 'application/json',
                                'Authorization': token
                            }),
                            body: JSON.stringify({
                                domain: slug,
                                enabled: api_engine_enabled_switch,
                                strict_mode_enabled: api_engine_strict_mode_enabled_switch
                            })
                        }).then(async (res) => {
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
                        })
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

    let classes;
    let message;
    let submessage;

    let error;
    let error_submessage;

    const newMatch = () => {
        new_rule_matches = [...new_rule_matches, {trigger: { query: {}, headers: {} }, match_type: "Exact", inversed: false, required: false}]
    }

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
                                    {#if page_rules_enabled_switch !== domain_info.rules.enabled}
                                        {#if page_rules_enabled_switch === true}
                                            <li><a class="text-green-500">Enable</a> page rules (rules will be activated)</li>
                                        {:else}
                                            <li><a class="text-red-500">Disable</a> page rules (all rules will be paused)</li>
                                        {/if}
                                    {/if}
<!--
                                    {#if getNewSettings().length !== 0}
                                        <li><a class="text-green-500">New</a> API engine settings:</li>
                                        <ol class="list-disc pl-6">
                                            {#each getNewSettings() as new_setting}
                                                <li class="font-light">{new_setting._id}
                                                    {#if new_setting.whitelist_factors.ips.length !== 0}
                                                        <div class="pl-8">
                                                            <p class="text-gray-700">With whitelisted IPs:</p>
                                                            <ol class="pl-10 list-desc text-gray-500">
                                                                {#each new_setting.ips as ip}
                                                                    <li>{ip}</li>
                                                                {/each}
                                                            </ol>
                                                        </div>
                                                    {/if}

                                                    {#if getChangedApiRulesFromSetting(new_setting._id).length !== 0}
                                                        <div class="pl-8">
                                                            <p class="text-gray-700">With new rules</p>
                                                            <ol class="pl-10 list-desc text-gray-500">
                                                                {#each getChangedApiRulesFromSetting(new_setting._id) as rule}
                                                                    <li>{rule.path} ({rule.match_type.toLowerCase()}) - methods: {(() => rule.http_methods.length !== 0 ? `HTTP methods: ${rule.http_methods} (query string allowed: ${(rule.allow_query_string ?? "false")})`: `no HTTP methods`)()}, {(() => rule.ws_methods.length !== 0 ? `WS methods: ${rule.ws_methods}`: `no WebSocket methods`)()}{rule.actions.length !== 0 ? `, ${rule.actions}` : ``}<br></li>
                                                                {/each}
                                                            </ol>
                                                        </div>
                                                    {/if}
                                                </li>
                                            {/each}
                                        </ol>
                                    {/if}

                                    {#if getChangedSettings().length !== 0}
                                        <li><a class="text-yellow-500">Changed</a> API engine settings:</li>
                                        <ol class="list-disc pl-6">
                                            {#each getChangedSettings() as changed_setting}
                                                <li class="font-light">{changed_setting._id}
                                                    {#if (changed_setting.changed.open_api === true || changed_setting.changed.open_api === false) && changed_setting.changed.open_api !== changed_setting.open_api}
                                                        <div class="pl-8">
                                                            {#if changed_setting.changed.open_api === true}
                                                                <p class="text-gray-700"><a class="text-green-500">Allow</a> open API functionality</p>
                                                            {:else}
                                                                <p class="text-gray-700"><a class="text-red-500">Disallow</a> open API functionality</p>
                                                            {/if}
                                                        </div>
                                                    {/if}

                                                    {#if changed_setting.changed.whitelist_factors && changed_setting.changed.whitelist_factors.new_ips && changed_setting.changed.whitelist_factors.new_ips.length !== 0}
                                                        <div class="pl-8">
                                                            <p class="text-gray-700">With <a class="text-green-500">new</a> whitelisted IPs:</p>
                                                            <ol class="pl-10 list-desc text-gray-500">
                                                                {#each changed_setting.changed.whitelist_factors.new_ips as ip}
                                                                    <li>{ip}</li>
                                                                {/each}
                                                            </ol>
                                                        </div>
                                                    {/if}

                                                    {#if changed_setting.changed.whitelist_factors && changed_setting.changed.whitelist_factors.deleted_ips && changed_setting.changed.whitelist_factors.deleted_ips.length !== 0}
                                                        <div class="pl-8">
                                                            <p class="text-gray-700">With <a class="text-red-500">deleted</a> whitelisted IPs:</p>
                                                            <ol class="pl-10 list-desc text-gray-500">
                                                                {#each changed_setting.changed.whitelist_factors.deleted_ips as ip}
                                                                    <li>{ip}</li>
                                                                {/each}
                                                            </ol>
                                                        </div>
                                                    {/if}
                                                </li>
                                            {/each}
                                        </ol>
                                    {/if}
-->
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
    <Menu page_rules={true} api_engine_class={api_engine} page_rules_class={page_rules} optimization_class={cache_engine} bot_engine_class={bot_engine} />

    {#if new_rule_popup}
        <div in:fade out:slide|global class="relative z-40" aria-labelledby="slide-over-title" role="dialog" aria-modal="true">
            <div class="fixed inset-0 bg-gray-700 bg-opacity-60 transition-opacity bg-no-repeat bg-center"></div>
            <div class="fixed inset-0 overflow-hidden">
                <div class="absolute inset-0 overflow-hidden">
                    <div class="pointer-events-none fixed inset-y-0 right-0 flex max-w-full pl-10">
                        <div class="pointer-events-auto w-screen max-w-md">
                            <div class="flex h-full flex-col overflow-y-scroll bg-gray-700 py-6 shadow-xl">
                                <div class="px-4 sm:px-6">
                                    <div class="flex items-start justify-between">
                                        <div class="text-slate-100">
                                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M16.862 4.487l1.687-1.688a1.875 1.875 0 112.652 2.652L6.832 19.82a4.5 4.5 0 01-1.897 1.13l-2.685.8.8-2.685a4.5 4.5 0 011.13-1.897L16.863 4.487zm0 0L19.5 7.125" />
                                            </svg>
                                        </div>
                                        <h2 class="font-semibold leading-6 text-slate-100 text-xl" id="slide-over-title">Create a page rule</h2>
                                        <div class="ml-3 flex h-7 items-center">
                                            <button on:click={() => { new_rule_popup = false }} type="button" class="rounded-md text-slate-100 hover:text-slate-400 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2">
                                                <span class="sr-only">Close panel</span>
                                                <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                                                    <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                                                </svg>
                                            </button>
                                        </div>
                                    </div>
                                </div>
                                <form class="relative mt-6 flex-1 px-4 sm:px-6" on:submit|preventDefault={newPageRule}>
                                    <div>
                                        <div class="flex justify-between flex-1 items-center">
                                            <div class="justify-start">
                                                <label class="text-xl block font-medium text-slate-100 pt-6">If...</label>
                                                <a class="text-gray-300 text-sm tracking-tight">Specify how this page rule should be matched</a>
                                            </div>
                                            <div class="justify-end items-center">
                                                <button on:click={() => { newMatch() }} class="bg-fuchsia-500 text-white px-3 py-1.5 mx-3 rounded text-sm">
                                                    +
                                                </button>
                                            </div>
                                        </div>
                                        <div class="bg-opacity-10 bg-background p-3 mt-5 rounded-lg"> <!-- or bg white? -->
                                            {#each new_rule_matches as match, match_index}
                                                <div class="{match_index !== 0 ?? `mt-6`} p-3">
                                                    <!-- trigger: {},
                                                    match_type: null,
                                                    inversed: false,
                                                    required: false -->
                                                    <div class="flex flex-1 justify-between">
                                                        <p class="text-sm uppercase text-slate-300">Match #{match_index+1}</p>
                                                        <div on:click={() => { new_rule_matches.splice(match_index, 1); new_rule_matches = new_rule_matches; }} class="text-xl text-red-500 cursor-grab">𝕏</div>
                                                    </div>

                                                    <div on:keydown={async (event) => {
                                                        if (event.key === "Enter") {
                                                            event.preventDefault();

                                                            if (new_rule_matches.length >= 10) {
                                                                await error_notification("Too many matches", "Only 10 matches are allowed");
                                                                return
                                                            }

                                                            if (match.trigger.ip) {
                                                                const reg = /^(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$/;

                                                                if (!reg.test(match.trigger.ip)) {
                                                                    await error_notification("Invalid IP address", "This IP address won't be accepted by rule engine");
                                                                    return
                                                                }

                                                                for (const rule_match of new_rule_matches) {
                                                                    if (rule_match.trigger.ip === match.trigger.ip && rule_match !== match) {
                                                                        await error_notification("Duplicate IP address", "You can't have two rules with the same IP address");
                                                                        return
                                                                    }
                                                                }
                                                            }

                                                            // newMatch()
                                                        }
                                                    }}>
                                                        <OneOption bind:selected={match.triggerSelected} options={["IP address", "HTTP Path",
                                                        "Query string", "ASN", "Country", "Continent", "Header"]} />
                                                        {#if match.triggerSelected === "Continent"}
                                                            <div>
                                                                <a class="text-gray-300 text-sm tracking-tight">The two-digit continent to match (ex: NA, EU)</a>
                                                                <Input max={2} min={2} required={true} bind:binder={match.trigger.continent} placeholder="Enter a two-digit continent code" />
                                                            </div>
                                                        {/if}

                                                        {#if match.triggerSelected === "Country"}
                                                            <div>
                                                                <a class="text-gray-300 text-sm tracking-tight">The two-digit country to match (ex: US)</a>
                                                                <Input bind:binder={match.trigger.country} placeholder="Enter a two-digit country code" />
                                                            </div>
                                                        {/if}

                                                        {#if match.triggerSelected === "ASN"}
                                                            <div>
                                                                <a class="text-gray-300 text-sm tracking-tight">The ASN to match</a>
                                                                <Input bind:binder={match.trigger.asn} placeholder="Enter an ASN..." />
                                                            </div>
                                                        {/if}

                                                        {#if match.triggerSelected === "IP address"}
                                                            <div>
                                                                <a class="text-gray-300 text-sm tracking-tight">The IP address to match</a>
                                                                <Input bind:binder={match.trigger.ip} placeholder="Enter an IP..." />
                                                            </div>
                                                        {/if}

                                                        {#if match.triggerSelected === "HTTP Path"}
                                                            <div>
                                                                <a class="text-gray-300 text-sm tracking-tight">The path to match</a>
                                                                <Input bind:binder={match.trigger.path} placeholder="Enter a path..." />
                                                            </div>
                                                        {/if}

                                                        {#if match.triggerSelected === "Query string"}
                                                            <div>
                                                                <a class="text-gray-300 text-sm tracking-tight">Query string key/value to match</a>
                                                                <div class="my-1">
                                                                    <Input bind:binder={match.trigger.query.key} placeholder="Enter a key..." />
                                                                    <Input bind:binder={match.trigger.query.value} placeholder="Enter a value..." />
                                                                </div>
                                                            </div>
                                                        {/if}

                                                        {#if match.triggerSelected === "Header"}
                                                            <div>
                                                                <a class="text-gray-300 text-sm tracking-tight">Header string key/value to match</a>
                                                                <div class="my-1">
                                                                    <Input bind:binder={match.trigger.headers.key} placeholder="Enter a key..." />
                                                                    <Input bind:binder={match.trigger.headers.value} placeholder="Enter a value..." />
                                                                </div>
                                                            </div>
                                                        {/if}

                                                        {#if match.triggerSelected}
                                                            <div class="mt-3">
                                                                <a class="text-gray-300 text-sm tracking-tight">Specify how this match should be completed. For key-value properties, this completes the match on key and value.</a>
                                                                <OneOption bind:selected={match.match_type} options={["Exact", "Contains", "StartsWith"]} />
                                                            </div>

                                                            <div class="flex flex-1 justify-between mt-3">
                                                                <a class="text-gray-300 text-sm tracking-tight">Inverse</a>
                                                                {#if !match.inverse}
                                                                    <div class="mt-5 sm:ml-6 sm:mt-0 sm:flex sm:flex-shrink-0 sm:items-center">
                                                                        <!-- Enabled: "bg-indigo-600", Not Enabled: "bg-gray-200" -->
                                                                        <button on:click={() => match.inverse = true} type="button" class="bg-gray-400 relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-600 focus:ring-offset-2" role="switch" aria-checked="false" aria-labelledby="renew-subscription-label" aria-describedby="renew-subscription-description">
                                                                            <!-- Enabled: "translate-x-5", Not Enabled: "translate-x-0" -->
                                                                            <span aria-hidden="true" class="translate-x-0 inline-block h-5 w-5 transform rounded-full bg-background shadow ring-0 transition duration-200 ease-in-out"></span>
                                                                        </button>
                                                                    </div>
                                                                {:else}
                                                                    <div class="mt-5 sm:ml-6 sm:mt-0 sm:flex sm:flex-shrink-0 sm:items-center">
                                                                        <!-- Enabled: "bg-indigo-600", Not Enabled: "bg-gray-200" -->
                                                                        <button on:click={() => match.inverse = false} type="button" class="bg-indigo-600 relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-600 focus:ring-offset-2" role="switch" aria-checked="false" aria-labelledby="renew-subscription-label" aria-describedby="renew-subscription-description">
                                                                            <!-- Enabled: "translate-x-5", Not Enabled: "translate-x-0" -->
                                                                            <span aria-hidden="true" class="translate-x-5 inline-block h-5 w-5 transform rounded-full bg-background shadow ring-0 transition duration-200 ease-in-out"></span>
                                                                        </button>
                                                                    </div>
                                                                {/if}
                                                            </div>

                                                            <div class="flex flex-1 justify-between mt-3">
                                                                <a class="text-gray-300 text-sm tracking-tight">Required</a>
                                                                {#if !match.required}
                                                                    <div class="mt-5 sm:ml-6 sm:mt-0 sm:flex sm:flex-shrink-0 sm:items-center">
                                                                        <!-- Enabled: "bg-indigo-600", Not Enabled: "bg-gray-200" -->
                                                                        <button on:click={() => match.required = true} type="button" class="bg-gray-400 relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-600 focus:ring-offset-2" role="switch" aria-checked="false" aria-labelledby="renew-subscription-label" aria-describedby="renew-subscription-description">
                                                                            <!-- Enabled: "translate-x-5", Not Enabled: "translate-x-0" -->
                                                                            <span aria-hidden="true" class="translate-x-0 inline-block h-5 w-5 transform rounded-full bg-background shadow ring-0 transition duration-200 ease-in-out"></span>
                                                                        </button>
                                                                    </div>
                                                                {:else}
                                                                    <div class="mt-5 sm:ml-6 sm:mt-0 sm:flex sm:flex-shrink-0 sm:items-center">
                                                                        <!-- Enabled: "bg-indigo-600", Not Enabled: "bg-gray-200" -->
                                                                        <button on:click={() => match.inverse = false} type="button" class="bg-indigo-600 relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-600 focus:ring-offset-2" role="switch" aria-checked="false" aria-labelledby="renew-subscription-label" aria-describedby="renew-subscription-description">
                                                                            <!-- Enabled: "translate-x-5", Not Enabled: "translate-x-0" -->
                                                                            <span aria-hidden="true" class="translate-x-5 inline-block h-5 w-5 transform rounded-full bg-background shadow ring-0 transition duration-200 ease-in-out"></span>
                                                                        </button>
                                                                    </div>
                                                                {/if}
                                                            </div>
                                                        {/if}
                                                    </div>
                                                </div>
                                            {/each}
                                        </div>

                                        <!-- input to specify one or all -->
                                        <div class="pt-6 pb-6 border-b-2 border-dashed">
                                            <a class="text-gray-300 text-sm tracking-tight">One indicates only one match has to be met, all indicates every match. The required property on each rule overrides this value</a>
                                            <OneOption bind:selected={new_rule_trigger_requirement} options={["One", "All"]} />
                                        </div>
                                    </div>
                                    <div class="pb-6 border-b-2 border-dashed">
                                        <div class="flex justify-between flex-1 items-center">
                                            <div class="justify-start">
                                                <label class="block text-xl font-medium text-slate-100 pt-6">Then...</label>
                                                <a class="text-gray-300 text-sm tracking-tight">Specify what should happen when this rule is matched</a>
                                            </div>
                                        </div>

                                        <div class="pt-6">
                                            <a class="text-gray-300 text-sm tracking-tight"><a class="text-gray-200 text-base">Choose an action type.</a> A monopoly is the only action that can happen (e.g. block the request). Trustbust options can happen concurrently of each other.</a>
                                            <OneOption bind:selected={new_rule_actiontype} options={["Monopoly", "Trustbust"]} />
                                        </div>

                                        {#if new_rule_actiontype === "Monopoly"}
                                            <div class="pt-6">
                                                <a class="text-gray-300 text-sm tracking-tight"><a class="text-gray-200 text-base">Pick a monopoly action.</a> Only one of these will occur.</a>
                                                <OneOption bind:selected={new_rule_monopoly} options={["Block"]} />
                                            </div>
                                        {:else if new_rule_actiontype === "Trustbust"}
                                            <div class="pt-6">
                                                <a class="text-gray-300 text-sm tracking-tight"><a class="text-gray-200 text-base">Pick your trustbust option(s).</a></a>
                                                <Options name={"trustbusts"} bind:chosen_data={new_rule_trustbust} options={["Smart Challenge", "Captcha", "Ratelimit", "Cache", "Redirect", "Use Origin Setting"]} />
                                            </div>
                                        {/if}

                                        {#if new_rule_trustbust.includes("Use Origin Setting")}
                                            <div class="pt-6">
                                                <a class="text-gray-300 text-sm tracking-tight"><a class="text-base">Select an origin setting</a> to send the requests to.</a>
                                                <div>
                                                    <div>
                                                        <div>
                                                            <button on:click={() => new_rule_origin_popup = !new_rule_origin_popup} type="button" class="inline-flex w-full justify-center gap-x-1.5 rounded-md bg-background px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50" id="menu-button" aria-expanded="true" aria-haspopup="true">
                                                                {#if !new_rule_backend_host}
                                                                    Set an origin
                                                                {:else}
                                                                    {new_rule_backend_host}
                                                                {/if}
                                                                {#if new_rule_origin_popup}
                                                                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                                                        <path in:draw stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                                                                    </svg>
                                                                {:else}
                                                                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                                                        <path in:draw stroke-linecap="round" stroke-linejoin="round" d="M3 4.5h14.25M3 9h9.75M3 13.5h9.75m4.5-4.5v12m0 0l-3.75-3.75M17.25 21L21 17.25" />
                                                                    </svg>
                                                                {/if}
                                                            </button>
                                                        </div>

                                                        {#if new_rule_origin_popup}
                                                            <div class="absolute z-10 mt-2 w-56 origin-top-right rounded-md bg-background shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none" role="menu" aria-orientation="vertical" aria-labelledby="menu-button" tabindex="-1">
                                                                <div class="py-1" role="none">
                                                                    {#if domain_info.origin_settings.length > 0}
                                                                        {#each domain_info.origin_settings as origin_setting}
                                                                            <button on:click={() => { new_rule_backend_host = origin_setting._id; new_rule_origin_popup = false }} class="w-full hover:bg-gray-300 duration-100 text-gray-700 block px-4 py-2 text-sm" role="menuitem" tabindex="-1" id="menu-item-0">{origin_setting._id} - origins: {formatOriginIps(origin_setting)}</button>
                                                                        {/each}
                                                                    {:else}
                                                                        <a href="/i/dash/domains/{domain_info._id}/origin_settings" target="_blank" class="w-full hover:bg-gray-300 duration-100 text-gray-700 block px-4 py-2 text-sm" role="menuitem" tabindex="-1" id="menu-item-0">no origins available - create one in a new tab</a>
                                                                    {/if}
                                                                </div>
                                                            </div>
                                                        {/if}
                                                    </div>
                                                </div>
                                            </div>
                                        {/if}

                                        {#if new_rule_trustbust.includes("Redirect")}
                                            <div class="pt-6">
                                                <a class="text-gray-300 text-sm tracking-tight"><a class="text-gray-200 text-base">Select where to send your users.</a> This should include http:// or https://. Redirection will occur after ratelimiting and challenging.</a>
                                                <input bind:value={new_rule_redirect} type="text" minlength="1" maxlength="100" class="mt-1 focus:ring-indigo-500 focus:border-indigo-500 block w-full shadow-sm sm:text-sm border-fuchsia-500 rounded-md" placeholder="https://based.ceo" required>
                                            </div>
                                        {/if}

                                        {#if new_rule_trustbust.includes("Cache")}
                                            <div class="pt-6">
                                                <a class="text-gray-300 text-sm tracking-tight"><a class="text-gray-200 text-base">Choose cache level and time-to-live.</a> A cache level of None will override general caching settings.</a>
                                                <OneOption bind:selected={new_rule_cache_settings.level} options={["None", "Standard", "IgnoreQueryString", "Aggressive"]} />
                                                <input bind:value={new_rule_cache_settings.ttl} type="number" min="5" max="604800" name="setting-name" id="rule-order" class="mt-1 focus:ring-indigo-500 focus:border-indigo-500 block w-full shadow-sm sm:text-sm border-fuchsia-500 rounded-md" placeholder="Cache time-to-live" required>
                                            </div>
                                        {/if}

                                        {#if new_rule_trustbust.includes("Ratelimit")}
                                            <div class="pt-6">
                                                <a class="text-gray-300 text-sm tracking-tight"><a class="text-base">Select a bucket</a> to use for ratelimiting.</a>
                                                <div>
                                                    <div>
                                                        <div>
                                                            <button on:click={() => new_rule_ratelimit_popup = !new_rule_ratelimit_popup} type="button" class="inline-flex w-full justify-center gap-x-1.5 rounded-md bg-background px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50" id="menu-button" aria-expanded="true" aria-haspopup="true">
                                                                {#if !new_rule_bucket}
                                                                    Set a bucket
                                                                {:else}
                                                                    {new_rule_bucket}
                                                                {/if}
                                                                {#if new_rule_ratelimit_popup}
                                                                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                                                        <path in:draw stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                                                                    </svg>
                                                                {:else}
                                                                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                                                        <path in:draw stroke-linecap="round" stroke-linejoin="round" d="M3 4.5h14.25M3 9h9.75M3 13.5h9.75m4.5-4.5v12m0 0l-3.75-3.75M17.25 21L21 17.25" />
                                                                    </svg>
                                                                {/if}
                                                            </button>
                                                        </div>

                                                        {#if new_rule_ratelimit_popup}
                                                            <div class="absolute z-10 mt-2 w-56 origin-top-right rounded-md bg-background shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none" role="menu" aria-orientation="vertical" aria-labelledby="menu-button" tabindex="-1">
                                                                <div class="py-1" role="none">
                                                                    {#if domain_info.ratelimit_buckets.length > 0}
                                                                        {#each domain_info.ratelimit_buckets as bucket}
                                                                            <button on:click={() => { new_rule_bucket = bucket._id; new_rule_ratelimit_popup = false }} class="w-full hover:bg-gray-300 duration-100 text-gray-700 block px-4 py-2 text-sm" role="menuitem" tabindex="-1" id="menu-item-0">{bucket._id} - {bucket.threshold} per {bucket.secs}</button>
                                                                        {/each}
                                                                    {:else}
                                                                        <a href="/i/dash/domains/{domain_info._id}/buckets" target="_blank" class="w-full hover:bg-gray-300 duration-100 text-gray-700 block px-4 py-2 text-sm" role="menuitem" tabindex="-1" id="menu-item-0">no buckets available - create one in a new tab</a>
                                                                    {/if}
                                                                </div>
                                                            </div>
                                                        {/if}
                                                    </div>
                                                </div>
                                            </div>
                                         {/if}
                                    </div>

                                    <div>
                                        <label class="block text-xl font-medium text-slate-100 pt-6">Finally...</label>
                                        <a class="text-gray-300 text-sm tracking-tight">Finalize the rule</a>

                                        <p>
                                            <label class="block font-medium text-slate-100 pt-6">Order</label>
                                            <a class="text-gray-300 text-sm tracking-tight">A 1-20 unique value that represents the order page rules are processed in.</a>
                                            <input bind:value={new_rule_order} type="number" min="1" max="20" name="setting-name" id="rule-order" class="mt-1 focus:ring-indigo-500 focus:border-indigo-500 block w-full shadow-sm sm:text-sm border-fuchsia-500 rounded-md" placeholder="7" required>
                                        </p>


                                        {#if new_rule_matches}
                                            <div class="flex justify-center pt-8">
                                                <button transition:fade on:click={() => (Math.random())} type="submit" class="mt-3 inline-flex w-full items-center justify-center rounded-md bg-blue-500 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-fuchsia-500 duration-100 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 sm:ml-3 sm:mt-0 sm:w-auto">Stage rule that applies {JSON.stringify(new_rule_matches)}</button>
                                            </div>
                                        {:else}
                                            <div class="flex justify-center pt-8">
                                                <p class="mt-3 inline-flex w-full items-center justify-center rounded-md text-sm font-semibold text-white shadow-sm hover:bg-red-500 duration-100 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 sm:ml-3 sm:mt-0 sm:w-auto">Fill in at least one match, one action & order to create rule</p>
                                            </div>
                                        {/if}
                                    </div>
                                    <p class="text-sm text-slate-200">
                                        <br><br>If you have any questions, our <a href="/support" class="text-indigo-500 hover:text-indigo-400">support team</a> stands by, ready to lend a hand if you have any questions. It is staffed by the same people who built Packetware.
                                        You can also check the <a href="/docs" class="text-indigo-500 hover:text-indigo-400">documentation</a> on API engine settings.
                                    </p>
                                </form>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    {/if}

    {#if loaded}
        <div transition:fade class="w-full p-5 pb-28 backdrop-blur-lg rounded-b-2xl shadow">
            <!-- <div>
                <p class="uppercase text-gray-300">at a glance</p>
                <p class="text-slate-200 text-xl">API engine has handled 239,392 requests</p>
            </div> -->
            <div class="flex items-center justify-between">
                <p class="uppercase text-gray-300">Page rules - {domain_info._id}</p>
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
                    <a href="/docs/api_engine" class="uppercase text-indigo-200 hover:text-indigo-100 py-1.5">documentation</a>
                </div>
            </div>
            <div>

                <div class="pt-5">
                    <div class="shadow bg-opacity-90 bg-amber-100">
                        <div class="px-4 py-5 sm:p-6">
                            <h3 class="text-base font-semibold leading-6 text-gray-900" id="api-engine-enabled">Page rules</h3>
                            <div class="mt-2 sm:flex sm:items-start sm:justify-between">
                                <div class="max-w-xl text-sm text-gray-500">
                                    <p id="api-engine-enabled-text">Enabling page rules will activate each rule, disabling rules will pause each rule</p>
                                </div>
                                {#if !page_rules_enabled_switch}
                                    <div class="mt-5 sm:ml-6 sm:mt-0 sm:flex sm:flex-shrink-0 sm:items-center">
                                        <!-- Enabled: "bg-indigo-600", Not Enabled: "bg-gray-200" -->
                                        <button on:click={enabledButtonClick} type="button" class="bg-gray-400 relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-600 focus:ring-offset-2" role="switch" aria-checked="false" aria-labelledby="renew-subscription-label" aria-describedby="renew-subscription-description">
                                            <!-- Enabled: "translate-x-5", Not Enabled: "translate-x-0" -->
                                            <span aria-hidden="true" class="translate-x-0 inline-block h-5 w-5 transform rounded-full bg-background shadow ring-0 transition duration-200 ease-in-out"></span>
                                        </button>
                                    </div>
                                {:else}
                                    <div class="mt-5 sm:ml-6 sm:mt-0 sm:flex sm:flex-shrink-0 sm:items-center">
                                        <!-- Enabled: "bg-indigo-600", Not Enabled: "bg-gray-200" -->
                                        <button on:click={enabledButtonClick} type="button" class="bg-indigo-600 relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-600 focus:ring-offset-2" role="switch" aria-checked="false" aria-labelledby="renew-subscription-label" aria-describedby="renew-subscription-description">
                                            <!-- Enabled: "translate-x-5", Not Enabled: "translate-x-0" -->
                                            <span aria-hidden="true" class="translate-x-5 inline-block h-5 w-5 transform rounded-full bg-background shadow ring-0 transition duration-200 ease-in-out"></span>
                                        </button>
                                    </div>
                                {/if}
                            </div>
                        </div>
                    </div>
                </div>

                <div class="h-full">
                    <div class="flex items-center justify-between py-5">
                        <p class="uppercase text-gray-300">Rules</p>
                        <div class="flex flex-1 justify-end">
                            <button on:click={() => { new_rule_popup = true }} class="bg-fuchsia-500 text-white px-3 py-1.5 mx-3 rounded text-sm">
                                + New rule
                            </button>
                        </div>
                    </div>
<!--
    order: z.number().min(0).max(50),
    trigger: z.object({
        match_type: z.array(z.object({ // multiple matching things
            trigger: oneTriggerTypeSchema.partial().withPredicate(oneDefined, "At least one variable must be defined."),
            match_type: z.enum(["Exact", "Contains", "StartsWith"]),
            inversed: z.boolean(),
            required: z.boolean()
        })), // add match type, delete match type use by _id
        trigger_requirement: z.enum(["One", "All"]),
    }),
    action: z.object({
        // one or the other TODO verify it's one or the other
        monopoly: z.enum(["Block"]).optional(),
        // smart challenge, captcha, ratelimit (with bucket name), cache (with level + ttl), redirect, origin setting
        trustbusting: z.array(z.enum(["Smart Challenge", "Captcha", "Ratelimit", "Cache", "Redirect", "Use Origin Setting"])).optional(),

        // special information TODO verify that it's here if it needs to be
        bucket_name: z.string().max(50).min(3).optional(), // TODO ensure no spaces

        cache_level: z.enum(["None", "Standard", "IgnoreQueryString", "Aggressive"]).optional(),
        cache_level_ttl: z.number().min(3).max(604000).optional(),

        redirect: z.string().min(3).max(100).optional(), // TODO regex
        backend_host: z.string().min(3).max(50).optional()
    })
});-->
                    <div class="pb-5 pt-5">
                        <p class="text-gray-900 uppercase pr-3">rules</p>
                        {#each rules as rule, rule_index}
                            <div class="pt-5">
                                <div class="shadow bg-opacity-90 bg-amber-100">
                                    <div class="px-4 py-5 sm:p-6">
                                        <div class="sm:flex sm:flex-1 sm:w-full sm:justify-between">
                                            <div class="justify-start sm:flex sm:flex-1">
                                                <h3 class="text-base font-semibold leading-6 text-gray-900 py-0.5 pr-3" id="api-strict-mode-enabled">{rule.path}</h3>
                                                {#if rule.new_staged}
                                                    <span class="inline-flex flex-shrink-0 items-center rounded-full bg-blue-50 px-1.5 py-0.5 font-medium text-blue-700 ring-1 ring-inset ring-green-600/20">staged</span>
                                                {:else if rule.existed_modified}
                                                    <span class="inline-flex flex-shrink-0 items-center rounded-full bg-yellow-50 px-1.5 py-0.5 font-medium text-yellow-700 ring-1 ring-inset ring-yellow-600/20">modified</span>
                                                {:else if rule.being_deleted}
                                                    <span class="inline-flex flex-shrink-0 items-center rounded-full bg-red-50 px-1.5 py-0.5 font-medium text-red-700 ring-1 ring-inset ring-red-600/20">staged to be deleted</span>
                                                {/if}
                                            </div>
                                            <div class="justify-end flex flex-1 items-center">
                                                <p class="mr-3 text-gray-800 text-xl px-3 border-r-2">#{rule.order}</p>
                                                <button on:click={() => { new_rule_popup = true; }} class="border-r-2 text-gray-600 pr-3 mr-3 rounded text-sm">
                                                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                                        <path stroke-linecap="round" stroke-linejoin="round" d="M16.862 4.487l1.687-1.688a1.875 1.875 0 112.652 2.652L6.832 19.82a4.5 4.5 0 01-1.897 1.13l-2.685.8.8-2.685a4.5 4.5 0 011.13-1.897L16.863 4.487zm0 0L19.5 7.125" />
                                                    </svg>
                                                </button>
                                                <button on:click={async () => deleteApiRule(rule, rule_index)} class="text-red-500 hover:text-red-700 rounded text-sm">
                                                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                                        <path stroke-linecap="round" stroke-linejoin="round" d="M14.74 9l-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 01-2.244 2.077H8.084a2.25 2.25 0 01-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 00-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 013.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 00-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 00-7.5 0" />
                                                    </svg>
                                                </button>
                                            </div>
                                        </div>
                                        <div class="mt-2 sm:flex sm:items-start sm:justify-between">
                                            <div class="max-w-xl text-sm text-gray-500">
                                                <p>Match method: {JSON.stringify(rule.match_type)}</p>
                                                <!--
                                                <p id="api-strict-mode-enabled-text">{(() => rule.http_methods.length !== 0 ? `HTTP methods: ${rule.http_methods} (query string allowed: ${(rule.allow_query_string ?? "false")})`: `No HTTP methods`)()}, {(() => rule.ws_methods.length !== 0 ? `WS methods: ${rule.ws_methods}`: `no WebSocket methods`)()}</p>
                                                {#if rule.actions.includes("Microcache")}
                                                    <p>Microcache: {rule.cache_settings.level} - {rule.cache_settings.ttl} seconds</p>
                                                {/if}
                                                {#if rule.actions.includes("Use ratelimit bucket")}
                                                    <-- iterate through bucket list to get the specs
                                                    {#each domain_info.ratelimiting_buckets as bucket}
                                                        {#if rule.bucket === bucket._id}
                                                            <p>Ratelimit bucket: {bucket._id} - {bucket.threshold} per {bucket.secs} seconds</p>
                                                        {/if}
                                                    {/each}
                                                {/if}
                                                -->
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
