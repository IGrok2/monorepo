<script>
    import { page } from '$app/stores';
    import {onMount} from "svelte";
    import { fade, fly, slide, draw } from 'svelte/transition';
    import Options from "$lib/Options.svelte";
    import Menu from "$lib/Menu.svelte";
    import OneOption from "$lib/OneOption.svelte";
    import { getCookie } from "$lib/utils/auth";

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
    let api_engine_enabled_switch;
    let api_engine_strict_mode_enabled_switch;
    // setting specific stuff
    let settings = [];
    // rule specific stuff
    let rules = [];

    // API Engine helpers
    let new_setting_popup = false;
    let new_setting_name;
    // rules
    let new_rule_popup;
    let new_rule_setting;
    let new_rule_path;
    let new_rule_match_type_popup = false;
    let new_rule_cache_popup = false;
    let new_rule_ratelimit_popup = false;
    let new_rule_match_type;
    let new_rule_order;
    let new_rule_allow_query_string;
    let new_rule_ws_methods = [];
    let new_rule_http_methods = [];
    let new_rule_actions = [];
    let new_rule_cache_settings = {
        level: null,
        ttl: null
    }; // level and ttl
    let new_rule_bucket;

    // edited rules
    let edit_rule_popup;
    let edit_rule_setting;
    let edit_rule_path;
    let edit_rule_match_type_popup = false;
    let edit_rule_cache_popup = false;
    let edit_rule_ratelimit_popup = false;
    let edit_rule_match_type;
    let edit_rule_order;
    let edit_rule_allow_query_string;
    let edit_rule_ws_methods = [];
    let edit_rule_http_methods = [];
    let edit_rule_actions = [];
    let edit_rule_cache_settings = {
        level: null,
        ttl: null
    }; // level and ttl
    let edit_rule_bucket;
    let edit_rule_index;

    const load = async () => {
        let body;

        try {
            const slug = $page.params.slug;

            const token = getCookie("jwt")

            await fetch("https://iucuwv-ip-24-189-107-128.tunnelmole.net/@/domain", {
                method: "POST",
                headers: new Headers({'content-type': 'application/json', 'Authorization': token}),
                body: JSON.stringify({
                    domain: slug
                })
            }).then(async (rawBody) => {
                body = await rawBody.json();

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

                api_engine_enabled_switch = domain_info.api_engine.enabled;
                api_engine_strict_mode_enabled_switch = domain_info.api_engine.strict_mode_enabled;
                settings = domain_info.api_engine.rule_settings;

                for (let i = 0; i < settings.length; i++) {
                    settings[i].changed = {};
                    settings[i].changed.open_api = settings[i].open_api;
                    // add rules from each setting to the rules array
                    for (let j = 0; j < settings[i].rules.length; j++) {
                        rules = [...rules, {
                            setting: settings[i]._id,
                            ...settings[i].rules[j]
                        }]
                    }
                }

                settings = settings;

                console.log(rules)

                loaded = true;
            })
        } catch (err) {
            console.log(body);

            if (body && body.message === "Email not verified") {
                document.location.href = "/i/auth/verify";

                return;
            }

            await error_notification("Error loading user profile", "Redirecting to login page ...")

            document.location.href = "/i/auth/login";
        }
    }

    onMount(load);

    const checkForStagedSetting = async () => {
        let oneYes = false;

        for (let i = 0; i < settings.length; i++) {
            let yes = false;

            if (settings[i].changed) {
                for (const [key, value] of Object.entries(settings[i].changed)) {
                    console.log(key, value)
                    if (key === "whitelist_factors") {
                        console.log("here", value)
                        // whitelist factors exists
                        if (value.new_ips && value.new_ips.length !== 0) yes = true;
                        else if (value.deleted_ips && value.deleted_ips.length !== 0) yes = true;
                    }
                    else if (key === "open_api") {
                        if (value !== settings[i].open_api) yes = true;
                    }
                    else (yes = true)
                }
            }

            if (settings[i].new_staged === true || settings[i].being_deleted === true) {
                oneYes = true;
            }

            if (yes) {
                oneYes = true;
                // organize it for the popup
                settings[i].existed_modified = true
            } else {
                settings[i].existed_modified = false;
            }
        }

        console.log(settings)

        return oneYes;
    }

    const letSaveChanges = async () => {
        if (!(await checkForStagedSetting()) && !(await checkForStagedRules())
            && api_engine_enabled_switch === domain_info.api_engine.enabled &&
            api_engine_strict_mode_enabled_switch === domain_info.api_engine.strict_mode_enabled) {

            saved_class = "cursor-not-allowed bg-gray-400";
            able_to_save = false

        } else {
            saved_class = "bg-fuchsia-500";
            able_to_save = true;
        }
    }

    // check if there are any staged or new rules
    const checkForStagedRules = async () => {
        for (const rule of rules) {
            if (rule.new_staged === true || rule.existed_modified === true || rule.being_deleted) {
                return true;
            }
        }

        return false;
    }

    // for enable button
    const enabledButtonClick = async () => {
        api_engine_enabled_switch = !api_engine_enabled_switch
        await letSaveChanges()
    }

    // for strict mode button
    const strictModeButtonClick = async () => {
        api_engine_strict_mode_enabled_switch = !api_engine_strict_mode_enabled_switch
        await letSaveChanges()
    }

    const getRulesFromSetting = (setting) => {
        const setting_name = setting._id;

        let setting_rules = [];

        for (let i = 0; i < rules.length; i++) {
            let rule = rules[i];

            if (rule.setting === setting_name) {
                setting_rules.push(rule)
            }
        }

        return setting_rules
    }

    const newEngineSetting = async () => {
        if (new_setting_name.includes(domain_info._id)) {

            for (const setting of settings) {
                if (setting._id === new_setting_name) {
                    await error_notification("Couldn't stage new API Engine setting", "Setting with that name already exists")
                    return;
                }
            }

            new_setting_popup = false;
            settings = [...settings, {
                _id: new_setting_name,
                changed: {},
                open_api: false,
                whitelist_factor_bind: "",
                whitelist_factors: {
                    ips: [],
                },

                new_staged: true,
            }]
            await letSaveChanges()
        } else {
            await error_notification("Couldn't stage new API Engine setting", "Didn't include the root domain in the setting")
        }
    }

    // sort API engine rules by order
    const sortRules = async () => {
        rules.sort((a, b) => {
            return a.order - b.order;
        })
    }

    const newApiRule = async () => {
        if (new_rule_path.includes("/")) {
            for (const rule of rules) {
                if (rule.path === new_rule_path && rule.setting === new_rule_setting) {
                    await error_notification("Couldn't stage new API Engine rule", "Rule with that path already exists")
                    return;
                }
            }

            new_rule_popup = false;

            rules = [...rules, {
                new_staged: true,
                existed_modified: false,
                being_deleted: false,
                setting: new_rule_setting,
                path: new_rule_path,
                match_type: new_rule_match_type,
                order: new_rule_order,
                allow_query_string: new_rule_allow_query_string,
                ws_methods: new_rule_ws_methods,
                http_methods: new_rule_http_methods,
                actions: new_rule_actions,
                cache_settings: new_rule_cache_settings,
                bucket: new_rule_bucket
            }]

            await sortRules()

            await letSaveChanges()
        } else {
            await error_notification("Couldn't stage new API Engine rule", "Didn't include any paths")
        }
    }

    // update api engine rule
    const editApiRule = async (rule_index) => {
        if (edit_rule_path.includes("/")) {
            for (const rule of rules) {
                if (rule.path === edit_rule_path && rule.setting === edit_rule_setting && rule_index !== edit_rule_index) {
                    await error_notification("Couldn't stage new API Engine rule", "Rule with that path already exists")
                    return;
                }
            }

            const old_rule = rules[edit_rule_index];

            let existed_modified = false;
            let new_staged = false;

            // check if the rule was changed, or if it's just staged
            if (old_rule.new_staged) {
                new_staged = true;
            } else {
                existed_modified = true;
            }

            // apply changes by overriding all properties
            rules[edit_rule_index] = {
                new_staged,
                existed_modified,
                being_deleted: false,
                setting: edit_rule_setting,
                path: edit_rule_path,
                match_type: edit_rule_match_type,
                order: edit_rule_order,
                allow_query_string: edit_rule_allow_query_string,
                ws_methods: edit_rule_ws_methods,
                http_methods: edit_rule_http_methods,
                actions: edit_rule_actions,
                cache_settings: edit_rule_cache_settings,
                bucket: edit_rule_bucket
            };

            rules = rules;

            await sortRules()

            edit_rule_popup = false;

            await letSaveChanges()
        } else {
            await error_notification("Couldn't stage new API Engine rule", "Didn't include any paths")
        }
    }

    const getChangedApiRulesFromSetting = (setting_name) => {
        let changed_rules = [];

        for (let i = 0; i < rules.length; i++) {
            let rule = rules[i];

            if (rule.setting === setting_name) {
                if (rule.new_staged || rule.existed_modified || rule.being_deleted) {
                    changed_rules.push(rule)
                }
            }
        }

        return changed_rules
    }

    const deleteApiRule = async (rule, index) => {
        await confirmChange(`Delete ${rule.new_staged ? `staged` : `existing`} API engine rule?`, `${rule.new_staged ? "Because this rule was staged, nothing will change in your production environment" : "This rule will be staged for deletion"}`,
            () => {
                if (rule.new_staged) {
                    rules.splice(index, 1);
                    rules = rules;
                } else {
                    rule.being_deleted = true;
                }
        })
    }

    const settingOpenApi = async (index) => {
        const baller = settings;

        if (!baller[index].changed) baller[index].changed = {};

        baller[index].changed.open_api = !baller[index].changed.open_api;

        if (!baller[index].new_staged) {
            baller.existed_modified = true;
        }

        // TODO check why open_api is enabled / disabled and change is still staged

        await letSaveChanges();

        settings = baller;
    }

    // get new settings
    const getNewSettings = () => {
        let new_settings = [];

        for (let i = 0; i < settings.length; i++) {
            let setting = settings[i];

            if (setting.new_staged === true) {
                new_settings.push(setting)
            }
        }

        return new_settings
    }

    // get all changed settings and a description of what's been changed
    const getChangedSettings = () => {
        let changed_settings = [];

        for (let i = 0; i < settings.length; i++) {
            let setting = settings[i];

            if (setting.existed_modified === true) {
                changed_settings.push(setting)
            }
        }

        return changed_settings
    }

    // get deleted settings
    const getDeletedSettings = () => {
        let deleted_settings = [];

        for (let i = 0; i < settings.length; i++) {
            let setting = settings[i];

            if (setting.being_deleted === true) {
                deleted_settings.push(setting)
            }
        }

        return deleted_settings
    }

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


    const saveChanges = async () => {
        if (able_to_save === true) {
            saving = true;
            save_button_success = true;
            try {
                const slug = $page.params.slug;

                const token = getCookie("jwt");

                (async () => {
                    if (settings.length !== 0) {
                        for (let i = 0; i < settings.length; i++) {
                            if (settings[i].new_staged) {
                                settings[i].new_staged = undefined;
                                settings[i].existed_modified = undefined;
                                settings[i].being_deleted = undefined;

                                await fetch("https://iucuwv-ip-24-189-107-128.tunnelmole.net/@/domain/api-engine/add-setting", {
                                    method: "POST",
                                    headers: new Headers({'content-type': 'application/json', 'Authorization': token}),
                                    body: JSON.stringify({
                                        domain: slug,
                                        _id: settings[i]._id,
                                        open_api: settings[i].open_api
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
                            } else if (settings[i].being_deleted) {
                                settings[i].new_staged = undefined;
                                settings[i].existed_modified = undefined;
                                settings[i].being_deleted = undefined;

                                await fetch("https://iucuwv-ip-24-189-107-128.tunnelmole.net/@/domain/api-engine/delete-setting", {
                                    method: "POST",
                                    headers: new Headers({'content-type': 'application/json', 'Authorization': token}),
                                    body: JSON.stringify({
                                        domain: slug,
                                        setting_id: settings[i]._id,
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

                            } else if (settings[i].existed_modified && settings[i].changed) {
                                settings[i].new_staged = undefined;
                                settings[i].existed_modified = undefined;
                                settings[i].being_deleted = undefined;

                                const c = settings[i].changed;

                                console.log("here like actually", c);

                                if (c.whitelist_factors && (c.whitelist_factors.new_ips || c.whitelist_factors.deleted_ips)) {
                                    console.log("wtf");

                                    await fetch("https://iucuwv-ip-24-189-107-128.tunnelmole.net/@/domain/api-engine/setting/update-whitelistfactors", {
                                        method: "POST",
                                        headers: new Headers({'content-type': 'application/json', 'Authorization': token}),
                                        body: JSON.stringify({
                                            domain: slug,
                                            setting_id: settings[i]._id,
                                            whitelist_factors: {
                                                ips: settings[i].whitelist_factors.ips
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

                                if (c.open_api !== settings[i].open_api) {
                                    await fetch("https://iucuwv-ip-24-189-107-128.tunnelmole.net/@/domain/api-engine/update-setting", {
                                        method: "POST",
                                        headers: new Headers({'content-type': 'application/json', 'Authorization': token}),
                                        body: JSON.stringify({
                                            domain: slug,
                                            _id: settings[i]._id,
                                            open_api: c.open_api
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

                    if (api_engine_enabled_switch !== domain_info.api_engine.enabled || api_engine_strict_mode_enabled_switch !== domain_info.api_engine.strict_mode_enabled) {
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
                    await load();
                    await notification("Updated settings", "Pushed changes to edge")
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

    // transfer data from the rule to the edit_ variables for the popup
    const transferRuleInfo = (rule_index) => {
        const r = rules[rule_index];

        edit_rule_setting = r.setting;
        edit_rule_path = r.path;
        edit_rule_match_type = r.match_type;
        edit_rule_order = r.order;
        edit_rule_allow_query_string = r.allow_query_string;
        edit_rule_ws_methods = r.ws_methods;
        edit_rule_http_methods = r.http_methods;
        edit_rule_actions = r.actions;
        edit_rule_cache_settings = r.cache_settings;
        edit_rule_bucket = r.bucket;
        edit_rule_index = rule_index;
    }

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
                                    {#if api_engine_enabled_switch !== domain_info.api_engine.enabled}
                                        {#if api_engine_enabled_switch === true}
                                            <li><a class="text-green-500">Enable</a> API engine (settings and their rules will be activated)</li>
                                        {:else}
                                            <li><a class="text-red-500">Disable</a> API engine (all rules and settings will be deactivated)</li>
                                        {/if}
                                    {/if}

                                    {#if api_engine_strict_mode_enabled_switch !== domain_info.api_engine.strict_mode_enabled}
                                        {#if api_engine_strict_mode_enabled_switch === true}
                                            <li><a class="text-green-500">Enable</a> strict mode (POST requests MUST go through the API engine and GET or OPTION HTTP request bodies are discarded)</li>
                                        {:else}
                                            <li><a class="text-red-500">Disable</a> strict mode (POST requests WON'T have to go through the API engine and all HTTP request bodies WILL be streamed)</li>
                                        {/if}
                                    {/if}

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

                                    {#if getDeletedSettings().length !== 0}
                                        <li><a class="text-red-500">Delete</a> API engine settings:</li>
                                        <ol class="list-disc pl-6">
                                            {#each getDeletedSettings() as deleted_setting}
                                                <li class="font-light">{deleted_setting._id}
                                                    {#if getRulesFromSetting(deleted_setting._id).length !== 0}
                                                        <div class="pl-8">
                                                            <p class="text-gray-700">These rules will also be deleted</p>
                                                            <ol class="pl-10 list-desc text-gray-500">
                                                                {#each getRulesFromSetting(deleted_setting._id) as rule}
                                                                    <li>{rule.path} ({rule.match_type.toLowerCase()}) - methods: {(() => rule.http_methods.length !== 0 ? `HTTP methods: ${rule.http_methods} (query string allowed: ${(rule.allow_query_string ?? "false")})`: `no HTTP methods`)()}, {(() => rule.ws_methods.length !== 0 ? `WS methods: ${rule.ws_methods}`: `no WebSocket methods`)()}{rule.actions.length !== 0 ? `, ${rule.actions}` : ``}<br></li>
                                                                {/each}
                                                            </ol>
                                                        </div>
                                                    {/if}
                                                </li>
                                            {/each}
                                        </ol>
                                    {/if}

                                    {#if getNewRules().length !== 0}
                                        <li><a class="text-green-500">New</a> API engine rules:</li>
                                        <ol class="list-disc pl-6">
                                            {#each getNewRules() as new_rule}
                                                <li class="font-light">{new_rule._id}
                                                    <p>Match method: {new_rule.match_type}</p>
                                                    <p id="api-strict-mode-enabled-text">{(() => new_rule.http_methods.length !== 0 ? `HTTP methods: ${new_rule.http_methods} (query string allowed: ${(new_rule.allow_query_string ?? "false")})`: `No HTTP methods`)()}, {(() => new_rule.ws_methods.length !== 0 ? `WS methods: ${new_rule.ws_methods}`: `no WebSocket methods`)()}</p>
                                                    {#if new_rule.actions.includes("Microcache")}
                                                        <p>Microcache: {new_rule.cache_settings.level} - {new_rule.cache_settings.ttl} seconds</p>
                                                    {/if}
                                                    {#if new_rule.actions.includes("Use ratelimit bucket")}
                                                        <!-- iterate through bucket list to get the specs -->
                                                        {#each domain_info.ratelimit_buckets as bucket}
                                                            {#if new_rule.bucket === bucket._id}
                                                                <p>Ratelimit bucket: {bucket._id} - {bucket.threshold} per {bucket.secs} seconds</p>
                                                            {/if}
                                                        {/each}
                                                    {/if}
                                                </li>
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
    <Menu api_engine={true} api_engine_class={api_engine} page_rules_class={page_rules} optimization_class={cache_engine} bot_engine_class={bot_engine} />

    {#if new_setting_popup}
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
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M21 7.5l-2.25-1.313M21 7.5v2.25m0-2.25l-2.25 1.313M3 7.5l2.25-1.313M3 7.5l2.25 1.313M3 7.5v2.25m9 3l2.25-1.313M12 12.75l-2.25-1.313M12 12.75V15m0 6.75l2.25-1.313M12 21.75V19.5m0 2.25l-2.25-1.313m0-16.875L12 2.25l2.25 1.313M21 14.25v2.25l-2.25 1.313m-13.5 0L3 16.5v-2.25" />
                                            </svg>
                                        </div>
                                        <h2 class="font-semibold leading-6 text-slate-100 text-xl" id="slide-over-title">New API Engine setting</h2>
                                        <div class="ml-3 flex h-7 items-center">
                                            <button on:click={() => { new_setting_popup = false }} type="button" class="rounded-md text-slate-100 hover:text-slate-400 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2">
                                                <span class="sr-only">Close panel</span>
                                                <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                                                    <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                                                </svg>
                                            </button>
                                        </div>
                                    </div>
                                </div>
                                <form class="relative mt-6 flex-1 px-4 sm:px-6" on:submit|preventDefault={newEngineSetting}>
                                    <p>
                                        <label for="setting-name" class="block font-medium text-slate-100">Host and (optional) path</label>
                                        <input bind:value={new_setting_name} type="text" name="setting-name" id="setting-name" class="mt-1 focus:ring-indigo-500 focus:border-indigo-500 block w-full shadow-sm sm:text-sm border-fuchsia-500 rounded-md" placeholder="api.based.ceo or based.ceo/api">
                                    </p>
                                    <p class="pt-3 text-sm text-slate-200">
                                        Creating an API Engine setting is the best way to protect your API. You can create multiple settings, if you have multiple APIs.
                                        <br><br>API settings contain a batch of rules that run under this host and path. For example, if you create a setting for <a class="underline italic">`api.based.ceo`</a>, then API engine will apply to <a class="font-bold">all paths under `api.based.ceo`</a>.
                                        It's also flexible, so you can have API engine run under a path like <a class="underline italic">`based.ceo/api`</a>. API engine will then stamp all paths under this host and path, and will only adhere to allow traffic specifically whitelisted with an API engine rule.
                                        <br><br>Rules are specific: you can use them to only allow certain HTTP and WebSocket methods, apply ratelimiting specifically, check the bodies of POST and WebSocket requests, microcache public endpoints, and ensure proof-of-work for paths you don't wish for bots to access.
                                        <br><br><a class="italic">By staging this change, it doesn't occur until you finalize and save changes.</a>
                                    </p>
                                    {#if new_setting_name}
                                    <div class="flex justify-center pt-8">
                                            <button transition:fade type="submit" class="mt-3 inline-flex w-full items-center justify-center rounded-md bg-blue-500 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-fuchsia-500 duration-100 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 sm:ml-3 sm:mt-0 sm:w-auto">Stage setting {new_setting_name}</button>
                                    </div>
                                    {/if}
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
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M21 7.5l-2.25-1.313M21 7.5v2.25m0-2.25l-2.25 1.313M3 7.5l2.25-1.313M3 7.5l2.25 1.313M3 7.5v2.25m9 3l2.25-1.313M12 12.75l-2.25-1.313M12 12.75V15m0 6.75l2.25-1.313M12 21.75V19.5m0 2.25l-2.25-1.313m0-16.875L12 2.25l2.25 1.313M21 14.25v2.25l-2.25 1.313m-13.5 0L3 16.5v-2.25" />
                                            </svg>
                                        </div>
                                        <h2 class="font-semibold leading-6 text-slate-100 text-xl" id="slide-over-title">Create an API Engine rule</h2>
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
                                <form class="relative mt-6 flex-1 px-4 sm:px-6" on:submit|preventDefault={newApiRule}>
                                    <p>
                                        <label for="rule-path" class="block font-medium text-slate-100">Specific API path</label>
                                        <a class="text-gray-300 text-sm tracking-tight">The path to invoke this rule</a>
                                        <input bind:value={new_rule_path} type="text" name="setting-name" id="rule-path" class="mt-1 focus:ring-indigo-500 focus:border-indigo-500 block w-full shadow-sm sm:text-sm border-fuchsia-500 rounded-md" placeholder="/auth/login or /auth/*" required>
                                    </p>

                                    <div>
                                        <label for="rule-path" class="block font-medium text-slate-100 pt-6">Match type</label>
                                        <a class="text-gray-300 text-sm tracking-tight">How to match the API path</a>
                                        <div>
                                            <div>
                                                <button on:click={() => new_rule_match_type_popup = !new_rule_match_type_popup} type="button" class="inline-flex w-full justify-center gap-x-1.5 rounded-md bg-background px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50" id="menu-button" aria-expanded="true" aria-haspopup="true">
                                                    {#if !new_rule_match_type}
                                                        Set a match type
                                                    {:else}
                                                        {new_rule_match_type}
                                                    {/if}
                                                    {#if new_rule_match_type_popup}
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

                                            {#if new_rule_match_type_popup}
                                                <div class="absolute z-10 mt-2 w-56 origin-top-right rounded-md bg-background shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none" role="menu" aria-orientation="vertical" aria-labelledby="menu-button" tabindex="-1">
                                                    <div class="py-1" role="none">
                                                        <!-- Active: "bg-gray-100 text-gray-900", Not Active: "text-gray-700" -->
                                                        <button on:click={() => { new_rule_match_type = "Exact"; new_rule_match_type_popup = false }} class="w-full hover:bg-gray-300 duration-100 text-gray-700 block px-4 py-2 text-sm" role="menuitem" tabindex="-1" id="menu-item-0">Exact</button>
                                                        <button on:click={() => { new_rule_match_type = "Contains"; new_rule_match_type_popup = false }} class="w-full hover:bg-gray-300 duration-100 text-gray-700 block px-4 py-2 text-sm" role="menuitem" tabindex="-1" id="menu-item-1">Contains</button>
                                                        <button on:click={() => { new_rule_match_type = "StartsWith"; new_rule_match_type_popup = false }} class="w-full hover:bg-gray-300 duration-100 text-gray-700 block px-4 py-2 text-sm" role="menuitem" tabindex="-1" id="menu-item-2">StartsWith</button>
                                                        <button on:click={() => { new_rule_match_type = "UseStar"; new_rule_match_type_popup = false }} class="w-full hover:bg-gray-300 duration-100 text-gray-700 block px-4 py-2 text-sm" role="menuitem" tabindex="-1" id="menu-item-3">UseStar</button>
                                                    </div>
                                                </div>
                                            {/if}
                                        </div>
                                    </div>

                                    <p>
                                        <label for="rule-path" class="block font-medium text-slate-100 pt-6">Order</label>
                                        <a class="text-gray-300 text-sm tracking-tight">A 1-20 value that represents the order API rules are processed in.</a>
                                        <input bind:value={new_rule_order} type="number" min="1" max="20" name="setting-name" id="rule-order" class="mt-1 focus:ring-indigo-500 focus:border-indigo-500 block w-full shadow-sm sm:text-sm border-fuchsia-500 rounded-md" placeholder="7" required>
                                    </p>

                                    <div>
                                        <label class="block font-medium text-slate-100 pt-6">HTTP methods allowed</label>
                                        <a class="text-gray-300 text-sm tracking-tight">HTTP methods to be allowed for this endpoint</a>
                                        <Options name={"HTTP methods"} options={["GET", "POST", "DELETE", "OPTIONS"]} bind:chosen_data={new_rule_http_methods} />
                                    </div>

                                    <div>
                                        <label class="block font-medium text-slate-100 pt-6">WebSocket methods allowed</label>
                                        <a class="text-gray-300 text-sm tracking-tight">WebSocket methods to be allowed for this endpoint. If no methods are set, WebSockets will not be allowed to connect.</a>
                                        <Options name={"WebSocket methods"} options={["TEXT", "BINARY", "PING", "CLOSE"]} bind:chosen_data={new_rule_ws_methods} />
                                    </div>

                                    <div class="flex flex-1">
                                        <div>
                                            <label class="block font-medium text-slate-100 pt-6">Allow query string</label>
                                            <a class="text-gray-300 text-sm tracking-tight">Allow query string in requests, such as /public/stats<a class="underline">?query=500</a></a>
                                        </div>
                                        {#if !new_rule_allow_query_string}
                                            <div class="mt-5 sm:ml-6 sm:mt-0 sm:flex sm:flex-shrink-0 sm:items-center">
                                                <!-- Enabled: "bg-indigo-600", Not Enabled: "bg-gray-200" -->
                                                <button on:click={async () => new_rule_allow_query_string = true} type="button" class="bg-gray-400 relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-600 focus:ring-offset-2" role="switch" aria-checked="false" aria-labelledby="renew-subscription-label" aria-describedby="renew-subscription-description">
                                                    <!-- Enabled: "translate-x-5", Not Enabled: "translate-x-0" -->
                                                    <span aria-hidden="true" class="translate-x-0 inline-block h-5 w-5 transform rounded-full bg-background shadow ring-0 transition duration-200 ease-in-out"></span>
                                                </button>
                                            </div>
                                        {:else}
                                            <div class="mt-5 sm:ml-6 sm:mt-0 sm:flex sm:flex-shrink-0 sm:items-center">
                                                <!-- Enabled: "bg-indigo-600", Not Enabled: "bg-gray-200" -->
                                                <button on:click={async () => new_rule_allow_query_string = false} type="button" class="bg-indigo-600 relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-600 focus:ring-offset-2" role="switch" aria-checked="false" aria-labelledby="renew-subscription-label" aria-describedby="renew-subscription-description">
                                                    <!-- Enabled: "translate-x-5", Not Enabled: "translate-x-0" -->
                                                    <span aria-hidden="true" class="translate-x-5 inline-block h-5 w-5 transform rounded-full bg-background shadow ring-0 transition duration-200 ease-in-out"></span>
                                                </button>
                                            </div>
                                        {/if}
                                    </div>

                                    <div>
                                        <label class="block font-medium text-slate-100 pt-6">Actions</label>
                                        <a class="text-gray-300 text-sm tracking-tight">Optional and configurable actions</a>
                                        <Options name={"actions"} options={["Microcache", "Use ratelimit bucket"]} bind:chosen_data={new_rule_actions} />
                                    </div>

                                    {#if new_rule_actions.includes("Microcache")}
                                        <div>
                                            <label class="block font-medium text-slate-100 pt-6">Caching settings</label>
                                            <a class="text-gray-300 text-sm tracking-tight">For microcaching public endpoints on your API</a>
                                            <div>
                                                <div>
                                                    <button on:click={() => new_rule_cache_popup = !new_rule_cache_popup} type="button" class="inline-flex w-full justify-center gap-x-1.5 rounded-md bg-background px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50" id="menu-button" aria-expanded="true" aria-haspopup="true">
                                                        {#if !new_rule_cache_settings.level}
                                                            Set a cache level
                                                        {:else}
                                                            {new_rule_cache_settings.level}
                                                        {/if}
                                                        {#if new_rule_cache_popup}
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

                                                {#if new_rule_cache_popup}
                                                    <div class="absolute z-10 mt-2 w-56 origin-top-right rounded-md bg-background shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none" role="menu" aria-orientation="vertical" aria-labelledby="menu-button" tabindex="-1">
                                                        <div class="py-1" role="none">
                                                            <button on:click={() => { new_rule_cache_settings.level = "None"; new_rule_cache_popup = false }} class="w-full hover:bg-gray-300 duration-100 text-gray-700 block px-4 py-2 text-sm" role="menuitem" tabindex="-1" id="menu-item-0">None</button>
                                                            <button on:click={() => { new_rule_cache_settings.level = "Standard"; new_rule_cache_popup = false }} class="w-full hover:bg-gray-300 duration-100 text-gray-700 block px-4 py-2 text-sm" role="menuitem" tabindex="-1" id="menu-item-1">Standard</button>
                                                            <button on:click={() => { new_rule_cache_settings.level = "IgnoreQueryString"; new_rule_cache_popup = false }} class="w-full hover:bg-gray-300 duration-100 text-gray-700 block px-4 py-2 text-sm" role="menuitem" tabindex="-1" id="menu-item-2">Standard - ignore query string</button>
                                                            <button on:click={() => { new_rule_cache_settings.level = "Aggressive"; new_rule_cache_popup = false }} class="w-full hover:bg-gray-300 duration-100 text-gray-700 block px-4 py-2 text-sm" role="menuitem" tabindex="-1" id="menu-item-3">Aggressive</button>
                                                        </div>
                                                    </div>
                                                {/if}
                                            </div>
                                            <input bind:value={new_rule_cache_settings.ttl} type="number" min="5" max="604800" name="setting-name" id="rule-order" class="mt-1 focus:ring-indigo-500 focus:border-indigo-500 block w-full shadow-sm sm:text-sm border-fuchsia-500 rounded-md" placeholder="Cache time-to-live" required>
                                        </div>
                                    {/if}

                                    {#if new_rule_actions.includes("Use ratelimit bucket")}
                                        <div>
                                            <label class="block font-medium text-slate-100 pt-6">Ratelimiting</label>
                                            <a class="text-gray-300 text-sm tracking-tight">Use buckets to ratelimit IPs against this endpoint</a>
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


                                    {#if new_rule_path && new_rule_order && new_rule_match_type}
                                        <div class="flex justify-center pt-8">
                                            <button transition:fade type="submit" class="mt-3 inline-flex w-full items-center justify-center rounded-md bg-blue-500 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-fuchsia-500 duration-100 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 sm:ml-3 sm:mt-0 sm:w-auto">Stage rule that applies on {new_rule_setting}{new_rule_path}</button>
                                        </div>
                                        {:else}
                                        <div class="flex justify-center pt-8">
                                            <p class="mt-3 inline-flex w-full items-center justify-center rounded-md text-sm font-semibold text-white shadow-sm hover:bg-red-500 duration-100 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 sm:ml-3 sm:mt-0 sm:w-auto">Fill in at least path, match type & order to create rule</p>
                                        </div>
                                    {/if}
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

    {#if edit_rule_popup}
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
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M21 7.5l-2.25-1.313M21 7.5v2.25m0-2.25l-2.25 1.313M3 7.5l2.25-1.313M3 7.5l2.25 1.313M3 7.5v2.25m9 3l2.25-1.313M12 12.75l-2.25-1.313M12 12.75V15m0 6.75l2.25-1.313M12 21.75V19.5m0 2.25l-2.25-1.313m0-16.875L12 2.25l2.25 1.313M21 14.25v2.25l-2.25 1.313m-13.5 0L3 16.5v-2.25" />
                                            </svg>
                                        </div>
                                        <h2 class="font-semibold leading-6 text-slate-100 text-xl" id="slide-over-title">Modify API engine rule</h2>
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
                                <form class="relative mt-6 flex-1 px-4 sm:px-6" on:submit|preventDefault={() => { editApiRule(edit_rule_index) }}>
                                    <p>
                                        <label for="rule-path" class="block font-medium text-slate-100">Specific API path</label>
                                        <a class="text-gray-300 text-sm tracking-tight">The path to invoke this rule</a>
                                        <input bind:value={edit_rule_path} type="text" name="setting-name" id="rule-path" class="mt-1 focus:ring-indigo-500 focus:border-indigo-500 block w-full shadow-sm sm:text-sm border-fuchsia-500 rounded-md" placeholder="/auth/login or /auth/*" required>
                                    </p>

                                    <div>
                                        <label for="rule-path" class="block font-medium text-slate-100 pt-6">Match type</label>
                                        <a class="text-gray-300 text-sm tracking-tight">How to match the API path</a>
                                        <OneOption name={"match type"} options={["Exact", "Contains", "StartsWith", "UseStar"]} bind:selected={edit_rule_match_type} />
                                    </div>

                                    <p>
                                        <label for="rule-path" class="block font-medium text-slate-100 pt-6">Order</label>
                                        <a class="text-gray-300 text-sm tracking-tight">A 1-20 value that represents the order API rules are processed in.</a>
                                        <input bind:value={edit_rule_order} type="number" min="1" max="20" name="setting-name" id="rule-order" class="mt-1 focus:ring-indigo-500 focus:border-indigo-500 block w-full shadow-sm sm:text-sm border-fuchsia-500 rounded-md" placeholder="7" required>
                                    </p>

                                    <div>
                                        <label class="block font-medium text-slate-100 pt-6">HTTP methods allowed</label>
                                        <a class="text-gray-300 text-sm tracking-tight">HTTP methods to be allowed for this endpoint</a>
                                        <Options name={"HTTP methods"} options={["GET", "POST", "DELETE", "OPTIONS"]} bind:chosen_data={edit_rule_http_methods} />
                                    </div>

                                    <div>
                                        <label class="block font-medium text-slate-100 pt-6">WebSocket methods allowed</label>
                                        <a class="text-gray-300 text-sm tracking-tight">WebSocket methods to be allowed for this endpoint. If no methods are set, WebSockets will not be allowed to connect.</a>
                                        <Options name={"WebSocket methods"} options={["TEXT", "BINARY", "PING", "CLOSE"]} bind:chosen_data={edit_rule_ws_methods} />
                                    </div>

                                    <div class="flex flex-1">
                                        <div>
                                            <label class="block font-medium text-slate-100 pt-6">Allow query string</label>
                                            <a class="text-gray-300 text-sm tracking-tight">Allow query string in requests, such as /public/stats<a class="underline">?query=500</a></a>
                                        </div>
                                        {#if !edit_rule_allow_query_string}
                                            <div class="mt-5 sm:ml-6 sm:mt-0 sm:flex sm:flex-shrink-0 sm:items-center">
                                                <!-- Enabled: "bg-indigo-600", Not Enabled: "bg-gray-200" -->
                                                <button on:click={async () => edit_rule_allow_query_string = true} type="button" class="bg-gray-400 relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-600 focus:ring-offset-2" role="switch" aria-checked="false" aria-labelledby="renew-subscription-label" aria-describedby="renew-subscription-description">
                                                    <!-- Enabled: "translate-x-5", Not Enabled: "translate-x-0" -->
                                                    <span aria-hidden="true" class="translate-x-0 inline-block h-5 w-5 transform rounded-full bg-background shadow ring-0 transition duration-200 ease-in-out"></span>
                                                </button>
                                            </div>
                                        {:else}
                                            <div class="mt-5 sm:ml-6 sm:mt-0 sm:flex sm:flex-shrink-0 sm:items-center">
                                                <!-- Enabled: "bg-indigo-600", Not Enabled: "bg-gray-200" -->
                                                <button on:click={async () => edit_rule_allow_query_string = false} type="button" class="bg-indigo-600 relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-600 focus:ring-offset-2" role="switch" aria-checked="false" aria-labelledby="renew-subscription-label" aria-describedby="renew-subscription-description">
                                                    <!-- Enabled: "translate-x-5", Not Enabled: "translate-x-0" -->
                                                    <span aria-hidden="true" class="translate-x-5 inline-block h-5 w-5 transform rounded-full bg-background shadow ring-0 transition duration-200 ease-in-out"></span>
                                                </button>
                                            </div>
                                        {/if}
                                    </div>

                                    <div>
                                        <label class="block font-medium text-slate-100 pt-6">Actions</label>
                                        <a class="text-gray-300 text-sm tracking-tight">Optional and configurable actions</a>
                                        <Options name={"actions"} options={["Microcache", "Use ratelimit bucket"]} bind:chosen_data={edit_rule_actions} />
                                    </div>

                                    {#if edit_rule_actions.includes("Microcache")}
                                        <div>
                                            <label class="block font-medium text-slate-100 pt-6">Caching settings</label>
                                            <a class="text-gray-300 text-sm tracking-tight">For microcaching public endpoints on your API</a>
                                            <OneOption bind:selected={edit_rule_cache_settings.level} options={["None", "Standard", "IgnoreQueryString", "Aggressive"]} />
                                            <input bind:value={edit_rule_cache_settings.ttl} type="number" min="5" max="604800" name="setting-name" id="rule-order" class="mt-1 focus:ring-indigo-500 focus:border-indigo-500 block w-full shadow-sm sm:text-sm border-fuchsia-500 rounded-md" placeholder="Cache time-to-live" required>
                                        </div>
                                    {/if}

                                    {#if edit_rule_actions.includes("Use ratelimit bucket")}
                                        <div>
                                            <label class="block font-medium text-slate-100 pt-6">Ratelimiting</label>
                                            <a class="text-gray-300 text-sm tracking-tight">Use buckets to ratelimit IPs against this endpoint</a>
                                            <div>
                                                <div>
                                                    <div>
                                                        <button on:click={() => edit_rule_ratelimit_popup = !edit_rule_ratelimit_popup} type="button" class="inline-flex w-full justify-center gap-x-1.5 rounded-md bg-background px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50" id="menu-button" aria-expanded="true" aria-haspopup="true">
                                                            {#if !edit_rule_bucket}
                                                                Set a bucket
                                                            {:else}
                                                                {edit_rule_bucket}
                                                            {/if}
                                                            {#if edit_rule_ratelimit_popup}
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

                                                    {#if edit_rule_ratelimit_popup}
                                                        <div class="absolute z-10 mt-2 w-56 origin-top-right rounded-md bg-background shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none" role="menu" aria-orientation="vertical" aria-labelledby="menu-button" tabindex="-1">
                                                            <div class="py-1" role="none">
                                                                {#if domain_info.ratelimit_buckets.length > 0}
                                                                    {#each domain_info.ratelimit_buckets as bucket}
                                                                        <button on:click={() => { edit_rule_bucket = bucket._id; edit_rule_ratelimit_popup = false }} class="w-full hover:bg-gray-300 duration-100 text-gray-700 block px-4 py-2 text-sm" role="menuitem" tabindex="-1" id="menu-item-0">{bucket._id} - {bucket.threshold} per {bucket.secs}</button>
                                                                    {/each}
                                                                {:else}
                                                                    <a href="/i/dash/domains/{domain_info._id}/buckets" target="_blank" class="w-full hover:bg-gray-300 duration-100 text-gray-700 block px-4 py-2 text-sm" role="menuitem" tabindex="-1" id="menu-item-0">No buckets available - create one in a new tab?</a>
                                                                {/if}
                                                            </div>
                                                        </div>
                                                    {/if}
                                                </div>
                                            </div>
                                        </div>
                                    {/if}


                                    {#if edit_rule_path && edit_rule_order && edit_rule_match_type}
                                        <div class="flex justify-center pt-8">
                                            <button transition:fade type="submit" class="mt-3 inline-flex w-full items-center justify-center rounded-md bg-blue-500 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-fuchsia-500 duration-100 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 sm:ml-3 sm:mt-0 sm:w-auto">Edit {rules[edit_rule_index].new_staged ? `staged` : `<a class="font-bold">existing</a>`} rule that is applicable to {new_rule_setting}{new_rule_path}</button>
                                        </div>
                                    {:else}
                                        <div class="flex justify-center pt-8">
                                            <p class="mt-3 inline-flex w-full items-center justify-center rounded-md text-sm font-semibold text-white shadow-sm hover:bg-red-500 duration-100 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 sm:ml-3 sm:mt-0 sm:w-auto">Fill in at least path, match type & order to create rule</p>
                                        </div>
                                    {/if}
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
                    <p class="uppercase text-gray-300">API Engine settings - {domain_info._id}</p>
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
                                <h3 class="text-base font-semibold leading-6 text-gray-900" id="api-engine-enabled">API Engine enabled</h3>
                                <div class="mt-2 sm:flex sm:items-start sm:justify-between">
                                    <div class="max-w-xl text-sm text-gray-500">
                                        <p id="api-engine-enabled-text">Controls whether the API engine is enabled. Enabling enforces rules set, disabling disables all rules. Open APIs and WebSockets won't work without API engine.</p>
                                    </div>
                                    {#if !api_engine_enabled_switch}
                                    <div class="mt-5 sm:ml-6 sm:mt-0 sm:flex sm:flex-shrink-0 sm:items-center">
                                        <!-- Enabled: "bg-indigo-600", Not Enabled: "bg-gray-200" -->
                                        <button on:click={enabledButtonClick} type="button" class="bg-gray-400 relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-600 focus:ring-offset-2" role="switch" aria-checked="false" aria-labelledby="renew-subscription-label" aria-describedby="renew-subscription-description">
                                            <!-- Enabled: "translate-x-5", Not Enabled: "translate-x-0" -->
                                            <!-- TODO: fly -->
                                            <span in:fly aria-hidden="true" class="translate-x-0 inline-block h-5 w-5 transform rounded-full bg-background shadow ring-0 transition duration-200 ease-in-out"></span>
                                        </button>
                                    </div>
                                    {:else}
                                        <div class="mt-5 sm:ml-6 sm:mt-0 sm:flex sm:flex-shrink-0 sm:items-center">
                                            <!-- Enabled: "bg-indigo-600", Not Enabled: "bg-gray-200" -->
                                            <button on:click={enabledButtonClick} type="button" class="bg-indigo-600 relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-600 focus:ring-offset-2" role="switch" aria-checked="false" aria-labelledby="renew-subscription-label" aria-describedby="renew-subscription-description">
                                                <!-- Enabled: "translate-x-5", Not Enabled: "translate-x-0" -->
                                                <!-- TODO: fade -->
                                                <span in:fade aria-hidden="true" class="translate-x-5 inline-block h-5 w-5 transform rounded-full bg-background shadow ring-0 transition duration-200 ease-in-out"></span>
                                            </button>
                                        </div>
                                    {/if}
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="pt-5">
                        <div class="shadow bg-opacity-90 bg-amber-100">
                            <div class="px-4 py-5 sm:p-6">
                                <h3 class="text-base font-semibold leading-6 text-gray-900" id="api-strict-mode-enabled">Authentication</h3>
                                <div class="mt-2 sm:flex sm:items-start sm:justify-between">
                                    <div class="max-w-xl text-sm text-gray-500">
                                        <p id="api-strict-mode-enabled-text">Specify that requesters must be human to access this resource, and indicate how they should prove their authenticity (ex: you have to go to the login page before attempting to POST to the login page). Serves SMART challenge. Ultimate way to stop bots.</p>
                                    </div>
                                    {#if !api_engine_strict_mode_enabled_switch}
                                        <div class="mt-5 sm:ml-6 sm:mt-0 sm:flex sm:flex-shrink-0 sm:items-center">
                                            <!-- Enabled: "bg-indigo-600", Not Enabled: "bg-gray-200" -->
                                            <button on:click={strictModeButtonClick} type="button" class="bg-gray-400 relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-600 focus:ring-offset-2" role="switch" aria-checked="false" aria-labelledby="renew-subscription-label" aria-describedby="renew-subscription-description">
                                                <!-- Enabled: "translate-x-5", Not Enabled: "translate-x-0" -->
                                                <span aria-hidden="true" class="translate-x-0 inline-block h-5 w-5 transform rounded-full bg-background shadow ring-0 transition duration-200 ease-in-out"></span>
                                            </button>
                                        </div>
                                    {:else}
                                        <div class="mt-5 sm:ml-6 sm:mt-0 sm:flex sm:flex-shrink-0 sm:items-center">
                                            <!-- Enabled: "bg-indigo-600", Not Enabled: "bg-gray-200" -->
                                            <button on:click={strictModeButtonClick} type="button" class="bg-indigo-600 relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-600 focus:ring-offset-2" role="switch" aria-checked="false" aria-labelledby="renew-subscription-label" aria-describedby="renew-subscription-description">
                                                <!-- Enabled: "translate-x-5", Not Enabled: "translate-x-0" -->
                                                <span aria-hidden="true" class="translate-x-5 inline-block h-5 w-5 transform rounded-full bg-background shadow ring-0 transition duration-200 ease-in-out"></span>
                                            </button>
                                        </div>
                                    {/if}
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="pt-5">
                        <div class="shadow bg-opacity-90 bg-amber-100">
                            <div class="px-4 py-5 sm:p-6">
                                <h3 class="text-base font-semibold leading-6 text-gray-900" id="api-strict-mode-enabled">Strict mode enabled</h3>
                                <div class="mt-2 sm:flex sm:items-start sm:justify-between">
                                    <div class="max-w-xl text-sm text-gray-500">
                                        <p id="api-strict-mode-enabled-text">Holds all traffic to safe Web standards - POST requests must go through the API engine and GET or OPTION HTTP request bodies are discarded.</p>
                                    </div>
                                    {#if !api_engine_strict_mode_enabled_switch}
                                        <div class="mt-5 sm:ml-6 sm:mt-0 sm:flex sm:flex-shrink-0 sm:items-center">
                                            <!-- Enabled: "bg-indigo-600", Not Enabled: "bg-gray-200" -->
                                            <button on:click={strictModeButtonClick} type="button" class="bg-gray-400 relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-600 focus:ring-offset-2" role="switch" aria-checked="false" aria-labelledby="renew-subscription-label" aria-describedby="renew-subscription-description">
                                                <!-- Enabled: "translate-x-5", Not Enabled: "translate-x-0" -->
                                                <span aria-hidden="true" class="translate-x-0 inline-block h-5 w-5 transform rounded-full bg-background shadow ring-0 transition duration-200 ease-in-out"></span>
                                            </button>
                                        </div>
                                    {:else}
                                        <div class="mt-5 sm:ml-6 sm:mt-0 sm:flex sm:flex-shrink-0 sm:items-center">
                                            <!-- Enabled: "bg-indigo-600", Not Enabled: "bg-gray-200" -->
                                            <button on:click={strictModeButtonClick} type="button" class="bg-indigo-600 relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-600 focus:ring-offset-2" role="switch" aria-checked="false" aria-labelledby="renew-subscription-label" aria-describedby="renew-subscription-description">
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
                            <p class="uppercase text-gray-300">API Engine Settings</p>
                            <div class="flex flex-1 justify-end">
                                <button on:click={() => { new_setting_popup = true }} class="bg-fuchsia-500 text-white px-3 py-1.5 mx-3 rounded text-sm">
                                    + New setting
                                </button>
                            </div>
                        </div>

                        {#each settings as setting, index}
                            <div class="row-auto shadow bg-opacity-90 bg-amber-100 mb-8">
                                <div class="flex flex-1 pt-5">
                                    <div class="justify-start flex flex-1">
                                        <p class="justify-start pl-5 text-xl">{setting._id}</p>
                                        <div class="items-center pl-5">
                                            {#if setting.new_staged}
                                                <span class="inline-flex flex-shrink-0 items-center rounded-full bg-blue-50 px-1.5 py-0.5 font-medium text-blue-700 ring-1 ring-inset ring-green-600/20">staged</span>
                                            {:else if setting.being_deleted}
                                                <span class="inline-flex flex-shrink-0 items-center rounded-full bg-red-50 px-1.5 py-0.5 font-medium text-red-700 ring-1 ring-inset ring-red-600/20">staged to be deleted</span>
                                            {:else if setting.existed_modified}
                                                <span class="inline-flex flex-shrink-0 items-center rounded-full bg-yellow-50 px-1.5 py-0.5 font-medium text-yellow-700 ring-1 ring-inset ring-yellow-600/20">modified</span>
                                            {/if}
                                        </div>
                                    </div>
                                    <div class="justify-end flex flex-1 items-center">
                                        {#if getRulesFromSetting(setting).length === 1}
                                            <p>1 rule</p>
                                        {:else}
                                            <p>{getRulesFromSetting(setting).length} rules</p>
                                        {/if}
                                        <button on:click={() => { new_rule_popup = true; new_rule_setting = setting._id }} class="bg-blue-500 text-white px-3 py-1.5 ml-4 mx-2 rounded text-sm">
                                            + New rule
                                        </button>
                                        <!-- TODO -->
                                        <button on:click={async () => { settings[index].new_staged ? await confirmChange("Delete staged API engine setting?", "This will also delete any staged rules set. Because this setting was staged, nothing will differ in your production environment.", () => {
                                            settings.splice(index, 1)

                                            settings = settings;
                                        }) : await confirmChange("Delete existing API engine setting?", "This will also delete any existing rules set.", () => {
                                            settings[index].being_deleted = true;

                                            settings = settings;
                                        })}} class="text-red-500 hover:text-red-700 mr-4 rounded text-sm">
                                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M14.74 9l-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 01-2.244 2.077H8.084a2.25 2.25 0 01-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 00-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 013.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 00-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 00-7.5 0" />
                                            </svg>
                                        </button>
                                    </div>
                                </div>

                                <div class="py-3">
                                    <div class="flex px-5">
                                        <p>Open API?</p>
                                        <div>
                                            {#if !setting.changed.open_api}
                                                <div class="mt-5 sm:ml-6 sm:mt-0 sm:flex sm:flex-shrink-0 sm:items-center">
                                                    <!-- Enabled: "bg-indigo-600", Not Enabled: "bg-gray-200" -->
                                                    <button on:click={async () => await settingOpenApi(index)} type="button" class="bg-gray-400 relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-600 focus:ring-offset-2" role="switch" aria-checked="false" aria-labelledby="renew-subscription-label" aria-describedby="renew-subscription-description">
                                                        <!-- Enabled: "translate-x-5", Not Enabled: "translate-x-0" -->
                                                        <span aria-hidden="true" class="translate-x-0 inline-block h-5 w-5 transform rounded-full bg-background shadow ring-0 transition duration-200 ease-in-out"></span>
                                                    </button>
                                                </div>
                                            {:else}
                                                <div class="mt-5 sm:ml-6 sm:mt-0 sm:flex sm:flex-shrink-0 sm:items-center">
                                                    <!-- Enabled: "bg-indigo-600", Not Enabled: "bg-gray-200" -->
                                                    <button on:click={async () => await settingOpenApi(index)} type="button" class="bg-indigo-600 relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-600 focus:ring-offset-2" role="switch" aria-checked="false" aria-labelledby="renew-subscription-label" aria-describedby="renew-subscription-description">
                                                        <!-- Enabled: "translate-x-5", Not Enabled: "translate-x-0" -->
                                                        <span aria-hidden="true" class="translate-x-5 inline-block h-5 w-5 transform rounded-full bg-background shadow ring-0 transition duration-200 ease-in-out"></span>
                                                    </button>
                                                </div>
                                            {/if}
                                        </div>
                                        <p class="pl-5 text-sm text-gray-500">Enabling allows CLIs (for APIs that want public access), disabling only allows browsers to access endpoints</p>
                                    </div>
                                </div>

                                <div class="py-5 mx-5">
                                    <div class="flex items-center">
                                        <label for="search" class="text-gray-900 uppercase pr-3">whitelisted ips</label>
                                        <p class="text-sm text-gray-500">Enabling allows CLIs (for APIs that want public access), disabling only allows browsers to access endpoints</p>
                                    </div>
                                    <div class="flex">
                                        <ol class="flex flex-wrap">
                                            {#each setting.whitelist_factors.ips as ip}
                                                <li class="p-0.5 rounded-lg m-2 shadow backdrop-blur-md hover:shadow-2xl">
                                                    <p class="pr-5">{ip}</p>
                                                    <div class="absolute inset-y-0 right-0 flex py-1.5 pl-1.5">
                                                        <button on:click={async () => {
                                                            const foo = settings;

                                                            foo[index].whitelist_factors.ips.splice(foo[index].whitelist_factors.ips.indexOf(ip), 1);
                                                            console.log(foo[index].whitelist_factors.ips)
                                                            if (foo[index].changed && foo[index].changed.whitelist_factors && foo[index].changed.whitelist_factors.new_ips && foo[index].changed.whitelist_factors.new_ips.includes(ip)) {
                                                                // This ip was just staged, slice it from the staged changes
                                                                foo[index].changed.whitelist_factors.new_ips.splice(foo[index].changed.whitelist_factors.new_ips.indexOf(ip), 1);
                                                            } else if (foo[index].changed && foo[index].changed.whitelist_factors && foo[index].changed.whitelist_factors.deleted_ips && foo[index].changed.whitelist_factors.deleted_ips.includes(ip)) {
                                                                // this IP was just deleted, so we're adding it back
                                                                foo[index].changed.whitelist_factors.deleted_ips.splice(foo[index].changed.whitelist_factors.deleted_ips.indexOf(ip), 1);
                                                            } else {
                                                                // this IP was not staged, add it to the staged changes
                                                                foo[index].existed_modified = true;
                                                                if (!foo[index].changed) foo[index].changed = {};
                                                                if (!foo[index].changed.whitelist_factors) foo[index].changed.whitelist_factors = {};
                                                                if (!foo[index].changed.whitelist_factors.deleted_ips) foo[index].changed.whitelist_factors.deleted_ips = [];
                                                                foo[index].changed.whitelist_factors.deleted_ips.push(ip);
                                                            }


                                                            await letSaveChanges()

                                                            settings = foo;
                                                        }} type="button" class="inline-flex items-center px-1 font-sans text-red-400">𝕏</button>
                                                    </div>
                                                </li>
                                            {/each}
                                        </ol>
                                        <input
                                                bind:value={setting.whitelist_factor_bind}
                                                on:keydown={async (event) => {
                                                            if (event.key === 'Enter') {
                                                                const reg = /^(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$/;

                                                                if (!reg.test(setting.whitelist_factor_bind)) {
                                                                    await error_notification("Invalid IP address", "Please enter a valid IP address");
                                                                    return
                                                                }

                                                                if (setting.whitelist_factors.ips.length > 10) {
                                                                    await error_notification("Too many IPs", "You can only have 10 whitelisted IPs per setting");
                                                                    return
                                                                }

                                                                for (const ip of setting.whitelist_factors.ips) {
                                                                    if (ip === setting.whitelist_factor_bind) {
                                                                        await error_notification("IP already whitelisted", `${setting.whitelist_factor_bind} has already been added to the whitelist`);
                                                                        return
                                                                    }
                                                                }

                                                                const foo = settings;
                                                                foo[index].whitelist_factors.ips.push(setting.whitelist_factor_bind);
                                                                if (!foo[index].changed) foo[index].changed = {};
                                                                if (!foo[index].changed.whitelist_factors) foo[index].changed.whitelist_factors = {};
                                                                if (!foo[index].changed.whitelist_factors.new_ips) foo[index].changed.whitelist_factors.new_ips = [];
                                                                if (!foo[index].changed.whitelist_factors.deleted_ips) foo[index].changed.whitelist_factors.deleted_ips = [];
                                                                console.log(foo[index].changed.whitelist_factors.deleted_ips, setting.whitelist_factor_bind);
                                                                if (foo[index].changed.whitelist_factors.deleted_ips.length !== 0 && foo[index].changed.whitelist_factors.deleted_ips.includes(setting.whitelist_factor_bind)) {
                                                                    // it was just deleted, so we're adding it back

                                                                    foo[index].changed.whitelist_factors.deleted_ips = foo[index].changed.whitelist_factors.deleted_ips.filter(ip => ip !== setting.whitelist_factor_bind);
                                                                } else {
                                                                    foo[index].existed_modified = true;
                                                                    foo[index].changed.whitelist_factors.new_ips.push(setting.whitelist_factor_bind);
                                                                }

                                                                await letSaveChanges()

                                                                settings = foo;
                                                                setting.whitelist_factor_bind = undefined;
                                                        }
                                                    }}
                                                placeholder="Add an IP here..." type="text" name="search" id="search" class="bg-amber-100 py-1 text-gray-900 shadow-sm placeholder:text-gray-400 sm:text-sm sm:leading-6">
                                    </div>
                                </div>

                                        <div class="border-t-2 border-gray-500 border-dotted mx-5">
                                            <div class="pb-5 pt-5">
                                                <p class="text-gray-900 uppercase pr-3">rules</p>
                                                {#each rules as rule, rule_index}
                                                    <!-- make sure the rule is for this setting -->
                                                    {#if rule.setting === setting._id}
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
                                                                        <button on:click={() => { transferRuleInfo(rule_index); edit_rule_popup = true; }} class="border-r-2 text-gray-600 pr-3 mr-3 rounded text-sm">
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
                                                                        <p>Match method: {rule.match_type}</p>
                                                                        <p id="api-strict-mode-enabled-text">{(() => rule.http_methods.length !== 0 ? `HTTP methods: ${rule.http_methods} (query string allowed: ${(rule.allow_query_string ?? "false")})`: `No HTTP methods`)()}, {(() => rule.ws_methods.length !== 0 ? `WS methods: ${rule.ws_methods}`: `no WebSocket methods`)()}</p>
                                                                        {#if rule.actions.includes("Microcache")}
                                                                            <p>Microcache: {rule.cache_settings.level} - {rule.cache_settings.ttl} seconds</p>
                                                                        {/if}
                                                                        {#if rule.actions.includes("Use ratelimit bucket")}
                                                                            <!-- iterate through bucket list to get the specs -->
                                                                            {#each domain_info.ratelimit_buckets as bucket}
                                                                                {#if rule.bucket === bucket._id}
                                                                                    <p>Ratelimit bucket: {bucket._id} - {bucket.threshold} per {bucket.secs} seconds</p>
                                                                                {/if}
                                                                            {/each}
                                                                        {/if}
                                                                    </div>
                                                                </div>
                                                            </div>
                                                        </div>
                                                    </div>
                                                    {/if}
                                                {/each}
                                            </div>
                                        </div>
                            </div>
                        {/each}
                    </div>
                </div>
            </div>
    {/if}
</div>