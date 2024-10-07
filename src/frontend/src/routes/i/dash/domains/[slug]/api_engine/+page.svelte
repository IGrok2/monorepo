<script>
    import { page } from "$app/stores";
    import { onMount } from "svelte";
    import { fade, fly, slide, draw } from "svelte/transition";
    import Options from "$lib/Options.svelte";
    import Dialog from "$lib/components/Dialog.svelte";
    import OneOption from "$lib/OneOption.svelte";
    import OverridePopup from "$lib/base/OverridePopup.svelte";
    import ErrorConfirmationHandler from "$lib/ErrorConfirmationHandler.svelte";
    import NewEditPopup from "$lib/base/NewEditPopup.svelte";
    import EditRule from "./(components)/EditRule.svelte";
    import { toast } from "svelte-sonner";
    import { getCookie } from "$lib/utils/auth";

    import { PUBLIC_API } from "$env/static/public";

    /** @type {import('./$types').PageData} */
    export let data;

    // For loading the dashboard
    let domain_info = data.resp.domain.domain;

    let loaded = false;

    // Basic saving class properties
    let saved_class = "cursor-not-allowed bg-gray-400";
    let saving = false;
    let able_to_save = false;
    let confirmation_title;
    let confirmation_message;
    let confirmation_function;

    // API Engine specific settings
    let api_engine_enabled_switch;
    let api_engine_strict_mode_enabled_switch;
    // setting specific stuff
    let settings = [];

    // API Engine helpers
    let new_setting_popup;
    let new_setting_name;
    // rules
    let new_rule_popup;
    let new_rule_setting_index;
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
    let new_rule_cache_level;
    let new_rule_cache_ttl;
    let new_rule_bucket;
    /*
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

 */
    let edit_rule_index;
    let edit_rule_setting_index;
    let edit_rule_popup = false;

    let time_based;
    let override;
    let override_popup;

    const load = async () => {
        let body;

        try {
            time_based = Date.now();
            override = false;

            api_engine_enabled_switch = domain_info.api_engine.enabled;
            api_engine_strict_mode_enabled_switch =
                domain_info.api_engine.strict_mode_enabled;
            settings = domain_info.api_engine.rule_settings;

            for (let i = 0; i < settings.length; i++) {
                settings[i].changed = {};
                for (let j = 0; j < settings[i].rules.length; j++) {
                    settings[i].rules[j].changed = {};
                }
            }

            settings = settings;

            loaded = true;
        } catch (err) {
            console.log(error);

            /*
            if (body && body.message === "Email not verified") {
                document.location.href = "/i/auth/verify";

                return;
            }*/

            await error_notification(
                "Error loading user profile",
                "Redirecting to login page ...",
            );

            //document.location.href = "/i/auth/login";
        }
    };

    onMount(load);

    const letSaveChanges = async () => {
        if (
            getNewSettings().length === 0 &&
            getModifiedSettings().length === 0 &&
            getDeletedSettings().length === 0 &&
            getNewRules().length === 0 &&
            getModifiedRules().length === 0 &&
            getDeletedRules().length === 0 &&
            api_engine_enabled_switch === domain_info.api_engine.enabled &&
            api_engine_strict_mode_enabled_switch ===
                domain_info.api_engine.strict_mode_enabled
        ) {
            saved_class = "cursor-not-allowed bg-gray-400";
            able_to_save = false;
        } else {
            saved_class = "bg-fuchsia-500";
            able_to_save = true;
        }
    };

    const getNewSettings = async () => {
        let target_settings = [];

        for (const setting of settings) {
            if (setting.changed.new_staged === true) {
                console.log("pushed: ZERO");
                target_settings.push(setting);
            }
        }

        console.log(`returned api settings: ${target_settings.length}`);
        return target_settings;
    };

    const getModifiedSettings = async () => {
        let target_settings = [];

        for (const setting of settings) {
            if (setting.changed.existed_modified === true) {
                console.log("pushed: ONE");
                target_settings.push(setting);
            }
        }

        return target_settings;
    };

    const getDeletedSettings = async () => {
        let target_settings = [];

        for (const setting of settings) {
            if (setting.changed.being_deleted === true) {
                console.log("pushed: TWO");
                target_settings.push(setting);
            }
        }

        return target_settings;
    };

    const getNewRules = async () => {
        let target_rules = [];

        for (const setting of settings) {
            for (const rule of setting.rules) {
                if (
                    rule.changed.new_staged === true &&
                    !setting.changed.new_staged
                ) {
                    console.log("pushed: THREE");
                    target_rules.push([setting._id, rule]);
                }
            }
        }

        return target_rules;
    };

    const getModifiedRules = async () => {
        let target_rules = [];

        for (const setting of settings) {
            for (const rule of setting.rules) {
                if (rule.changed.existed_modified === true) {
                    console.log("pushed: FOUR");
                    target_rules.push([setting._id, rule]);
                }
            }
        }

        return target_rules;
    };

    const getDeletedRules = async () => {
        let target_rules = [];

        for (const setting of settings) {
            for (const rule of setting.rules) {
                if (rule.changed.being_deleted === true) {
                    console.log("pushed: FIVE");
                    target_rules.push([setting._id, rule]);
                }
            }
        }

        return target_rules;
    };

    // for enable button
    const enabledButtonClick = async () => {
        api_engine_enabled_switch = !api_engine_enabled_switch;
        await letSaveChanges();
    };

    // for strict mode button
    const strictModeButtonClick = async () => {
        api_engine_strict_mode_enabled_switch =
            !api_engine_strict_mode_enabled_switch;
        await letSaveChanges();
    };

    const newEngineSetting = async () => {
        if (new_setting_name.includes(domain_info._id)) {
            for (const setting of settings) {
                if (setting._id === new_setting_name) {
                    await error_notification(
                        "Couldn't stage new API Engine setting",
                        "Setting with that name already exists",
                    );
                    return;
                }
            }

            new_setting_popup.close();
            settings = [
                ...settings,
                {
                    _id: new_setting_name,
                    changed: {
                        new_staged: true,
                    },
                    open_api: false,
                    whitelist_factor_bind: "",
                    whitelist_factors: {
                        ips: [],
                    },
                    rules: [],
                },
            ];

            await letSaveChanges();
        } else {
            await error_notification(
                "Couldn't stage new API Engine setting",
                "Didn't include the root domain in the setting",
            );
        }
    };

    // sort API engine rules by order
    const sortRules = async (setting_index) => {
        settings[setting_index].rules.sort((a, b) => {
            return a.order - b.order;
        });
    };

    const newApiRule = async () => {
        if (new_rule_path.includes("/")) {
            for (const rule of settings[new_rule_setting_index].rules) {
                if (rule.path === new_rule_path) {
                    await error_notification(
                        "Couldn't stage new API Engine rule",
                        "Rule with that path already exists",
                    );
                    return;
                }
            }

            new_rule_popup = false;

            settings[new_rule_setting_index].rules.push({
                changed: {
                    new_staged: true,
                },
                path: new_rule_path,
                match_type: new_rule_match_type,
                order: new_rule_order,
                allow_query_string: new_rule_allow_query_string,
                ws_methods: new_rule_ws_methods,
                http_methods: new_rule_http_methods,
                actions: new_rule_actions,
                cache_ttl: new_rule_cache_ttl,
                cache_level: new_rule_cache_level,
                bucket: new_rule_bucket,
            });

            settings = settings;

            await sortRules();

            await letSaveChanges();
        } else {
            await error_notification(
                "Couldn't stage new API Engine rule",
                "Didn't include any paths",
            );
        }
    };

    // update api engine rule
    const editApiRule = async () => {
        if (new_rule_path.includes("/")) {
            for (const rule of settings[new_rule_setting_index].rules) {
                if (
                    rule.path === new_rule_path &&
                    edit_rule_index !== edit_rule_index
                ) {
                    await error_notification(
                        "Couldn't stage new API Engine rule",
                        "Rule with that path already exists",
                    );
                    return;
                }
            }

            const old_rule =
                settings[new_rule_setting_index].rules[edit_rule_index];

            let existed_modified = false;
            let new_staged = false;

            // check if the rule was changed, or if it's just staged
            if (old_rule.changed.new_staged) {
                new_staged = true;
            } else {
                existed_modified = true;
            }

            settings[new_rule_setting_index].rules[edit_rule_index] = {
                changed: {
                    new_staged,
                    existed_modified,
                },
                path: new_rule_path,
                match_type: new_rule_match_type,
                order: new_rule_order,
                allow_query_string: new_rule_allow_query_string,
                ws_methods: new_rule_ws_methods,
                http_methods: new_rule_http_methods,
                actions: new_rule_actions,
                cache_ttl: new_rule_cache_ttl,
                cache_level: new_rule_cache_level,
                bucket: new_rule_bucket,
            };

            settings = settings;

            edit_rule_popup = false;

            await sortRules();

            await letSaveChanges();
        } else {
            await error_notification(
                "Couldn't stage new API Engine rule",
                "Didn't include any paths",
            );
        }
    };

    const getChangedApiRulesFromSetting = (setting_name) => {
        let changed_rules = [];

        for (let i = 0; i < settings.length; i++) {
            if (settings[i]._id === setting_name) {
                for (let v = 0; v < settings[i].rules.length; v++) {
                    if (
                        settings[i].rules[v].changed.new_staged ||
                        settings[i].rules[v].changed.existed_modified ||
                        settings[i].rules[v].changed.being_deleted
                    ) {
                        changed_rules.push(settings[i].rules[v]);
                    }
                }
            }
        }

        return changed_rules;
    };

    const deleteApiRule = async (setting_index, rule_index) => {
        const rule = settings[setting_index].rules[rule_index];
        await confirmChange(
            `Delete ${
                rule.changed.new_staged ? `staged` : `existing`
            } API engine rule?`,
            `${
                rule.changed.new_staged
                    ? "Because this rule was staged, nothing will change in your production environment"
                    : "This rule will be staged for deletion"
            }`,
            () => {
                if (rule.changed.new_staged) {
                    settings[setting_index].rules.splice(rule_index, 1);
                    settings = settings;
                } else {
                    rule.changed.being_deleted = true;
                }
            },
        );
    };

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
    };

    const saveChanges = async () => {
        let success = true;
        if (able_to_save === true) {
            saving = true;
            save_button_success = true;
            try {
                const slug = $page.params.slug;

                const token = getCookie("jwt");

                (async () => {
                    for (let i = 0; i < settings.length; i++) {
                        delete settings[i].changed;
                        delete settings[i].existed_modified;
                        delete settings[i].whitelist_factor_bind;
                    }

                    await fetch(`${PUBLIC_API}/@/project/${$page.params.project}/domain/${$page.params.id}/api-engine/update`, {
                        method: "POST",
                        headers: new Headers({
                            "content-type": "application/json",
                            Authorization: token,
                        }),
                        body: JSON.stringify({
                            domain: slug,
                            api_engine: {
                                enabled: api_engine_enabled_switch,
                                //strict_mode_enabled: api_engine_strict_mode_enabled_switch,
                                strict_mode_enabled: true,
                                rule_settings: settings,
                            },
                            time: time_based,
                            __foo_confirm: true,
                        }),
                    })
                        .then(async (res) => {
                            let response = await res.json();
                            if (res.status !== 200) {
                                success = false;

                                if (
                                    response.message ===
                                    "Changes have been made since data last checked. Confirm changes? Open new changes in a new tab?"
                                ) {
                                    override_popup = true;
                                } else {
                                    await error_notification(
                                        "Failed to update settings",
                                        `Error: ${response.message}`,
                                    );
                                }
                            }
                        })
                        .catch(async (err) => {
                            success = false;
                            await error_notification(
                                "Failed to update settings",
                                `Try refreshing the page? ${err}`,
                            );
                        });
                })().then(async () => {
                    if (!override_popup) {
                        saving = false;
                        show_staged_changes = false;
                        save_button_success = false;
                    }
                    if (success) {
                        able_to_save = false;
                        if (!able_to_save) {
                            await load();
                        }
                        await notification(
                            "Updated settings",
                            "Pushed changes to edge",
                        );
                    }
                });
            } catch (err) {
                saving = false;
                show_staged_changes = false;
                save_button_success = false;
                await error_notification(
                    "Failed to update settings",
                    `Try refreshing the page? ${err}`,
                );
            }
        } else {
            show_staged_changes = false;

            await error_notification(
                "Wasn't able to save changes",
                "Nothing was changed",
            );
        }
    };

    // transfer data from the rule to the edit_ variables for the popup
    const transferRuleInfo = (setting_index, rule_index) => {
        const r = settings[setting_index].rules[rule_index];

        // so we can go back
        edit_rule_setting_index = setting_index;
        edit_rule_index = rule_index;

        new_rule_path = r.path;
        new_rule_match_type = r.match_type;
        new_rule_order = r.order;
        new_rule_allow_query_string = r.allow_query_string;
        new_rule_ws_methods = r.ws_methods;
        new_rule_http_methods = r.http_methods;
        new_rule_actions = r.actions;
        new_rule_cache_ttl = r.cache_ttl;
        new_rule_cache_level = r.cache_level;
        new_rule_bucket = r.bucket;
    };

    let classes;
    let message;
    let submessage;

    let error;
    let error_submessage;

    const notification = async (newMsg, sub) => {
        classes =
            "transform ease-out duration-300 transition translate-y-0 opacity-100 sm:translate-x-0";

        message = newMsg;
        submessage = sub;

        await new Promise((resolve) => setTimeout(resolve, 3000));

        classes = "transition ease-in duration-100 opacity-0";

        message = undefined;
    };

    const error_notification = async (newMsg, sub) => {
        classes =
            "transform ease-out duration-300 transition translate-y-0 opacity-100 sm:translate-x-0";

        error = newMsg;
        error_submessage = sub;

        await new Promise((resolve) => setTimeout(resolve, 7000));

        classes = "transition ease-in duration-100 opacity-0";

        error = undefined;
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

<OverridePopup
    bind:override_popup
    bind:show_staged_changes
    bind:domain_info
    bind:override
    {saveChanges}
    bind:save_button_success
/>

{#if show_staged_changes}
    <div
        in:fade
        class="relative z-20"
        aria-labelledby="modal-title"
        role="dialog"
        aria-modal="true"
    >
        <div
            class="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity"
        ></div>
        <div class="fixed inset-0 z-10 overflow-y-auto">
            <div
                class="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0"
            >
                <div
                    class="relative transform overflow-hidden rounded-lg bg-background px-4 pb-4 pt-5 text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-lg sm:p-6"
                >
                    <div class="sm:flex sm:items-start">
                        <button
                            on:click={() => (show_staged_changes = undefined)}
                            class="mx-auto flex h-12 w-12 flex-shrink-0 items-center justify-center rounded-full bg-fuchsia-500 sm:mx-0 sm:h-10 sm:w-10"
                        >
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke-width="1.5"
                                stroke="currentColor"
                                class="w-6 h-6 text-white"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    d="M9 8.25H7.5a2.25 2.25 0 00-2.25 2.25v9a2.25 2.25 0 002.25 2.25h9a2.25 2.25 0 002.25-2.25v-9a2.25 2.25 0 00-2.25-2.25H15M9 12l3 3m0 0l3-3m-3 3V2.25"
                                />
                            </svg>
                        </button>
                        <div
                            class="mt-3 text-center sm:ml-4 sm:mt-0 sm:text-left"
                        >
                            <h3
                                class="text-base font-semibold leading-6 text-gray-900"
                                id="modal-title"
                            >
                                Push staged changes to edge
                            </h3>
                            <div class="mt-2">
                                <p class="text-sm text-gray-500">Changes:</p>
                                <ol class="list-disc">
                                    {#if api_engine_enabled_switch !== domain_info.api_engine.enabled}
                                        {#if api_engine_enabled_switch === true}
                                            <li>
                                                <a class="text-green-500"
                                                    >Enable</a
                                                > API engine (settings and their
                                                rules will be activated)
                                            </li>
                                        {:else}
                                            <li>
                                                <a class="text-red-500"
                                                    >Disable</a
                                                > API engine (all rules and settings
                                                will be deactivated)
                                            </li>
                                        {/if}
                                    {/if}

                                    {#if api_engine_strict_mode_enabled_switch !== domain_info.api_engine.strict_mode_enabled}
                                        {#if api_engine_strict_mode_enabled_switch === true}
                                            <li>
                                                <a class="text-green-500"
                                                    >Enable</a
                                                > strict mode (POST requests MUST
                                                go through the API engine and GET
                                                or OPTION HTTP request bodies are
                                                discarded)
                                            </li>
                                        {:else}
                                            <li>
                                                <a class="text-red-500"
                                                    >Disable</a
                                                > strict mode (POST requests WON'T
                                                have to go through the API engine
                                                and all HTTP request bodies WILL
                                                be streamed)
                                            </li>
                                        {/if}
                                    {/if}

                                    {#if getNewSettings().length === 0}
                                        <li>
                                            <a class="text-green-500">New</a> API
                                            engine settings:
                                        </li>
                                        <ol class="list-disc pl-6"></ol>
                                    {/if}

                                    {#if getDeletedSettings().length !== 0}
                                        <li>
                                            <a class="text-red-500">Delete</a> API
                                            engine settings:
                                        </li>
                                        <ol class="list-disc pl-6"></ol>
                                    {/if}

                                    {#if getNewRules().length !== 0}
                                        <li>
                                            <a class="text-green-500">New</a> API
                                            engine rules:
                                        </li>
                                        <ol class="list-disc pl-6"></ol>
                                    {/if}
                                </ol>
                            </div>
                        </div>
                    </div>
                    <div class="mt-5 sm:ml-10 sm:mt-4 sm:flex sm:pl-4">
                        {#if save_button_success}
                            <button
                                transition:fly={{ x: 0, y: 0 }}
                                type="button"
                                class="bg-green-700 inline-flex w-full justify-center rounded-md px-1.5 py-2 text-sm font-semibold text-white shadow-sm"
                                >Blasting off ...</button
                            >
                        {:else}
                            <button
                                on:click={saveChanges}
                                type="button"
                                class="hover:cursor-[url(/svelte.config.cur),_copy] inline-flex w-full justify-center rounded-md bg-gradient-to-br from-blue-500 via-gray-700 to-fuchsia-500 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:from-blue-500 hover:via-amber-500 hover:to-fuchsia-500 sm:w-auto font-krona"
                                >Blast off: push changes to edge</button
                            >
                            <button
                                on:click={() => (show_staged_changes = false)}
                                type="button"
                                class="mt-3 inline-flex w-full justify-center rounded-md bg-background px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50 sm:ml-3 sm:mt-0 sm:w-auto"
                                >Cancel (this doesn't lose your changes)</button
                            >
                        {/if}
                    </div>
                </div>
            </div>
        </div>
    </div>
{/if}

<ErrorConfirmationHandler
    bind:confirmation_title
    bind:confirmation_message
    {confirmation_function}
    bind:classes
    {letSaveChanges}
    bind:message
    bind:submessage
    bind:error
    bind:error_submessage
/>

<Dialog  title="New API Engine Setting" bind:dialog={new_setting_popup}>
    <form
        class="relative mt-6 flex-1"
        on:submit|preventDefault={newEngineSetting}
    >
        <p>
            <label for="setting-name" class="block font-medium text-slate-100"
                >Host and (optional) path</label
            >
            <input
                bind:value={new_setting_name}
                type="text"
                name="setting-name"
                id="setting-name"
                class="mt-1 focus:ring-indigo-500 focus:border-indigo-500 block w-full shadow-sm sm:text-sm border-fuchsia-500 rounded-md"
                placeholder="api.{domain_info._id} or {domain_info._id}/api"
            />
        </p>
        <p class="pt-3 text-sm text-slate-200">
            Creating an API Engine setting is the best way to protect your API.
            You can create multiple settings, if you have multiple APIs.
            <br /><br />API settings contain a batch of rules that run under
            this host and path. For example, if you create a setting for
            <a class="underline italic">`api.based.ceo`</a>, then API engine
            will apply to
            <a class="font-bold">all paths under `api.based.ceo`</a>. It's also
            flexible, so you can have API engine run under a path like
            <a class="underline italic">`based.ceo/api`</a>. API engine will
            then stamp all paths under this host and path, and will only adhere
            to allow traffic specifically whitelisted with an API engine rule.
            <br /><br />Rules are specific: you can use them to only allow
            certain HTTP and WebSocket methods, apply ratelimiting specifically,
            check the bodies of POST and WebSocket requests, microcache public
            endpoints, and ensure proof-of-work for paths you don't wish for
            bots to access.
            <br /><br /><a class="italic"
                >By staging this change, it doesn't occur until you finalize and
                save changes.</a
            >
        </p>
        {#if new_setting_name}
            <div class="flex justify-center pt-8">
                <button
                    transition:fade
                    type="submit"
                    class="mt-3 inline-flex w-full items-center justify-center rounded-md bg-blue-500 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-fuchsia-500 duration-100 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 sm:ml-3 sm:mt-0 sm:w-auto"
                    >Stage setting {new_setting_name}</button
                >
            </div>
        {/if}
        <p class="text-sm text-slate-200">
            <br /><br />If you have any questions, our
            <a href="/support" class="text-indigo-500 hover:text-indigo-400"
                >support team</a
            >
            stands by, ready to lend a hand if you have any questions. It is staffed
            by the same people who built Packetware. You can also check the
            <a href="/docs" class="text-indigo-500 hover:text-indigo-400"
                >documentation</a
            > on API engine settings.
        </p>
    </form>
</Dialog>

<div transition:fade class="flex h-full w-full">
    <!--
    <NewEditPopup
        name={"API engine"}
        callback={newApiRule}
        edit_callback={editApiRule}
        bind:popup={new_rule_popup}
        bind:edit_popup={edit_rule_popup}
    >
        <p>
            <label for="rule-path" class="block font-medium text-slate-100"
                >Specific API path</label
            >
            <a class="text-gray-300 text-sm tracking-tight"
                >The path to invoke this rule</a
            >
            <input
                bind:value={new_rule_path}
                type="text"
                name="setting-name"
                id="rule-path"
                class="mt-1 focus:ring-indigo-500 focus:border-indigo-500 block w-full shadow-sm sm:text-sm border-fuchsia-500 rounded-md"
                placeholder="/auth/login or /auth/*"
                required
            />
        </p>

        <div>
            <label for="rule-path" class="block font-medium text-slate-100 pt-6"
                >Match type</label
            >
            <a class="text-gray-300 text-sm tracking-tight"
                >How to match the API path</a
            >
            <div>
                <div>
                    <button
                        on:click={() =>
                            (new_rule_match_type_popup =
                                !new_rule_match_type_popup)}
                        type="button"
                        class="inline-flex w-full justify-center gap-x-1.5 rounded-md bg-background px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50"
                        id="menu-button"
                        aria-expanded="true"
                        aria-haspopup="true"
                    >
                        {#if !new_rule_match_type}
                            Set a match type
                        {:else}
                            {new_rule_match_type}
                        {/if}

                        <OneOption
                            options={[
                                "Exact",
                                "Contains",
                                "StartsWith",
                                "UseStar",
                            ]}
                            bind:selected={new_rule_match_type}
                        />
                    </button>
                </div>
            </div>
        </div>

        <p>
            <label for="rule-path" class="block font-medium text-slate-100 pt-6"
                >Order</label
            >
            <a class="text-gray-300 text-sm tracking-tight"
                >A 1-20 value that represents the order API rules are processed
                in.</a
            >
            <input
                bind:value={new_rule_order}
                type="number"
                min="1"
                max="20"
                name="setting-name"
                id="rule-order"
                class="mt-1 focus:ring-indigo-500 focus:border-indigo-500 block w-full shadow-sm sm:text-sm border-fuchsia-500 rounded-md"
                placeholder="7"
                required
            />
        </p>

        <div>
            <label class="block font-medium text-slate-100 pt-6"
                >HTTP methods allowed</label
            >
            <a class="text-gray-300 text-sm tracking-tight"
                >HTTP methods to be allowed for this endpoint</a
            >
            <Options
                name={"HTTP methods"}
                options={["GET", "POST", "DELETE", "OPTIONS"]}
                bind:chosen_data={new_rule_http_methods}
            />
        </div>

        <div>
            <label class="block font-medium text-slate-100 pt-6"
                >WebSocket methods allowed</label
            >
            <a class="text-gray-300 text-sm tracking-tight"
                >WebSocket methods to be allowed for this endpoint. If no
                methods are set, WebSockets will not be allowed to connect.</a
            >
            <Options
                name={"WebSocket methods"}
                options={["TEXT", "BINARY", "PING", "CLOSE"]}
                bind:chosen_data={new_rule_ws_methods}
            />
        </div>

        <div class="flex flex-1">
            <div>
                <label class="block font-medium text-slate-100 pt-6"
                    >Allow query string</label
                >
                <a class="text-gray-300 text-sm tracking-tight"
                    >Allow query string in requests, such as /public/stats<a
                        class="underline">?query=500</a
                    ></a
                >
            </div>
            {#if !new_rule_allow_query_string}
                <div
                    class="mt-5 sm:ml-6 sm:mt-0 sm:flex sm:flex-shrink-0 sm:items-center"
                >
                    <!- Enabled: "bg-indigo-600", Not Enabled: "bg-gray-200" ->
                    <button
                        on:click={async () =>
                            (new_rule_allow_query_string = true)}
                        type="button"
                        class="bg-gray-400 relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-600 focus:ring-offset-2"
                        role="switch"
                        aria-checked="false"
                        aria-labelledby="renew-subscription-label"
                        aria-describedby="renew-subscription-description"
                    >
                        <!- Enabled: "translate-x-5", Not Enabled: "translate-x-0" ->
                        <span
                            aria-hidden="true"
                            class="translate-x-0 inline-block h-5 w-5 transform rounded-full bg-background shadow ring-0 transition duration-200 ease-in-out"
                        ></span>
                    </button>
                </div>
            {:else}
                <div
                    class="mt-5 sm:ml-6 sm:mt-0 sm:flex sm:flex-shrink-0 sm:items-center"
                >
                    <!- Enabled: "bg-indigo-600", Not Enabled: "bg-gray-200" ->
                    <button
                        on:click={async () =>
                            (new_rule_allow_query_string = false)}
                        type="button"
                        class="bg-indigo-600 relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-600 focus:ring-offset-2"
                        role="switch"
                        aria-checked="false"
                        aria-labelledby="renew-subscription-label"
                        aria-describedby="renew-subscription-description"
                    >
                        <!- Enabled: "translate-x-5", Not Enabled: "translate-x-0" ->
                        <span
                            aria-hidden="true"
                            class="translate-x-5 inline-block h-5 w-5 transform rounded-full bg-background shadow ring-0 transition duration-200 ease-in-out"
                        ></span>
                    </button>
                </div>
            {/if}
        </div>

        <div>
            <label class="block font-medium text-slate-100 pt-6">Actions</label>
            <a class="text-gray-300 text-sm tracking-tight"
                >Optional and configurable actions</a
            >
            <Options
                name={"actions"}
                options={["Microcache", "Use ratelimit bucket"]}
                bind:chosen_data={new_rule_actions}
            />
        </div>

        {#if new_rule_actions.includes("Microcache")}
            <div>
                <label class="block font-medium text-slate-100 pt-6"
                    >Caching settings</label
                >
                <a class="text-gray-300 text-sm tracking-tight"
                    >For microcaching public endpoints on your API</a
                >
                <div>
                    <div>
                        <button
                            on:click={() =>
                                (new_rule_cache_popup = !new_rule_cache_popup)}
                            type="button"
                            class="inline-flex w-full justify-center gap-x-1.5 rounded-md bg-background px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50"
                            id="menu-button"
                            aria-expanded="true"
                            aria-haspopup="true"
                        >
                            {#if !new_rule_cache_level}
                                Set a cache level
                            {:else}
                                {new_rule_cache_level}
                            {/if}
                            {#if new_rule_cache_popup}
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    stroke-width="1.5"
                                    stroke="currentColor"
                                    class="w-6 h-6"
                                >
                                    <path
                                        in:draw
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        d="M6 18L18 6M6 6l12 12"
                                    />
                                </svg>
                            {:else}
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    stroke-width="1.5"
                                    stroke="currentColor"
                                    class="w-6 h-6"
                                >
                                    <path
                                        in:draw
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        d="M3 4.5h14.25M3 9h9.75M3 13.5h9.75m4.5-4.5v12m0 0l-3.75-3.75M17.25 21L21 17.25"
                                    />
                                </svg>
                            {/if}
                        </button>
                    </div>

                    {#if new_rule_cache_popup}
                        <div
                            class="absolute z-10 mt-2 w-56 origin-top-right rounded-md bg-background shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none"
                            role="menu"
                            aria-orientation="vertical"
                            aria-labelledby="menu-button"
                            tabindex="-1"
                        >
                            <div class="py-1" role="none">
                                <OneOption
                                    options={[
                                        "None",
                                        "Standard",
                                        "IgnoreQueryString",
                                        "Aggressive",
                                    ]}
                                    bind:selected={new_rule_cache_level}
                                />
                            </div>
                        </div>
                    {/if}
                </div>
                <input
                    bind:value={new_rule_cache_ttl}
                    type="number"
                    min="5"
                    max="604800"
                    name="setting-name"
                    id="rule-order"
                    class="mt-1 focus:ring-indigo-500 focus:border-indigo-500 block w-full shadow-sm sm:text-sm border-fuchsia-500 rounded-md"
                    placeholder="Cache time-to-live"
                    required
                />
            </div>
        {/if}

        {#if new_rule_actions.includes("Use ratelimit bucket")}
            <div>
                <label class="block font-medium text-slate-100 pt-6"
                    >Ratelimiting</label
                >
                <a class="text-gray-300 text-sm tracking-tight"
                    >Use old_buckets to ratelimit IPs against this endpoint</a
                >
                <div>
                    <div>
                        <div>
                            <button
                                on:click={() =>
                                    (new_rule_ratelimit_popup =
                                        !new_rule_ratelimit_popup)}
                                type="button"
                                class="inline-flex w-full justify-center gap-x-1.5 rounded-md bg-background px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50"
                                id="menu-button"
                                aria-expanded="true"
                                aria-haspopup="true"
                            >
                                {#if !new_rule_bucket}
                                    Set a bucket
                                {:else}
                                    {new_rule_bucket}
                                {/if}
                                {#if new_rule_ratelimit_popup}
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        fill="none"
                                        viewBox="0 0 24 24"
                                        stroke-width="1.5"
                                        stroke="currentColor"
                                        class="w-6 h-6"
                                    >
                                        <path
                                            in:draw
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            d="M6 18L18 6M6 6l12 12"
                                        />
                                    </svg>
                                {:else}
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        fill="none"
                                        viewBox="0 0 24 24"
                                        stroke-width="1.5"
                                        stroke="currentColor"
                                        class="w-6 h-6"
                                    >
                                        <path
                                            in:draw
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            d="M3 4.5h14.25M3 9h9.75M3 13.5h9.75m4.5-4.5v12m0 0l-3.75-3.75M17.25 21L21 17.25"
                                        />
                                    </svg>
                                {/if}
                            </button>
                        </div>

                        {#if new_rule_ratelimit_popup}
                            <div
                                class="absolute z-10 mt-2 w-56 origin-top-right rounded-md bg-background shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none"
                                role="menu"
                                aria-orientation="vertical"
                                aria-labelledby="menu-button"
                                tabindex="-1"
                            >
                                <div class="py-1" role="none">
                                    {#if domain_info.ratelimit_buckets.length > 0}
                                        {#each domain_info.ratelimit_buckets as bucket}
                                            <button
                                                on:click={() => {
                                                    new_rule_bucket =
                                                        bucket._id;
                                                    new_rule_ratelimit_popup = false;
                                                }}
                                                class="w-full hover:bg-gray-300 duration-100 text-gray-700 block px-4 py-2 text-sm"
                                                role="menuitem"
                                                tabindex="-1"
                                                id="menu-item-0"
                                                >{bucket._id} - {bucket.threshold}
                                                per {bucket.secs}</button
                                            >
                                        {/each}
                                    {:else}
                                        <a
                                            href="/i/dash/domains/{domain_info._id}/old_buckets"
                                            target="_blank"
                                            class="w-full hover:bg-gray-300 duration-100 text-gray-700 block px-4 py-2 text-sm"
                                            role="menuitem"
                                            tabindex="-1"
                                            id="menu-item-0"
                                            >no old_buckets available - create one
                                            in a new tab</a
                                        >
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
                <button
                    transition:fade
                    type="submit"
                    class="mt-3 inline-flex w-full items-center justify-center rounded-md bg-blue-500 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-fuchsia-500 duration-100 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 sm:ml-3 sm:mt-0 sm:w-auto"
                    >Stage rule that applies on {settings[
                        new_rule_setting_index
                    ]._id}{new_rule_path}</button
                >
            </div>
        {:else}
            <div class="flex justify-center pt-8">
                <p
                    class="mt-3 inline-flex w-full items-center justify-center rounded-md text-sm font-semibold text-white shadow-sm hover:bg-red-500 duration-100 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 sm:ml-3 sm:mt-0 sm:w-auto"
                >
                    Fill in at least path, match type & order to create rule
                </p>
            </div>
        {/if}
        <p class="text-sm text-slate-200">
            <br /><br />If you have any questions, our
            <a href="/support" class="text-indigo-500 hover:text-indigo-400"
                >support team</a
            >
            stands by, ready to lend a hand if you have any questions. It is staffed
            by the same people who built Packetware. You can also check the
            <a href="/docs" class="text-indigo-500 hover:text-indigo-400"
                >documentation</a
            > on API engine settings.
        </p>
    </NewEditPopup>


    <EditRule
        bind:callback={newApiRule}
        bind:domain_info
        bind:settings
        bind:new_rule_setting_index
        bind:new_rule_path
        bind:new_rule_match_type_popup
        bind:new_rule_cache_popup
        bind:new_rule_ratelimit_popup
        bind:new_rule_match_type
        bind:new_rule_order
        bind:new_rule_allow_query_string
        bind:new_rule_ws_methods
        bind:new_rule_http_methods
        bind:new_rule_actions
        bind:new_rule_cache_level
        bind:new_rule_cache_ttl
        bind:new_rule_bucket
    />
    -->

    {#if loaded}
        <div
            transition:fade
            class="w-full p-5 pb-28 backdrop-blur-lg rounded-b-2xl shadow"
        >
            <!-- <div>
                <p class="uppercase text-gray-300">at a glance</p>
                <p class="text-slate-200 text-xl">API engine has handled 239,392 requests</p>
            </div> -->
            <div class="flex items-center justify-between">
                <p class="uppercase text-gray-300">API Engine settings</p>
                <div class="flex flex-1 justify-end">
                    <button
                        on:click={() => {
                            if (able_to_save === true)
                                show_staged_changes = true;
                        }}
                        class="{saved_class} text-white px-3 py-1.5 mx-3 rounded text-sm"
                    >
                        {#if saving}
                            Saving...
                        {:else if able_to_save}
                            View staged changes
                        {:else}
                            No changes made yet
                        {/if}
                    </button>
                    <a
                        href="/docs/api_engine"
                        class="uppercase text-indigo-200 hover:text-indigo-100 py-1.5"
                        >documentation</a
                    >
                </div>
            </div>
            <div>
                <div class="pt-5">
                    <div class="shadow bg-opacity-90 bg-amber-100">
                        <div class="px-4 py-5 sm:p-6">
                            <h3
                                class="text-base font-semibold leading-6 text-gray-900"
                                id="api-engine-enabled"
                            >
                                API Engine enabled
                            </h3>
                            <div
                                class="mt-2 sm:flex sm:items-start sm:justify-between"
                            >
                                <div class="max-w-xl text-sm text-gray-500">
                                    <p id="api-engine-enabled-text">
                                        Controls whether the API engine is
                                        enabled. Enabling enforces rules set,
                                        disabling disables all rules. Open APIs
                                        and WebSockets won't work without API
                                        engine.
                                    </p>
                                </div>
                                {#if !api_engine_enabled_switch}
                                    <div
                                        class="mt-5 sm:ml-6 sm:mt-0 sm:flex sm:flex-shrink-0 sm:items-center"
                                    >
                                        <!-- Enabled: "bg-indigo-600", Not Enabled: "bg-gray-200" -->
                                        <button
                                            on:click={enabledButtonClick}
                                            type="button"
                                            class="bg-gray-400 relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-600 focus:ring-offset-2"
                                            role="switch"
                                            aria-checked="false"
                                            aria-labelledby="renew-subscription-label"
                                            aria-describedby="renew-subscription-description"
                                        >
                                            <!-- Enabled: "translate-x-5", Not Enabled: "translate-x-0" -->
                                            <!-- TODO: fly -->
                                            <span
                                                in:fly
                                                aria-hidden="true"
                                                class="translate-x-0 inline-block h-5 w-5 transform rounded-full bg-background shadow ring-0 transition duration-200 ease-in-out"
                                            ></span>
                                        </button>
                                    </div>
                                {:else}
                                    <div
                                        class="mt-5 sm:ml-6 sm:mt-0 sm:flex sm:flex-shrink-0 sm:items-center"
                                    >
                                        <!-- Enabled: "bg-indigo-600", Not Enabled: "bg-gray-200" -->
                                        <button
                                            on:click={enabledButtonClick}
                                            type="button"
                                            class="bg-indigo-600 relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-600 focus:ring-offset-2"
                                            role="switch"
                                            aria-checked="false"
                                            aria-labelledby="renew-subscription-label"
                                            aria-describedby="renew-subscription-description"
                                        >
                                            <!-- Enabled: "translate-x-5", Not Enabled: "translate-x-0" -->
                                            <!-- TODO: fade -->
                                            <span
                                                in:fade
                                                aria-hidden="true"
                                                class="translate-x-5 inline-block h-5 w-5 transform rounded-full bg-background shadow ring-0 transition duration-200 ease-in-out"
                                            ></span>
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
                            <h3
                                class="text-base font-semibold leading-6 text-gray-900"
                                id="api-strict-mode-enabled"
                            >
                                Authentication
                            </h3>
                            <div
                                class="mt-2 sm:flex sm:items-start sm:justify-between"
                            >
                                <div class="max-w-xl text-sm text-gray-500">
                                    <p id="api-strict-mode-enabled-text">
                                        Specify that requesters must be human to
                                        access this resource, and indicate how
                                        they should prove their authenticity
                                        (ex: you have to go to the login page
                                        before attempting to POST to the login
                                        page). Serves SMART challenge. Ultimate
                                        way to stop bots.
                                    </p>
                                </div>
                                {#if !api_engine_strict_mode_enabled_switch}
                                    <div
                                        class="mt-5 sm:ml-6 sm:mt-0 sm:flex sm:flex-shrink-0 sm:items-center"
                                    >
                                        <!-- Enabled: "bg-indigo-600", Not Enabled: "bg-gray-200" -->
                                        <button
                                            on:click={strictModeButtonClick}
                                            type="button"
                                            class="bg-gray-400 relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-600 focus:ring-offset-2"
                                            role="switch"
                                            aria-checked="false"
                                            aria-labelledby="renew-subscription-label"
                                            aria-describedby="renew-subscription-description"
                                        >
                                            <!-- Enabled: "translate-x-5", Not Enabled: "translate-x-0" -->
                                            <span
                                                aria-hidden="true"
                                                class="translate-x-0 inline-block h-5 w-5 transform rounded-full bg-background shadow ring-0 transition duration-200 ease-in-out"
                                            ></span>
                                        </button>
                                    </div>
                                {:else}
                                    <div
                                        class="mt-5 sm:ml-6 sm:mt-0 sm:flex sm:flex-shrink-0 sm:items-center"
                                    >
                                        <!-- Enabled: "bg-indigo-600", Not Enabled: "bg-gray-200" -->
                                        <button
                                            on:click={strictModeButtonClick}
                                            type="button"
                                            class="bg-indigo-600 relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-600 focus:ring-offset-2"
                                            role="switch"
                                            aria-checked="false"
                                            aria-labelledby="renew-subscription-label"
                                            aria-describedby="renew-subscription-description"
                                        >
                                            <!-- Enabled: "translate-x-5", Not Enabled: "translate-x-0" -->
                                            <span
                                                aria-hidden="true"
                                                class="translate-x-5 inline-block h-5 w-5 transform rounded-full bg-background shadow ring-0 transition duration-200 ease-in-out"
                                            ></span>
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
                            <h3
                                class="text-base font-semibold leading-6 text-gray-900"
                                id="api-strict-mode-enabled"
                            >
                                Strict mode enabled
                            </h3>
                            <div
                                class="mt-2 sm:flex sm:items-start sm:justify-between"
                            >
                                <div class="max-w-xl text-sm text-gray-500">
                                    <p id="api-strict-mode-enabled-text">
                                        Holds all traffic to safe Web standards
                                        - POST requests must go through the API
                                        engine and GET or OPTION HTTP request
                                        bodies are discarded.
                                    </p>
                                </div>
                                {#if !api_engine_strict_mode_enabled_switch}
                                    <div
                                        class="mt-5 sm:ml-6 sm:mt-0 sm:flex sm:flex-shrink-0 sm:items-center"
                                    >
                                        <!-- Enabled: "bg-indigo-600", Not Enabled: "bg-gray-200" -->
                                        <button
                                            on:click={strictModeButtonClick}
                                            type="button"
                                            class="bg-gray-400 relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-600 focus:ring-offset-2"
                                            role="switch"
                                            aria-checked="false"
                                            aria-labelledby="renew-subscription-label"
                                            aria-describedby="renew-subscription-description"
                                        >
                                            <!-- Enabled: "translate-x-5", Not Enabled: "translate-x-0" -->
                                            <span
                                                aria-hidden="true"
                                                class="translate-x-0 inline-block h-5 w-5 transform rounded-full bg-background shadow ring-0 transition duration-200 ease-in-out"
                                            ></span>
                                        </button>
                                    </div>
                                {:else}
                                    <div
                                        class="mt-5 sm:ml-6 sm:mt-0 sm:flex sm:flex-shrink-0 sm:items-center"
                                    >
                                        <!-- Enabled: "bg-indigo-600", Not Enabled: "bg-gray-200" -->
                                        <button
                                            on:click={strictModeButtonClick}
                                            type="button"
                                            class="bg-indigo-600 relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-600 focus:ring-offset-2"
                                            role="switch"
                                            aria-checked="false"
                                            aria-labelledby="renew-subscription-label"
                                            aria-describedby="renew-subscription-description"
                                        >
                                            <!-- Enabled: "translate-x-5", Not Enabled: "translate-x-0" -->
                                            <span
                                                aria-hidden="true"
                                                class="translate-x-5 inline-block h-5 w-5 transform rounded-full bg-background shadow ring-0 transition duration-200 ease-in-out"
                                            ></span>
                                        </button>
                                    </div>
                                {/if}
                            </div>
                        </div>
                    </div>
                </div>

                <div class="h-full">
                    <div class="flex items-center justify-between py-5">
                        <p class="uppercase text-gray-300">
                            API Engine Settings
                        </p>
                        <div class="flex flex-1 justify-end">
                            <button
                                on:click={() => {
                                    new_setting_popup.showModal();
                                }}
                                class="bg-fuchsia-500 text-white px-3 py-1.5 mx-3 rounded text-sm"
                            >
                                + New setting
                            </button>
                        </div>
                    </div>

                    {#each settings as setting, index}
                        <div
                            class="row-auto shadow bg-opacity-90 bg-amber-100 mb-8"
                        >
                            <div class="flex flex-1 pt-5">
                                <div class="justify-start flex flex-1">
                                    <p class="justify-start pl-5 text-xl">
                                        {setting._id}
                                    </p>
                                    <div class="items-center pl-5">
                                        {#if setting.changed.new_staged}
                                            <span
                                                class="inline-flex flex-shrink-0 items-center rounded-full bg-blue-50 px-1.5 py-0.5 font-medium text-blue-700 ring-1 ring-inset ring-green-600/20"
                                                >staged</span
                                            >
                                        {:else if setting.changed.being_deleted}
                                            <span
                                                class="inline-flex flex-shrink-0 items-center rounded-full bg-red-50 px-1.5 py-0.5 font-medium text-red-700 ring-1 ring-inset ring-red-600/20"
                                                >staged to be deleted</span
                                            >
                                        {:else if setting.changed.existed_modified}
                                            <span
                                                class="inline-flex flex-shrink-0 items-center rounded-full bg-yellow-50 px-1.5 py-0.5 font-medium text-yellow-700 ring-1 ring-inset ring-yellow-600/20"
                                                >modified</span
                                            >
                                        {/if}
                                    </div>
                                </div>
                                <div
                                    class="justify-end flex flex-1 items-center"
                                >
                                    {#if setting.rules.length === 1}
                                        <p>1 rule</p>
                                    {:else}
                                        <p>{setting.rules.length} rules</p>
                                    {/if}
                                    <button
                                        on:click={() => {
                                            new_rule_popup = true;
                                        }}
                                        class="bg-blue-500 text-white px-3 py-1.5 ml-4 mx-2 rounded text-sm"
                                    >
                                        + New rule
                                    </button>
                                    <!-- TODO -->
                                    <button
                                        on:click={async () => {
                                            settings[index].new_staged
                                                ? await confirmChange(
                                                      "Delete staged API engine setting?",
                                                      "This will also delete any staged rules set. Because this setting was staged, nothing will differ in your production environment.",
                                                      () => {
                                                          settings.splice(
                                                              index,
                                                              1,
                                                          );

                                                          settings = settings;
                                                      },
                                                  )
                                                : await confirmChange(
                                                      "Delete existing API engine setting?",
                                                      "This will also delete any existing rules set.",
                                                      () => {
                                                          settings[
                                                              index
                                                          ].being_deleted =
                                                              true;

                                                          settings = settings;
                                                      },
                                                  );
                                        }}
                                        class="text-red-500 hover:text-red-700 mr-4 rounded text-sm"
                                    >
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            fill="none"
                                            viewBox="0 0 24 24"
                                            stroke-width="1.5"
                                            stroke="currentColor"
                                            class="w-6 h-6"
                                        >
                                            <path
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                d="M14.74 9l-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 01-2.244 2.077H8.084a2.25 2.25 0 01-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 00-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 013.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 00-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 00-7.5 0"
                                            />
                                        </svg>
                                    </button>
                                </div>
                            </div>

                            <div class="py-3">
                                <div class="flex px-5">
                                    <p>Open API?</p>
                                    <div>
                                        {#if !setting.changed.open_api}
                                            <div
                                                class="mt-5 sm:ml-6 sm:mt-0 sm:flex sm:flex-shrink-0 sm:items-center"
                                            >
                                                <!-- Enabled: "bg-indigo-600", Not Enabled: "bg-gray-200" -->
                                                <button
                                                    on:click={async () =>
                                                        await settingOpenApi(
                                                            index,
                                                        )}
                                                    type="button"
                                                    class="bg-gray-400 relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-600 focus:ring-offset-2"
                                                    role="switch"
                                                    aria-checked="false"
                                                    aria-labelledby="renew-subscription-label"
                                                    aria-describedby="renew-subscription-description"
                                                >
                                                    <!-- Enabled: "translate-x-5", Not Enabled: "translate-x-0" -->
                                                    <span
                                                        aria-hidden="true"
                                                        class="translate-x-0 inline-block h-5 w-5 transform rounded-full bg-background shadow ring-0 transition duration-200 ease-in-out"
                                                    ></span>
                                                </button>
                                            </div>
                                        {:else}
                                            <div
                                                class="mt-5 sm:ml-6 sm:mt-0 sm:flex sm:flex-shrink-0 sm:items-center"
                                            >
                                                <!-- Enabled: "bg-indigo-600", Not Enabled: "bg-gray-200" -->
                                                <button
                                                    on:click={async () =>
                                                        await settingOpenApi(
                                                            index,
                                                        )}
                                                    type="button"
                                                    class="bg-indigo-600 relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-600 focus:ring-offset-2"
                                                    role="switch"
                                                    aria-checked="false"
                                                    aria-labelledby="renew-subscription-label"
                                                    aria-describedby="renew-subscription-description"
                                                >
                                                    <!-- Enabled: "translate-x-5", Not Enabled: "translate-x-0" -->
                                                    <span
                                                        aria-hidden="true"
                                                        class="translate-x-5 inline-block h-5 w-5 transform rounded-full bg-background shadow ring-0 transition duration-200 ease-in-out"
                                                    ></span>
                                                </button>
                                            </div>
                                        {/if}
                                    </div>
                                    <p class="pl-5 text-sm text-gray-500">
                                        Enabling allows CLIs (for APIs that want
                                        public access), disabling only allows
                                        browsers to access endpoints
                                    </p>
                                </div>
                            </div>

                            <div class="py-5 mx-5">
                                <div class="flex items-center">
                                    <label
                                        for="search"
                                        class="text-gray-900 uppercase pr-3"
                                        >whitelisted ips</label
                                    >
                                    <p class="text-sm text-gray-500">
                                        Enabling allows CLIs (for APIs that want
                                        public access), disabling only allows
                                        browsers to access endpoints
                                    </p>
                                </div>
                                <div class="flex">
                                    <ol class="flex flex-wrap">
                                        {#each setting.whitelist_factors.ips as ip}
                                            <li
                                                class="p-0.5 rounded-lg m-2 shadow backdrop-blur-md hover:shadow-2xl"
                                            >
                                                <p class="pr-5">{ip}</p>
                                                <div
                                                    class="absolute inset-y-0 right-0 flex py-1.5 pl-1.5"
                                                >
                                                    <button
                                                        on:click={async () => {
                                                            const foo =
                                                                settings;

                                                            foo[
                                                                index
                                                            ].whitelist_factors.ips.splice(
                                                                foo[
                                                                    index
                                                                ].whitelist_factors.ips.indexOf(
                                                                    ip,
                                                                ),
                                                                1,
                                                            );
                                                            console.log(
                                                                foo[index]
                                                                    .whitelist_factors
                                                                    .ips,
                                                            );
                                                            if (
                                                                foo[index]
                                                                    .changed &&
                                                                foo[index]
                                                                    .changed
                                                                    .whitelist_factors &&
                                                                foo[index]
                                                                    .changed
                                                                    .whitelist_factors
                                                                    .new_ips &&
                                                                foo[
                                                                    index
                                                                ].changed.whitelist_factors.new_ips.includes(
                                                                    ip,
                                                                )
                                                            ) {
                                                                // This ip was just staged, slice it from the staged changes
                                                                foo[
                                                                    index
                                                                ].changed.whitelist_factors.new_ips.splice(
                                                                    foo[
                                                                        index
                                                                    ].changed.whitelist_factors.new_ips.indexOf(
                                                                        ip,
                                                                    ),
                                                                    1,
                                                                );
                                                            } else if (
                                                                foo[index]
                                                                    .changed &&
                                                                foo[index]
                                                                    .changed
                                                                    .whitelist_factors &&
                                                                foo[index]
                                                                    .changed
                                                                    .whitelist_factors
                                                                    .deleted_ips &&
                                                                foo[
                                                                    index
                                                                ].changed.whitelist_factors.deleted_ips.includes(
                                                                    ip,
                                                                )
                                                            ) {
                                                                // this IP was just deleted, so we're adding it back
                                                                foo[
                                                                    index
                                                                ].changed.whitelist_factors.deleted_ips.splice(
                                                                    foo[
                                                                        index
                                                                    ].changed.whitelist_factors.deleted_ips.indexOf(
                                                                        ip,
                                                                    ),
                                                                    1,
                                                                );
                                                            } else {
                                                                // this IP was not staged, add it to the staged changes
                                                                foo[
                                                                    index
                                                                ].existed_modified =
                                                                    true;
                                                                if (
                                                                    !foo[index]
                                                                        .changed
                                                                )
                                                                    foo[
                                                                        index
                                                                    ].changed =
                                                                        {};
                                                                if (
                                                                    !foo[index]
                                                                        .changed
                                                                        .whitelist_factors
                                                                )
                                                                    foo[
                                                                        index
                                                                    ].changed.whitelist_factors =
                                                                        {};
                                                                if (
                                                                    !foo[index]
                                                                        .changed
                                                                        .whitelist_factors
                                                                        .deleted_ips
                                                                )
                                                                    foo[
                                                                        index
                                                                    ].changed.whitelist_factors.deleted_ips =
                                                                        [];
                                                                foo[
                                                                    index
                                                                ].changed.whitelist_factors.deleted_ips.push(
                                                                    ip,
                                                                );
                                                            }

                                                            await letSaveChanges();

                                                            settings = foo;
                                                        }}
                                                        type="button"
                                                        class="inline-flex items-center px-1 font-sans text-red-400"
                                                        >𝕏</button
                                                    >
                                                </div>
                                            </li>
                                        {/each}
                                    </ol>
                                    <input
                                        bind:value={setting.whitelist_factor_bind}
                                        on:keydown={async (event) => {
                                            if (event.key === "Enter") {
                                                const reg =
                                                    /^(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$/;

                                                if (
                                                    !reg.test(
                                                        setting.whitelist_factor_bind,
                                                    )
                                                ) {
                                                    await error_notification(
                                                        "Invalid IP address",
                                                        "Please enter a valid IP address",
                                                    );
                                                    return;
                                                }

                                                if (
                                                    setting.whitelist_factors
                                                        .ips.length > 10
                                                ) {
                                                    await error_notification(
                                                        "Too many IPs",
                                                        "You can only have 10 whitelisted IPs per setting",
                                                    );
                                                    return;
                                                }

                                                for (const ip of setting
                                                    .whitelist_factors.ips) {
                                                    if (
                                                        ip ===
                                                        setting.whitelist_factor_bind
                                                    ) {
                                                        await error_notification(
                                                            "IP already whitelisted",
                                                            `${setting.whitelist_factor_bind} has already been added to the whitelist`,
                                                        );
                                                        return;
                                                    }
                                                }

                                                const foo = settings;
                                                foo[
                                                    index
                                                ].whitelist_factors.ips.push(
                                                    setting.whitelist_factor_bind,
                                                );
                                                if (!foo[index].changed)
                                                    foo[index].changed = {};
                                                if (
                                                    !foo[index].changed
                                                        .whitelist_factors
                                                )
                                                    foo[
                                                        index
                                                    ].changed.whitelist_factors =
                                                        {};
                                                if (
                                                    !foo[index].changed
                                                        .whitelist_factors
                                                        .new_ips
                                                )
                                                    foo[
                                                        index
                                                    ].changed.whitelist_factors.new_ips =
                                                        [];
                                                if (
                                                    !foo[index].changed
                                                        .whitelist_factors
                                                        .deleted_ips
                                                )
                                                    foo[
                                                        index
                                                    ].changed.whitelist_factors.deleted_ips =
                                                        [];
                                                console.log(
                                                    foo[index].changed
                                                        .whitelist_factors
                                                        .deleted_ips,
                                                    setting.whitelist_factor_bind,
                                                );
                                                if (
                                                    foo[index].changed
                                                        .whitelist_factors
                                                        .deleted_ips.length !==
                                                        0 &&
                                                    foo[
                                                        index
                                                    ].changed.whitelist_factors.deleted_ips.includes(
                                                        setting.whitelist_factor_bind,
                                                    )
                                                ) {
                                                    // it was just deleted, so we're adding it back

                                                    foo[
                                                        index
                                                    ].changed.whitelist_factors.deleted_ips =
                                                        foo[
                                                            index
                                                        ].changed.whitelist_factors.deleted_ips.filter(
                                                            (ip) =>
                                                                ip !==
                                                                setting.whitelist_factor_bind,
                                                        );
                                                } else {
                                                    foo[
                                                        index
                                                    ].existed_modified = true;
                                                    foo[
                                                        index
                                                    ].changed.whitelist_factors.new_ips.push(
                                                        setting.whitelist_factor_bind,
                                                    );
                                                }

                                                await letSaveChanges();

                                                settings = foo;
                                                setting.whitelist_factor_bind =
                                                    undefined;
                                            }
                                        }}
                                        placeholder="Add an IP here..."
                                        type="text"
                                        name="search"
                                        id="search"
                                        class="bg-amber-100 py-1 text-gray-900 shadow-sm placeholder:text-gray-400 sm:text-sm sm:leading-6"
                                    />
                                </div>
                            </div>

                            <div
                                class="border-t-2 border-gray-500 border-dotted mx-5"
                            >
                                <div class="pb-5 pt-5">
                                    <p class="text-gray-900 uppercase pr-3">
                                        rules
                                    </p>
                                    <form on:submit|preventDefault={newApiRule}>
                                        <EditRule
                                            bind:domain_info
                                            bind:settings
                                            bind:new_rule_popup
                                            bind:new_rule_setting_index
                                            bind:new_rule_path
                                            bind:new_rule_match_type_popup
                                            bind:new_rule_cache_popup
                                            bind:new_rule_ratelimit_popup
                                            bind:new_rule_match_type
                                            bind:new_rule_order
                                            bind:new_rule_allow_query_string
                                            bind:new_rule_ws_methods
                                            bind:new_rule_http_methods
                                            bind:new_rule_actions
                                            bind:new_rule_cache_level
                                            bind:new_rule_cache_ttl
                                            bind:new_rule_bucket
                                        />
                                    </form>
                                    {#each setting.rules as rule, rule_index}
                                        <!-- make sure the rule is for this setting -->
                                        {#if rule.setting === setting._id}
                                            <div class="pt-5">
                                                <div
                                                    class="shadow bg-opacity-90 bg-amber-100"
                                                >
                                                    <div
                                                        class="px-4 py-5 sm:p-6"
                                                    >
                                                        <div
                                                            class="sm:flex sm:flex-1 sm:w-full sm:justify-between"
                                                        >
                                                            <div
                                                                class="justify-start sm:flex sm:flex-1"
                                                            >
                                                                <h3
                                                                    class="text-base font-semibold leading-6 text-gray-900 py-0.5 pr-3"
                                                                    id="api-strict-mode-enabled"
                                                                >
                                                                    {rule.path}
                                                                </h3>
                                                                {#if rule.changed.new_staged}
                                                                    <span
                                                                        class="inline-flex flex-shrink-0 items-center rounded-full bg-blue-50 px-1.5 py-0.5 font-medium text-blue-700 ring-1 ring-inset ring-green-600/20"
                                                                        >staged</span
                                                                    >
                                                                {:else if rule.changed.existed_modified}
                                                                    <span
                                                                        class="inline-flex flex-shrink-0 items-center rounded-full bg-yellow-50 px-1.5 py-0.5 font-medium text-yellow-700 ring-1 ring-inset ring-yellow-600/20"
                                                                        >modified</span
                                                                    >
                                                                {:else if rule.changed.being_deleted}
                                                                    <span
                                                                        class="inline-flex flex-shrink-0 items-center rounded-full bg-red-50 px-1.5 py-0.5 font-medium text-red-700 ring-1 ring-inset ring-red-600/20"
                                                                        >staged
                                                                        to be
                                                                        deleted</span
                                                                    >
                                                                {/if}
                                                            </div>
                                                            <div
                                                                class="justify-end flex flex-1 items-center"
                                                            >
                                                                <p
                                                                    class="mr-3 text-gray-800 text-xl px-3 border-r-2"
                                                                >
                                                                    #{rule.order}
                                                                </p>
                                                                <button
                                                                    on:click={() => {
                                                                        transferRuleInfo(
                                                                            rule_index,
                                                                        );
                                                                        edit_rule_popup = true;
                                                                    }}
                                                                    class="border-r-2 text-gray-600 pr-3 mr-3 rounded text-sm"
                                                                >
                                                                    <svg
                                                                        xmlns="http://www.w3.org/2000/svg"
                                                                        fill="none"
                                                                        viewBox="0 0 24 24"
                                                                        stroke-width="1.5"
                                                                        stroke="currentColor"
                                                                        class="w-6 h-6"
                                                                    >
                                                                        <path
                                                                            stroke-linecap="round"
                                                                            stroke-linejoin="round"
                                                                            d="M16.862 4.487l1.687-1.688a1.875 1.875 0 112.652 2.652L6.832 19.82a4.5 4.5 0 01-1.897 1.13l-2.685.8.8-2.685a4.5 4.5 0 011.13-1.897L16.863 4.487zm0 0L19.5 7.125"
                                                                        />
                                                                    </svg>
                                                                </button>
                                                                <button
                                                                    on:click={async () =>
                                                                        deleteApiRule(
                                                                            index,
                                                                            rule_index,
                                                                        )}
                                                                    class="text-red-500 hover:text-red-700 rounded text-sm"
                                                                >
                                                                    <svg
                                                                        xmlns="http://www.w3.org/2000/svg"
                                                                        fill="none"
                                                                        viewBox="0 0 24 24"
                                                                        stroke-width="1.5"
                                                                        stroke="currentColor"
                                                                        class="w-6 h-6"
                                                                    >
                                                                        <path
                                                                            stroke-linecap="round"
                                                                            stroke-linejoin="round"
                                                                            d="M14.74 9l-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 01-2.244 2.077H8.084a2.25 2.25 0 01-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 00-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 013.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 00-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 00-7.5 0"
                                                                        />
                                                                    </svg>
                                                                </button>
                                                            </div>
                                                        </div>
                                                        <div
                                                            class="mt-2 sm:flex sm:items-start sm:justify-between"
                                                        >
                                                            <div
                                                                class="max-w-xl text-sm text-gray-500"
                                                            >
                                                                <p>
                                                                    Match
                                                                    method: {rule.match_type}
                                                                </p>
                                                                <p
                                                                    id="api-strict-mode-enabled-text"
                                                                >
                                                                    {(() =>
                                                                        rule
                                                                            .http_methods
                                                                            .length !==
                                                                        0
                                                                            ? `HTTP methods: ${
                                                                                  rule.http_methods
                                                                              } (query string allowed: ${
                                                                                  rule.allow_query_string ??
                                                                                  "false"
                                                                              })`
                                                                            : `No HTTP methods`)()},
                                                                    {(() =>
                                                                        rule
                                                                            .ws_methods
                                                                            .length !==
                                                                        0
                                                                            ? `WS methods: ${rule.ws_methods}`
                                                                            : `no WebSocket methods`)()}
                                                                </p>
                                                                {#if rule.actions.includes("Microcache")}
                                                                    <p>
                                                                        Microcache:
                                                                        {rule
                                                                            .cache_settings
                                                                            .level}
                                                                        - {rule
                                                                            .cache_settings
                                                                            .ttl}
                                                                        seconds
                                                                    </p>
                                                                {/if}
                                                                {#if rule.actions.includes("Use ratelimit bucket")}
                                                                    <!-- iterate through bucket list to get the specs -->
                                                                    {#each domain_info.ratelimit_buckets as bucket}
                                                                        {#if rule.bucket === bucket._id}
                                                                            <p>
                                                                                Ratelimit
                                                                                bucket:
                                                                                {bucket._id}
                                                                                -
                                                                                {bucket.threshold}
                                                                                per
                                                                                {bucket.secs}
                                                                                seconds
                                                                            </p>
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
