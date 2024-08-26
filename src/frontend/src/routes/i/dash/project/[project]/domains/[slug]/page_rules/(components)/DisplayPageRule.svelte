<script>
    import Action from "./Action.svelte";

    import * as Card from "$lib/components/ui/card";
    import { Button } from "$lib/components/ui/button";
    import { Badge } from "$lib/components/ui/badge";
    import { Label } from "$lib/components/ui/label";
    import { Switch } from "$lib/components/ui/switch";
    import { Input } from "$lib/components/ui/input";
    import * as Select from "$lib/components/ui/select";
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu";

    import {Pencil, Trash2, Save, Plus, ChevronDown} from "lucide-svelte";
    import {beautiful_date} from "$lib/utils/beautiful_date.js";
    import {onMount} from "svelte";
    import {toast} from "svelte-sonner";

    export let rule;

    export let domain_info;

    // in order that they are processed
    const trustbustingOptions = [
        "Block",
        "Smart Challenge",
        // "Captcha",
        "Ratelimit",
        "Redirect",
        "Skip Human Engine",
        "Change caching settings",
        "Use Origin Setting",
    ];

    let edit = true;
    let loaded = false;

    let trustbustingSelected = [];

    function convertSelectedToApi() {
        rule.action.trustbusting = [];

        for (let i = 0; i < trustbustingSelected.length; i++) {
            rule.action.trustbusting.push(trustbustingSelected[i].value);
        }
    }

    $: trustbustingSelected, convertSelectedToApi();

    for (const value of rule.action.trustbusting) {
        if (value === "Cache") {
            trustbustingSelected.push({disabled: false, value: "Cache", label: "Change caching settings"});
        } else {
            trustbustingSelected.push({disabled: false, value: value, label: value});
        }
    };
</script>

{#if edit === false}
    <Card.Root>
        <Card.Header>
            <div class="flex justify-between text-muted-foreground">
                {#if rule.enabled}
                    <Badge>Enabled</Badge>
                {:else}
                    <Badge variant="destructive">Disabled</Badge>
                {/if}
                <div>{rule.order}</div>
            </div>
            <h1 class="text-lg">
                Triggers - {rule.trigger.trigger_requirement}
            </h1>
            {#each rule.trigger.match_type as match_type}
                {#each Object.entries(match_type.trigger) as [key, value]}
                    <p class="text-sm text-muted-foreground">
                        {#if match_type.inversed === true}Inversed
                        {/if}{match_type.match_type}
                        {key}: {value}
                    </p>
                {/each}
            {/each}
        </Card.Header>
        <Card.Content>
            Actions - {rule.action.redirect}
            <Card.Description>
                {#each Object.entries(rule.action.trustbusting) as [key, value]}
                    {#if Number(key) > 0},
                    {/if}{value}
                {/each}
            </Card.Description>
        </Card.Content>
        <Card.Footer>
            <div class="flex w-full justify-end space-x-2">
                <Button
                    on:click={() => {
                        edit = true;
                    }}><Pencil class="w-4 h-4 mr-1" />Edit</Button
                >
                <Button variant="destructive"
                    ><Trash2 class="w-4 h-4 mr-1" />Delete</Button
                >
            </div>
        </Card.Footer>
    </Card.Root>
{:else}
    <Card.Root>
        <Card.Header>
            <div class="flex justify-between text-muted-foreground">
                <div>
                    Enabled <br><Switch class="mt-1.5" bind:checked={rule.enabled} />
                </div>
                <div class="text-center md:text-left">
                    Processing order <Input class="mt-1.5" placeholder="Order Number" bind:value={rule.order} />
                </div>
            </div>
            <Card.Title>Rule</Card.Title>
            <div class="md:flex justify-between items-center py-2">
                <div class="flex items-center">
                    <p class="pr-1.5">
                        Trigger Requirement
                    </p>
                    <!-- the match type -->
                    <Select.Root class="w-[180px] mr-1.5" bind:selected={rule.trigger.trigger_requirement}>
                        <Select.Trigger class="w-[180px]">
                            <Select.Value placeholder={rule.trigger.trigger_requirement ? rule.trigger.trigger_requirement : "(not set)"}/>
                        </Select.Trigger>
                        <Select.Content class="w-[180px]">
                            <Select.Item value="All">
                                All
                            </Select.Item>
                            <Select.Item value="Multiple">
                                Multiple
                            </Select.Item>
                            <Select.Item value="One">
                                One
                            </Select.Item>
                        </Select.Content>
                    </Select.Root>
                    {#if rule.trigger.trigger_requirement?.value === "Multiple"}
                        ,
                        <div class="pl-1.5">
                            <Input
                                    type="number"
                                    id="default_ttl"
                                    name="default_ttl"
                                    max="{rule.trigger.match_type.length}"
                                    placeholder={rule.trigger.trigger_requirement_multiple ? rule.trigger.trigger_requirement_multiple : "(not set)"}
                                    bind:value={rule.trigger.trigger_requirement_multiple}
                            />
                        </div>
                        {#if rule.trigger.trigger_requirement_multiple > rule.trigger.match_type.length}
                            <p class="pl-1.5 flex-none text-red-500">out of {rule.trigger.match_type.length} ({rule.trigger.trigger_requirement_multiple && rule.trigger.match_type.length ? Math.round((rule.trigger.trigger_requirement_multiple / rule.trigger.match_type.length) * 100) : 0}%)</p>
                        {:else}
                            <p class="pl-1.5 flex-none">out of {rule.trigger.match_type.length} ({rule.trigger.trigger_requirement_multiple && rule.trigger.match_type.length ? Math.round((rule.trigger.trigger_requirement_multiple / rule.trigger.match_type.length) * 100) : 0}%)</p>
                        {/if}
                    {/if}
                </div>
                <Button on:click={() => {
                        console.log(rule.trigger.match_type)
                        // on-click, add a new match type at the front of the rule
                        rule.trigger.match_type.unshift({
                            trigger: {
                                key: "path",
                                value: "",
                            },
                            match_type: "Exact",
                            inversed: false,
                            required: false,
                            editing: true,
                        });

                        rule.trigger.match_type = rule.trigger.match_type;

                        rule = rule;
                    }}>
                    <Plus />
                </Button>
            </div>
            <div>
                <p class="text-sm text-muted-foreground">
                    How to match the triggers. All means that every trigger must match for actions to complete. Multiple means
                    a specific number of triggers must match. One means that only one trigger must match. Multiple and One can also have
                    a trigger that is specifically required to match.
                </p>

                <p class="pt-4 text-lg text-primary">
                    Triggers
                </p>
            </div>
            <ul
                    role="list"
                    class="md:grid grid-cols-3 gap-x-2 gap-y-2"
            >
                {#each rule.trigger.match_type as match_type}
                        {#if !match_type.editing}
                            <Card.Root>
                                <Card.Header>
                                    <div class="flex w-full justify-between">
                                        <p class="text-sm text-muted-foreground">
                                            <!-- the qualifiers -->
                                            matches {match_type.match_type === "Exact" ? "exactly" : "partially"}{match_type.inversed ? ", whitelists (inversed)" : ""}{match_type.required ? ", specifically required" : ""}
                                        </p>

                                        <div class="flex">
                                            <Button
                                                    variant="ghost"
                                                    on:click={() => {
                                                        // set match_type to editing
                                                        match_type.editing = true;
                                                }}
                                            >
                                                <Pencil size=16 strokeWidth=2 />
                                            </Button>
                                            <Button
                                                    variant="ghost"
                                                    size=12
                                                    strokeWidth=1
                                                    on:click={async () => {
                                                            rule.trigger.match_type = rule.trigger.match_type.filter(item => item !== match_type);
                                                        }}
                                            >
                                                <Trash2 size=16 strokeWidth=2 color="red" />
                                            </Button>
                                        </div>
                                    </div>
                                </Card.Header>
                                <Card.Content>
                                    {#if match_type.trigger.key === "any" || match_type.trigger.key?.value?.toLowerCase() === "any"}
                                        <p>Rule will trigger on any action</p>
                                    {:else}
                                        {#if match_type.trigger.key?.label}
                                            <p>{match_type.trigger.key.label.toLowerCase()} : {match_type.trigger.value}</p>
                                        {:else}
                                            <p>{match_type.trigger.key}: {match_type.trigger.value}</p>
                                        {/if}
                                    {/if}
                                </Card.Content>
                            </Card.Root>
                        {:else}
                            <!-- edit the match type -->
                            <Card.Root>
                                <Card.Header>
                                    <div class="flex w-full items-center">
                                        <p class="items-baseline">Match...</p>

                                        <div class="flex w-full justify-end">
                                            <Button
                                                    variant="ghost"
                                                    on:click={() => {
                                                        // set match_type to editing
                                                        // make sure that the match type is valid
                                                        if (!match_type.trigger.key || !match_type.trigger.value) {
                                                            toast.error("Please fill out the key and value fields")
                                                        } else {
                                                            match_type.editing = false;
                                                        }
                                                }}
                                            >
                                                <Save size=16 strokeWidth=2 />
                                            </Button>
                                            <Button
                                                    variant="ghost"
                                                    size=12
                                                    strokeWidth=1
                                                    on:click={async () => {
                                                            rule.trigger.match_type = rule.trigger.match_type.filter(item => item !== match_type);
                                                        }}
                                            >
                                                <Trash2 size=16 strokeWidth=2 color="red" />
                                            </Button>
                                        </div>
                                    </div>
                                </Card.Header>
                                <Card.Content>
                                    <!-- the match type -->
                                    <Select.Root class="min-w-full mr-1.5" bind:selected={match_type.trigger.key}>
                                        <Select.Trigger class="">
                                            <Select.Value placeholder={match_type.trigger.key ? `current: ${match_type.trigger.key}` : "(not set)"}/>
                                        </Select.Trigger>
                                        <Select.Content class="min-w-full">
                                            <Select.Item value="path">
                                                Path
                                            </Select.Item>
                                            <Select.Item value="ip">
                                                IP Address
                                            </Select.Item>
                                            <Select.Item value="query">
                                                Query string
                                            </Select.Item>
                                            <Select.Item value="asn">
                                                ASN
                                            </Select.Item>
                                            <Select.Item value="country">
                                                Country
                                            </Select.Item>
                                            <Select.Item value="continent">
                                                Continent
                                            </Select.Item>
                                            <Select.Item value="host">
                                                Host
                                            </Select.Item>
                                            <Select.Item value="method">
                                                Method
                                            </Select.Item>
                                            <Select.Item value="user_agent">
                                                User agent
                                            </Select.Item>
                                            <Select.Item value="cookie">
                                                Cookie
                                            </Select.Item>
                                            <Select.Item value="referrer">
                                                Referrer
                                            </Select.Item>
                                            <Select.Item value="any">
                                                Any
                                            </Select.Item>
                                        </Select.Content>
                                    </Select.Root>

                                    <!-- change these labels to allow for either -->
                                    <div class="pt-3">
                                    <!-- where data can be entered -->
                                        {#if match_type.trigger.key?.value === "path" || match_type.trigger.key === "path"}
                                            <Label class="text-sm text-muted-foreground" for="path">with value</Label>
                                            <Input
                                                    label="Path"
                                                    placeholder={match_type.trigger.value ? `change from ${match_type.trigger.value}` : "/packetware/best"}
                                                    bind:value={match_type.trigger.value}
                                            />
                                        {/if}
                                        {#if match_type.trigger.key?.value === "ip" || match_type.trigger.key === "ip"}
                                            <Label for="path">IP Address</Label>
                                            <Input
                                                    placeholder={match_type.trigger.value ? `change from ${match_type.trigger.value}` : "1.1.1.1"}
                                                    bind:value={match_type.trigger.value}
                                            />
                                        {/if}
                                        {#if match_type.trigger.key?.value === "query" || match_type.trigger.key === "query"}
                                            <Input
                                                    placeholder={match_type.trigger.value ? `change from ${match_type.trigger.value}` : "?hello=world"}
                                                    bind:value={match_type.trigger.value}
                                            />
                                        {/if}
                                        {#if match_type.trigger.key?.value === "asn" || match_type.trigger.key === "asn"}
                                            <Input
                                                    placeholder={match_type.trigger.value ? `change from ${match_type.trigger.value}` : "AS400495"}
                                                    bind:value={match_type.trigger.value}
                                            />
                                        {/if}
                                        {#if match_type.trigger.key?.value === "country" || match_type.trigger.key === "country"}
                                            <Input
                                                    placeholder={match_type.trigger.value ? `change from ${match_type.trigger.value}` : "CN"}
                                                    bind:value={match_type.trigger.value}
                                            />
                                        {/if}
                                        {#if match_type.trigger.key?.value === "continent" || match_type.trigger.key === "continent"}
                                            <Input
                                                    placeholder={match_type.trigger.value ? `change from ${match_type.trigger.value}` : "NA"}
                                                    bind:value={match_type.trigger.value}
                                            />
                                        {/if}
                                        {#if match_type.trigger.key?.value === "host" || match_type.trigger.key === "host"}
                                            <Input
                                                    placeholder={match_type.trigger.value ? `change from ${match_type.trigger.value}` : "api." + domain_info.domain}
                                                    bind:value={match_type.trigger.value}
                                            />
                                        {/if}
                                        {#if match_type.trigger.key?.value === "method" || match_type.trigger.key === "method"}
                                            <Input
                                                    placeholder={match_type.trigger.value ? `change from ${match_type.trigger.value}` : "GET"}
                                                    bind:value={match_type.trigger.value}
                                            />
                                        {/if}
                                        {#if match_type.trigger.key?.value === "user_agent" || match_type.trigger.key === "user_agent"}
                                            <Input
                                                    placeholder={match_type.trigger.value ? `change from ${match_type.trigger.value}` : "Chrome"}
                                                    bind:value={match_type.trigger.value}
                                            />
                                        {/if}
                                        {#if match_type.trigger.key?.value === "cookie" || match_type.trigger.key === "cookie"}
                                            <Input
                                                    placeholder={match_type.trigger.value ? `change from ${match_type.trigger.value}` : "Authentication"}
                                                    bind:value={match_type.trigger.value}
                                            />
                                        {/if}
                                        {#if match_type.trigger.key?.value === "referrer" || match_type.trigger.key === "referrer"}
                                            <Input
                                                    placeholder={match_type.trigger.value ? `change from ${match_type.trigger.value}` : "google.com"}
                                                    bind:value={match_type.trigger.value}
                                            />
                                        {/if}
                                        {#if match_type.trigger.key?.value === "any" || match_type.trigger.key === "any"}
                                            <p>Rule will trigger on any action</p>
                                        {/if}
                                    </div>

                                    <div class="pt-3">
                                        <p class="pb-1.5">And match...</p>

                                        <!-- the match type -->
                                        <Select.Root bind:selected={match_type.match_type}>
                                            <Select.Trigger class="">
                                                <Select.Value placeholder={match_type.match_type ? match_type.match_type : "Match type"}/>
                                            </Select.Trigger>
                                            <Select.Content>
                                                <Select.Item value="Exact">
                                                    Exactly
                                                </Select.Item>
                                                <Select.Item value="Contains">
                                                    If it contains
                                                </Select.Item>
                                                <Select.Item value="StartsWith">
                                                    If it starts with
                                                </Select.Item>
                                            </Select.Content>
                                        </Select.Root>

                                    </div>

                                    <div class="flex items-center w-full justify-center">
                                        <div class="w-1/2 flex items-center">
                                            <!-- the inversed switch -->
                                            <Label class="pr-1.5 pb-0.5">Inversed</Label>
                                            <Switch class="mb-0.5 ml-1.5" bind:checked={match_type.inversed} />
                                        </div>
                                        <p class="text-center p-2 text-sm text-muted-foreground">flip the result of the comparison</p>
                                    </div>

                                    {#if rule.trigger.trigger_requirement !== "All" && rule.trigger.trigger_requirement.value !== "All"}
                                        <div class="flex items-center w-full justify-center">
                                            <div class="w-1/2 flex items-center">
                                                <!-- the inversed switch -->
                                                <Label class="pr-1.5 pb-0.5">Require</Label>
                                                <Switch class="mb-0.5 ml-1.5" bind:checked={match_type.require} />
                                            </div>
                                            <p class="text-center p-2 text-sm text-muted-foreground">specifically require this matches</p>
                                        </div>
                                    {/if}
                                </Card.Content>
                            </Card.Root>
                        {/if}
                {/each}
            </ul>
        </Card.Header>
        <Card.Content>
            <Card.Description>
                <h1 class="text-lg text-primary">
                    Actions
                </h1>
                <h2 class="pb-2">
                    What to do when the rule is triggered
                </h2>
                <Select.Root bind:selected={trustbustingSelected} multiple={true}>
                    <Select.Trigger class="">
                        <Select.Value placeholder="Actions"/>
                    </Select.Trigger>
                    <Select.Content>
                        {#each trustbustingOptions as option}
                            {#if option === "Change caching settings"}
                                <Select.Item value="Cache" label="Change caching settings" on:click={() => {
                                    trustbustingSelected = trustbustingSelected
                                    console.log(`trustbustingSelected: ${JSON.stringify(trustbustingSelected)}`);
                                }}>
                                    Change caching settings
                                </Select.Item>
                            {:else}
                                <Select.Item value={option} label={option} on:click={() => {
                                    console.log(`option: ${option}`);
                                    trustbustingSelected = trustbustingSelected
                                    console.log(`trustbustingSelected: ${JSON.stringify(trustbustingSelected)}`);
                                }}>
                                    {option}
                                </Select.Item>
                            {/if}
                        {/each}
                    </Select.Content>
                </Select.Root>
            </Card.Description>

            {#if Array.isArray(trustbustingSelected) && trustbustingSelected.some(item => item.value === "Ratelimit")}
                <div class="py-2">
                    <Label for="buckets">Ratelimit bucket</Label>
                    <Select.Root bind:selected={rule.action.bucket_name}>
                        <Select.Trigger class="">
                            <Select.Value placeholder="Choose a bucket"/>
                        </Select.Trigger>
                        <Select.Content>
                            {#each domain_info.ratelimit_buckets as bucket}
                                <Select.Item value={bucket._id} on:click={() => {
                                    trustbustingSelected = trustbustingSelected
                                    console.log(`trustbustingSelected: ${JSON.stringify(trustbustingSelected)}`);
                                }}>
                                    {bucket._id} - {bucket.threshold} reqs per {bucket.secs} secs
                                </Select.Item>
                            {/each}
                            <Select.Separator />
                            <Button
                                    class="m-1 bg-muted text-muted-foreground justify-center"
                                    href="./buckets"
                                    target="_blank">
                                Create new bucket
                            </Button>
                        </Select.Content>
                    </Select.Root>
                </div>
            {/if}

            {#if Array.isArray(trustbustingSelected) && trustbustingSelected.some(item => item.label === "Change caching settings")}
                <div class="py-2">
                    <div class="flex w-full">
                        <div class="w-1/2 pr-2">
                            <Label for="caching">Cache settings</Label>
                            <Select.Root bind:selected={rule.action.cache_level}>
                                <Select.Trigger class="">
                                    <Select.Value
                                            placeholder={rule.action.cache_level ? rule.action.cache_level : "Change from " + domain_info.caching_settings.default_cache_level}
                                    />
                                </Select.Trigger>
                                <Select.Content>
                                    <Select.Item value={"None"}>
                                        None
                                    </Select.Item>
                                    <Select.Item value={"Standard"}>
                                        Standard
                                    </Select.Item>
                                    <Select.Item value={"IgnoreQueryString"}>
                                        IgnoreQueryString
                                    </Select.Item>
                                    <Select.Item value={"Aggressive"}>
                                        Aggressive
                                    </Select.Item>
                                </Select.Content>
                            </Select.Root>
                        </div>

                        <div class="w-1/2 pl-2">
                            <Label for="default_ttl">Time-to-live (in seconds)</Label>
                            <Input
                                    type="number"
                                    id="default_ttl"
                                    name="default_ttl"
                                    placeholder={rule.action.cache_level === "None" ? "Caching disabled" : rule.action.cache_level_ttl ? rule.action.cache_level_ttl : "Change from " + domain_info.caching_settings.default_ttl}
                                    bind:value={rule.action.cache_level_ttl}
                            />
                            <p class="pt-1">{beautiful_date(rule.action.cache_level_ttl)}</p>
                        </div>
                    </div>
                </div>
            {/if}

            {#if Array.isArray(trustbustingSelected) && trustbustingSelected.some(item => item.value === "Redirect")}
                <div class="py-2">
                    <Label for="redirect">Redirect URL</Label>
                    <Input bind:value={rule.action.redirect} />
                </div>
            {/if}

            {#if Array.isArray(trustbustingSelected) && trustbustingSelected.some(item => item.value === "Use Origin Setting")}
                <div class="py-2">
                    <Label for="buckets">Origin setting</Label>
                    <Select.Root bind:selected={rule.action.backend_host}>
                        <Select.Trigger class="">
                            <Select.Value placeholder="Choose an origin to send traffic"/>
                        </Select.Trigger>
                        <Select.Content>
                            {#each domain_info.origin_settings as setting}
                                <Select.Item value={setting._id} on:click={() => {
                                    trustbustingSelected = trustbustingSelected
                                    console.log(`trustbustingSelected: ${JSON.stringify(trustbustingSelected)}`);
                                }}>
                                    {setting._id} - with {setting.origins.length} origin(s)
                                </Select.Item>
                            {/each}
                            <Select.Separator />
                            <Button
                                    class="m-1 bg-muted text-muted-foreground justify-center"
                                    href="./origins"
                                    target="_blank">
                                Create new origin
                            </Button>
                        </Select.Content>
                    </Select.Root>
                </div>
            {/if}
        </Card.Content>
    </Card.Root>
{/if}
