<!--
    _id: z.string().max(50), // host
    ssl: z.boolean(),
    http2: z.boolean(),
    timeout: z.number().max(60).min(3),
    ip_data: z.boolean(),
    origins: z.array(z.object({
        url: z.string().max(50).withPredicate(ip_checker, "Invalid origin."), // TODO regex
        weight: z.number().max(20).min(1),
    })).max(5),
-->

<script>
    import { page } from "$app/stores";
    import { onMount } from "svelte";
    import { fade, fly, slide, draw } from "svelte/transition";
    import Options from "$lib/Options.svelte";
    import DisplayOriginSetting from "$lib/origin_settings/DisplayOriginSetting.svelte";
    import Confirmation from "$lib/base/Confirmation.svelte";
    import BasicSwitch from "$lib/base/BasicSwitch.svelte";
    import BasicLink from "$lib/base/BasicLink.svelte";
    import DocsLink from "$lib/base/DocsLink.svelte";
    import Input from "$lib/Input.svelte";
    import OneOption from "$lib/OneOption.svelte";
    import ip_checker from "$lib/tools/ip_checker.js";
    import OverridePopup from "$lib/base/OverridePopup.svelte";
    import Popup from "$lib/Popup.svelte";
    import NewEditPopup from "$lib/base/NewEditPopup.svelte";
    import { PUBLIC_API } from "$env/static/public";
    import Notifications from "$lib/components/notifications/Notifications.svelte";
    import { getCookie } from "$lib/utils/auth";

    // For loading the dashboard
    let domain_info;
    let loaded = false;
    let time_based;
    let override;
    let override_popup;

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

    // origin setting helpers

    let origin_settings = [];

    let new_origin_setting_popup = false;
    let edit_origin_setting_popup = false;
    let edit_origin_index;

    let new_origin_id;
    let new_origin_ssl = true;
    let new_origin_http2 = true;
    let new_origin_timeout = false;
    let new_origin_ipdata = false;
    let new_origin_origins = [
        {
            url: "",
            weight: 1,
        },
    ];

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

                if (rawBody.status) {
                    domain_info = body.domain;

                    if (domain_info.api_engine.enabled) {
                        api_engine = "from-green-500";
                    } else {
                        api_engine = "from-red-500";
                    }
                    if (domain_info.caching_settings.enabled) {
                        cache_engine = "from-green-500";
                    } else {
                        cache_engine = "from-red-500";
                    }
                    if (domain_info.bot_management.enabled) {
                        bot_engine = "from-green-500";
                    } else {
                        bot_engine = "from-red-500";
                    }
                    if (domain_info.rules.enabled) {
                        page_rules = "from-green-500";
                    } else {
                        page_rules = "from-red-500";
                    }
                    const foo_origin_settings = domain_info.origin_settings;

                    for (let i = 0; i < foo_origin_settings.length; i++) {
                        foo_origin_settings[i].changed = {};
                    }

                    origin_settings = foo_origin_settings;

                    time_based = Date.now();

                    loaded = true;
                } else {
                    document.location.href = `/i/dash/domains/${$page.params.slug}`;
                }
            });
        } catch (err) {
            console.log(err);

            await error_notification(
                "Error loading user profile",
                "Redirecting to login page ...",
            );

            document.location.href = "/i/auth/login";
        }
    };

    onMount(load);

    const deleteOriginSetting = async (name) => {
        for (let i = 0; i < origin_settings.length; i++) {
            if (origin_settings[i]._id === name) {
                // was the bucket staged, if so, slice
                if (origin_settings[i].changed.new_staged) {
                    await confirmChange(
                        "Delete staged origin setting?",
                        "Because this origin setting was staged, nothing will differ in your production environment",
                        () => {
                            origin_settings.splice(i, 1);
                            origin_settings = origin_settings;
                        },
                    );
                } else {
                    await confirmChange(
                        "Delete production origin setting?",
                        "This origin setting will be staged for deletion",
                        () => {
                            origin_settings[i].changed.being_deleted = true;
                            origin_settings = origin_settings;
                        },
                    );
                }
            }
        }
    };

    const getNewSettings = () => {
        let new_settings = [];

        for (let i = 0; i < origin_settings.length; i++) {
            if (origin_settings[i].changed?.new_staged) {
                new_settings.push(origin_settings[i]);
            }
        }

        return new_settings;
    };

    const getDeletedSettings = () => {
        let deleted_settings = [];

        for (let i = 0; i < origin_settings.length; i++) {
            if (origin_settings[i].changed?.being_deleted) {
                deleted_settings.push(origin_settings[i]);
            }
        }

        return deleted_settings;
    };

    const getChangedSettings = () => {
        let changed_settings = [];

        for (let i = 0; i < origin_settings.length; i++) {
            const setting = origin_settings[i];

            if (setting.changed?.existed_modified) {
                changed_settings.push(setting);
            }
        }

        return changed_settings;
    };

    const letSaveChanges = () => {
        let local_able_to_save =
            getNewSettings().length !== 0 ||
            getChangedSettings().length !== 0 ||
            getDeletedSettings().length !== 0;

        if (local_able_to_save) {
            able_to_save = true;
            saved_class = "bg-fuchsia-500";
        } else {
            able_to_save = false;
            saved_class = "cursor-not-allowed bg-gray-400";
        }
    };

    const editSetting = async () => {
        if (
            new_origin_id.startsWith(".") ||
            !new_origin_id ||
            !new_origin_id.includes(domain_info._id) ||
            !new_origin_origins ||
            new_origin_origins.length === 0 ||
            new_origin_http2 === undefined
        ) {
            await error_notification(
                "Couldn't edit origin setting",
                "Please check the host and make sure it includes the domain",
            );
            return;
        }

        if (!checkOrigins()) {
            await error_notification(
                "Couldn't edit origin setting",
                "Please make sure all origins are filled out!",
            );
            return;
        }

        // see if the name has already been used
        for (let i = 0; i < origin_settings.length; i++) {
            if (
                origin_settings[i]._id === new_origin_id &&
                i !== edit_origin_index
            ) {
                await error_notification(
                    "Couldn't edit origin setting",
                    "There's a origin setting with that host",
                );
                return;
            }
        }

        const ssl_specs = { ...origin_settings[edit_origin_index].ssl_specs };

        origin_settings.push({
            changed: {
                existed_modified: true,
            },
            ssl_specs,
            _id: new_origin_id,
            ssl: new_origin_ssl,
            http2: new_origin_http2,
            timeout: new_origin_timeout,
            ip_data: new_origin_ipdata,
            origins: new_origin_origins,
        });

        origin_settings.splice(edit_origin_index, 1);

        origin_settings = origin_settings;

        edit_origin_setting_popup = false;

        letSaveChanges();
    };

    const newSetting = async () => {
        if (
            new_origin_id.startsWith(".") ||
            !new_origin_id ||
            !new_origin_id.includes(domain_info._id) ||
            !new_origin_origins ||
            new_origin_origins.length === 0 ||
            new_origin_http2 === undefined
        ) {
            await error_notification(
                "Couldn't stage new origin setting",
                "Please check the host and make sure it includes the domain",
            );
            return;
        }

        if (!checkOrigins()) {
            await error_notification(
                "Couldn't stage new origin setting",
                "Please make sure all origins are filled out!",
            );
            return;
        }

        // see if the name has already been used
        for (let i = 0; i < origin_settings.length; i++) {
            if (origin_settings[i]._id === new_origin_id) {
                await error_notification(
                    "Couldn't stage new origin setting",
                    "There is a setting with the same host",
                );
                return;
            }
        }

        new_origin_setting_popup = false;

        origin_settings.push({
            changed: {
                new_staged: true,
            },
            ssl_specs: {
                verified: false,
            },
            _id: new_origin_id,
            ssl: new_origin_ssl,
            http2: new_origin_http2,
            timeout: new_origin_timeout,
            ip_data: new_origin_ipdata,
            origins: new_origin_origins,
        });

        origin_settings = origin_settings;

        letSaveChanges();
    };

    const modifyOriginSettingPopup = (origin_index) => {
        const origin_setting = origin_settings[origin_index];

        new_origin_id = origin_setting._id;
        new_origin_http2 = origin_setting.http2;
        new_origin_ssl = origin_setting.ssl;
        new_origin_ipdata = origin_setting.ip_data;
        new_origin_timeout = origin_setting.timeout;
        new_origin_origins = origin_setting.origins;
        edit_origin_index = origin_index;

        edit_origin_setting_popup = true;
    };

    const checkOrigins = () => {
        let cool = true;

        for (let i = 0; i < new_origin_origins.length; i++) {
            if (
                !new_origin_origins[i].url ||
                new_origin_origins[i].weight === null ||
                !ip_checker(new_origin_origins[i].url)
            ) {
                console.log(
                    !new_origin_origins[i].ip,
                    new_origin_origins[i].weight === null,
                    !ip_checker(new_origin_origins[i].ip),
                );
                cool = false;
                break;
            }
        }

        return cool;
    };

    const saveChanges = async () => {
        if (able_to_save === true) {
            saving = true;
            save_button_success = true;

            try {
                const slug = $page.params.slug;

                const token = getCookie("jwt");

                let temp_origin_settings = JSON.parse(
                    JSON.stringify(origin_settings),
                );

                for (let i = 0; i < temp_origin_settings.length; i++) {
                    if (
                        temp_origin_settings[i].changed.being_deleted === true
                    ) {
                        temp_origin_settings.splice(i, 1);
                        i--;
                    } else {
                        delete temp_origin_settings[i].changed;
                        delete temp_origin_settings[i].ssl_specs;
                        for (
                            let v = 0;
                            v < temp_origin_settings[i].origins.length;
                            v++
                        ) {
                            delete temp_origin_settings[i].origins[v]._id;
                        }
                    }
                }

                const request = await fetch(
                    `${PUBLIC_API}/@/project/${$page.params.project}/domain/${$page.params.id}/origins/update`,
                    {
                        method: "POST",
                        headers: new Headers({
                            "content-type": "application/json",
                            Authorization: token,
                        }),
                        body: JSON.stringify({
                            domain: slug,
                            origin_settings: temp_origin_settings,
                            time: time_based,
                            __foo_confirm: override,
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
                        throw new Error(response.message);
                    }
                } else {
                    saving = false;
                    show_staged_changes = false;
                    save_button_success = false;
                    await load();
                    letSaveChanges();
                    await notification(
                        "Updated settings",
                        "Pushed changes to edge",
                    );
                }
            } catch (err) {
                saving = false;
                show_staged_changes = false;
                save_button_success = false;
                await error_notification(
                    "Failed to update settings",
                    `Try refreshing the page? ${err}`,
                );
            }
        }
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

<Notifications
    bind:message
    bind:classes
    bind:submessage
    bind:error
    bind:error_submessage
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
                                    {#if getDeletedSettings().length !== 0}
                                        <li>
                                            <a class="text-red-500">Delete</a> origin
                                            settings:
                                        </li>
                                        <ol
                                            class="list-disc pl-6 text-slate-600"
                                        >
                                            {#each getDeletedSettings() as setting}
                                                <li class="font-light">
                                                    {setting._id}
                                                </li>
                                            {/each}
                                        </ol>
                                    {/if}

                                    {#if getChangedSettings().length !== 0}
                                        <li>
                                            <a class="text-yellow-500">Change</a
                                            > origin settings:
                                        </li>
                                        <ol
                                            class="list-disc pl-6 text-slate-600"
                                        >
                                            {#each getChangedSettings() as setting}
                                                <DisplayOriginSetting
                                                    {setting}
                                                />
                                            {/each}
                                        </ol>
                                    {/if}

                                    {#if getNewSettings().length !== 0}
                                        <li>
                                            <a class="text-green-500">New</a> origin
                                            settings:
                                        </li>
                                        <ol
                                            class="list-disc pl-6 text-slate-600"
                                        >
                                            {#each getNewSettings() as setting}
                                                <DisplayOriginSetting
                                                    {setting}
                                                />
                                            {/each}
                                        </ol>
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

<NewEditPopup
    bind:popup={new_origin_setting_popup}
    callback={newSetting}
    bind:edit_popup={edit_origin_setting_popup}
    edit_callback={editSetting}
    name="origin setting"
>
    <p>
        <label
            for="origin-setting-host"
            class="block font-medium text-slate-100">Host</label
        >
        <a class="text-gray-300 text-sm tracking-tight"
            >Where this origin setting will apply. This should include the
            domain.</a
        >
        <input
            bind:value={new_origin_id}
            type="text"
            name="setting-name"
            id="origin-setting-host"
            class="mt-1 focus:ring-indigo-500 focus:border-indigo-500 block w-full shadow-sm sm:text-sm border-fuchsia-500 rounded-md"
            placeholder="hey-jude.{domain_info._id}"
            required
        />
    </p>

    <div class="flex flex-1">
        <div>
            <label class="block font-medium text-slate-100 pt-6">SSL</label>
            <a class="text-gray-300 text-sm tracking-tight"
                >Indicates whether we contact your origins over SSL. It's
                recommended to have this on, and we'll respect both self-signed
                certificates and CA-signed certificates. It's important to note
                that if SSL is selected, we won't accept a downgrade to HTTP.</a
            >
        </div>
        <BasicSwitch bind:switch_variable={new_origin_ssl} />
    </div>

    <div class="flex flex-1">
        <div>
            <label class="block font-medium text-slate-100 pt-6">HTTP/2</label>
            <a class="text-gray-300 text-sm tracking-tight"
                >Indicates whether we should contact your backend over HTTP/2.
                Can be more efficient for a high number of requests. If a HTTP/2
                connection cannot be established, we will rely on HTTP/1.1</a
            >
        </div>
        <BasicSwitch bind:switch_variable={new_origin_http2} />
    </div>

    <div class="flex flex-1">
        <div>
            <label class="block font-medium text-slate-100 pt-6"
                >Forward IP data</label
            >
            <a class="text-gray-300 text-sm tracking-tight"
                >Indicates whether we should forward IP data, such as ASN and
                continent, to your backend via headers. See <BasicLink
                    text="documentation"
                    href="docs.packetware.net/origin_settings"
                /> for more details</a
            >
        </div>
        <BasicSwitch bind:switch_variable={new_origin_ipdata} />
    </div>

    <p>
        <label
            for="origin-setting-timeout"
            class="block font-medium text-slate-100 pt-6">Timeout</label
        >
        <a class="text-gray-300 text-sm tracking-tight"
            >A 3-60 (three seconds to 60 seconds) value that how long we should
            wait for a response from your backend.</a
        >
        <input
            bind:value={new_origin_timeout}
            type="number"
            min="3"
            max="60"
            name="origin-setting-timeout"
            id="rule-order"
            class="mt-1 focus:ring-indigo-500 focus:border-indigo-500 block w-full shadow-sm sm:text-sm border-fuchsia-500 rounded-md"
            placeholder="10"
            required
        />
    </p>

    <div>
        <div class="flex justify-between flex-1 items-center">
            <div class="justify-start">
                <label class="text-xl block font-medium text-slate-100 pt-6"
                    >Origins</label
                >
                <a class="text-gray-300 text-sm tracking-tight"
                    >Backends for {new_origin_id}</a
                >
            </div>
            <div class="justify-end items-center">
                <button
                    type="button"
                    on:click={() => {
                        new_origin_origins = [
                            ...new_origin_origins,
                            { url: "", weight: 1 },
                        ];
                    }}
                    class="bg-fuchsia-500 text-white px-3 py-1.5 mx-3 rounded text-sm"
                >
                    +
                </button>
            </div>
        </div>
        <div class="bg-opacity-10 bg-background p-3 mt-5 rounded-lg">
            <!-- or bg white? -->
            {#each new_origin_origins as origin, origin_index}
                <div class="{origin_index !== 0 ?? `mt-6`} p-3">
                    <!-- includes url and weight -->
                    <div class="flex flex-1 justify-between">
                        <p class="text-sm uppercase text-slate-300">
                            Origin #{origin_index + 1}
                        </p>
                        <div
                            on:click={async () => {
                                if (origin_index !== 0) {
                                    new_origin_origins.splice(origin_index, 1);
                                    new_origin_origins = new_origin_origins;
                                } else {
                                    await error_notification(
                                        "Couldn't delete origin",
                                        "At least one origin is required",
                                    );
                                }
                            }}
                            class="text-xl text-red-500 cursor-grab"
                        >
                            𝕏
                        </div>
                    </div>
                    <div>
                        <p>
                            <label
                                for="ip"
                                class="block font-medium text-slate-100 pt-4"
                                >IP address</label
                            >
                            <a class="text-gray-300 text-sm tracking-tight"
                                >The IP address of this origin</a
                            >
                            <input
                                bind:value={origin.url}
                                type="text"
                                name="ip"
                                id="ip"
                                class="mt-1 focus:ring-indigo-500 focus:border-indigo-500 block w-full shadow-sm sm:text-sm border-fuchsia-500 rounded-md"
                                placeholder="23.133.104.69"
                                required
                            />
                        </p>

                        <p>
                            <label
                                for="weight"
                                class="block font-medium text-slate-100 pt-6"
                                >Weight</label
                            >
                            <a class="text-gray-300 text-sm tracking-tight"
                                >A number from 1-20 that indicates how this
                                backend should be weighted against other
                                backends. For more see information, see <DocsLink
                                /></a
                            >
                            <input
                                bind:value={origin.weight}
                                type="number"
                                min="1"
                                max="20"
                                name="weight"
                                id="weight"
                                class="mt-1 focus:ring-indigo-500 focus:border-indigo-500 block w-full shadow-sm sm:text-sm border-fuchsia-500 rounded-md"
                                placeholder="10"
                                required
                            />
                        </p>
                    </div>
                </div>
            {/each}
        </div>
    </div>

    {#if new_origin_id && new_origin_timeout}
        <div class="flex justify-center pt-8">
            <button
                transition:fade
                type="submit"
                class="mt-3 inline-flex w-full items-center justify-center rounded-md bg-blue-500 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-fuchsia-500 duration-100 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 sm:ml-3 sm:mt-0 sm:w-auto"
                >Stage origin setting {new_origin_id}</button
            >
        </div>
    {:else}
        <div class="flex justify-center pt-8">
            <p
                class="mt-3 inline-flex w-full items-center justify-center rounded-md text-sm font-semibold text-white shadow-sm hover:bg-red-500 duration-100 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 sm:ml-3 sm:mt-0 sm:w-auto"
            >
                All fields must be filled in to create this bucket.
            </p>
        </div>
    {/if}
    <p class="text-sm text-slate-200">
        <br /><br />If you have any questions, our
        <a href="/support" class="text-indigo-500 hover:text-indigo-400"
            >support team</a
        >
        stands by, ready to lend a hand if you have any questions. It is staffed
        by the same people who built Packetware. You can also check the <DocsLink
        /> on origin settings.
    </p>
</NewEditPopup>

{#if loaded}
    <!-- <div>
                <p class="uppercase text-gray-300">at a glance</p>
                <p class="text-slate-200 text-xl">API engine has handled 239,392 requests</p>
            </div> -->
    <div class="md:flex md:items-center md:justify-between">
        <p class="uppercase text-xs md:text-base text-gray-300 mx-2">
            Origins - {domain_info._id}
        </p>
        <div class="flex flex-1 justify-end">
            <button
                on:click={() => {
                    if (able_to_save === true) show_staged_changes = true;
                }}
                class="{saved_class} text-white md:px-3 md:py-1.5 md:mx-3 px-1 mx-1 rounded text-sm"
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
                href="/docs/buckets"
                class="uppercase text-indigo-200 hover:text-indigo-100 py-1.5 text-xs md:text-base mx-2"
                >documentation</a
            >
        </div>
    </div>
    <div>
        <div class="h-full">
            <div class="flex items-center justify-between py-5">
                <p class="mx-2 uppercase text-gray-300 text-xs md:text-base">
                    Origin settings
                </p>
                <div class="flex flex-1 justify-end">
                    <button
                        on:click={() => {
                            console.log(new_origin_origins);
                            new_origin_id = undefined;
                            new_origin_setting_popup = true;
                        }}
                        class="bg-fuchsia-500 text-white md:px-3 md:py-1.5 md:mx-3 px-1 mx-1 rounded text-sm"
                    >
                        + New origin setting
                    </button>
                </div>
            </div>

            <div class="pb-5">
                {#each origin_settings as origin_setting, index}
                    <div class="pt-5">
                        <div class="shadow bg-opacity-90 bg-amber-100">
                            <div class="px-4 py-5 sm:p-6">
                                <div
                                    class="sm:flex sm:flex-1 sm:w-full sm:justify-between"
                                >
                                    <div
                                        class="justify-start sm:flex sm:flex-1"
                                    >
                                        <h3
                                            class="text-base font-semibold leading-6 text-gray-900 py-0.5 pr-3 flex"
                                            id="api-strict-mode-enabled"
                                        >
                                            {origin_setting._id}
                                            {#if origin_setting.ssl}<svg
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
                                                        d="M16.5 10.5V6.75a4.5 4.5 0 10-9 0v3.75m-.75 11.25h10.5a2.25 2.25 0 002.25-2.25v-6.75a2.25 2.25 0 00-2.25-2.25H6.75a2.25 2.25 0 00-2.25 2.25v6.75a2.25 2.25 0 002.25 2.25z"
                                                    />
                                                </svg>
                                            {/if}
                                        </h3>
                                        <!--{#if origin_setting.ssl_specs.verified}
                                                    <span class="inline-flex flex-shrink-0 items-center rounded-full bg-blue-50 px-1.5 py-0.5 font-medium text-blue-700 ring-1 ring-inset ring-green-600/20">SSL issued</span>
                                                {:else}
                                                    <span class="inline-flex flex-shrink-0 items-center rounded-full bg-red-50 px-1.5 py-0.5 font-medium text-red-700 ring-1 ring-inset ring-red-600/20">SSL not issued</span>
                                                {/if}-->

                                        {#if origin_setting.changed.new_staged}
                                            <span
                                                class="inline-flex flex-shrink-0 items-center rounded-full bg-blue-50 px-1.5 py-0.5 font-medium text-blue-700 ring-1 ring-inset ring-green-600/20"
                                                >staged</span
                                            >
                                        {:else if origin_setting.changed.existed_modified}
                                            <span
                                                class="inline-flex flex-shrink-0 items-center rounded-full bg-yellow-50 px-1.5 py-0.5 font-medium text-yellow-700 ring-1 ring-inset ring-yellow-600/20"
                                                >modified</span
                                            >
                                        {:else if origin_setting.changed.being_deleted}
                                            <span
                                                class="inline-flex flex-shrink-0 items-center rounded-full bg-red-50 px-1.5 py-0.5 font-medium text-red-700 ring-1 ring-inset ring-red-600/20"
                                                >staged to be deleted</span
                                            >
                                        {/if}
                                    </div>
                                    <div
                                        class="justify-end flex flex-1 items-center"
                                    >
                                        <button
                                            on:click={() => {
                                                modifyOriginSettingPopup(index);
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
                                                deleteOriginSetting(
                                                    origin_setting._id,
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
                                    <div class="max-w-xl text-sm text-gray-700">
                                        <DisplayOriginSetting
                                            setting={origin_setting}
                                        />
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                {/each}
            </div>
        </div>
    </div>
{/if}
