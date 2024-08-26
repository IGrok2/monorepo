<script>
    import { page } from "$app/stores";

    import { toast } from "svelte-sonner";

    import { Button } from "$lib/components/ui/button";
    import * as Card from "$lib/components/ui/card";
    import * as Select from "$lib/components/ui/select";
    import { Input } from "$lib/components/ui/input";
    import { Label } from "$lib/components/ui/label";
    import { Separator } from "$lib/components/ui/separator";
    import * as RadioGroup from "$lib/components/ui/radio-group";
    import * as Accordion from "$lib/components/ui/accordion";

    import { GitBranch, X, Plus } from "lucide-svelte";

    import Spinner from "$lib/components/Spinner.svelte";
    import Regions from "../(components)/Regions.svelte";

    import World from "$lib/assets/world.svg";

    import { PUBLIC_API } from "$env/static/public";
    import { getCookie } from "$lib/utils/auth";

    import { goto } from "$app/navigation";

    const token = getCookie(`jwt`);

    let loading = true;

    let repo;

    let branches = [];

    const protocols = [
        { value: "HTTP", label: "HTTP" },
        { value: "TCP", label: "TCP" },
        //{ value: "UDP", label: "UDP" },
        //{ value: "PRIVATE", label: "Private" },
        //{ value: "NONE", label: "None" },
    ];

    let name = "App Name";
    let regions;
    let runtime = "node";
    let branch = {
        value: "main",
        label: " main",
        disabled: false,
    };
    let subdirectory = "";
    let port = 3000;
    let protocol = {
        value: "HTTP",
        label: "HTTP",
        disabled: false,
    };
    let variables = [{ key: "", value: "" }];

    async function fetchRepo() {
        const response = await fetch(
            `${PUBLIC_API}/@/github/repo/${$page.params.repo}`,
            {
                method: "GET",
                headers: new Headers({
                    "content-type": "application/json",
                    Authorization: token,
                }),
            },
        );

        let res = await response.json();
        console.log(res);

        if (res.success) {
            repo = res.repo;
            name = repo.name;

            repo.branches.forEach((branch) => {
                branches.push({
                    value: branch.name,
                    label: branch.name,
                });
            });

            loading = false;
        } else {
            toast.error("Error loading repo.");
        }
    }

    fetchRepo();

    async function createApp() {
        let regionsFormatted = [];

        for (const region of regions) {
            if (region.selected) {
                regionsFormatted.push({
                    name: region.id,
                    replicas: {
                        min: 1,
                        max: 1,
                    },
                });
            }
        }

        toast.loading("Creating app...");
        const response = await fetch(
            `${PUBLIC_API}/@/project/${$page.params.project}/app-admin/create`,
            {
                method: "POST",
                headers: new Headers({
                    "content-type": "application/json",
                    Authorization: token,
                }),
                body: JSON.stringify({
                    provider: "github",
                    name: name,
                    runtime: runtime,
                    branch: branch.value,
                    subdirectory: subdirectory,
                    port: port,
                    protocol: protocol.value,
                    variables: variables,
                    repo_id: repo.id,
                    regions: regionsFormatted,
                    quota: {
                        cpu: 10,
                        memory: 512,
                    },
                    volumes: [],
                }),
            },
        );

        const res = await response.json();
        console.log(res);

        if (response.status === 200) {
            toast.success(res.message);
            goto(
                `/i/dash/project/${$page.params.project}/apps/${res.container_id}`,
            );
        } else {
            toast.error("App creation failed");
        }

        return res.installs;
    }
</script>

<Card.Root class="my-8">
    {#if loading}
        <div class="my-4">
            <Spinner />
        </div>
    {:else}
        <Card.Header>
            <Card.Title>Define App Details</Card.Title>
            <Card.Description
                ><Button variant="link" href={repo.html_url}
                    ><GitBranch class="w-4 h-4 mr-2" />{repo.full_name}</Button
                ></Card.Description
            >
        </Card.Header>
        <Card.Content>
            <Accordion.Root>
                <Accordion.Item value="name">
                    <Accordion.Trigger>Name</Accordion.Trigger>
                    <Accordion.Content class="px-1">
                        Enter the name you'd like to use for this Packetware
                        App.
                        <Input
                            type="text"
                            name="name"
                            id="name"
                            bind:value={name}
                        />
                    </Accordion.Content>
                </Accordion.Item>
                <Accordion.Item value="source">
                    <Accordion.Trigger>Source</Accordion.Trigger>
                    <Accordion.Content class="px-1">
                        <!-- Branch -->
                        <div>
                            <h2 class="text-base font-semibold leading-7">
                                Branch
                            </h2>
                            <p
                                class="mt-1 text-sm leading-6 text-muted-foreground"
                            >
                                What Git branch should we listen for pushes on?
                            </p>

                            <Select.Root bind:selected={branch}>
                                <Select.Trigger>
                                    <Select.Value />
                                </Select.Trigger>
                                <Select.Content>
                                    {#each branches as branch}
                                        <Select.Item
                                            value={branch.value}
                                            label={branch.label}
                                            >{branch.label}</Select.Item
                                        >
                                    {/each}
                                </Select.Content>
                            </Select.Root>
                        </div>
                        <!-- Subdirectory -->
                        <div>
                            <h2 class="text-base font-semibold leading-7">
                                App Directory
                            </h2>
                            <p
                                class="mt-1 text-sm leading-6 text-muted-foreground"
                            >
                                Leave this blank if you app is at the root of
                                the repository. This allows you to only deploy a
                                subdirectory of your repo. Very helpful for
                                monorepos.
                            </p>

                            <Input
                                type="text"
                                name="subdirectory"
                                id="subdirectory"
                                placeholder="/"
                                bind:value={subdirectory}
                            />
                        </div>
                    </Accordion.Content>
                </Accordion.Item>
                <Accordion.Item value="networking">
                    <Accordion.Trigger>Networking</Accordion.Trigger>
                    <Accordion.Content class="px-1">
                        <!-- App Port -->
                        <div>
                            <h2 class="text-base font-semibold leading-7">
                                App Port
                            </h2>
                            <p
                                class="mt-1 text-sm leading-6 text-muted-foreground"
                            >
                                What port should your app recieve traffic on?
                            </p>

                            <div class="mt-2">
                                <Input
                                    type="int"
                                    name="port"
                                    id="port"
                                    bind:value={port}
                                />
                            </div>
                        </div>

                        <!-- Protocol -->
                        <div>
                            <h2 class="text-base font-semibold leading-7">
                                Protocol
                            </h2>
                            <p
                                class="mt-1 text-sm leading-6 text-muted-foreground"
                            >
                                What protocol should we expose your app with?
                            </p>

                            <Select.Root bind:selected={protocol} portal={null}>
                                <Select.Trigger>
                                    <Select.Value
                                        placeholder="Select a protocol"
                                    />
                                </Select.Trigger>
                                <Select.Content>
                                    <Select.Group>
                                        <!-- <Select.Label>Protocols</Select.Label> -->
                                        {#each protocols as protocol}
                                            <Select.Item
                                                value={protocol.value}
                                                label={protocol.label}
                                                >{protocol.label}</Select.Item
                                            >
                                        {/each}
                                    </Select.Group>
                                </Select.Content>
                                <Select.Input name="protocol" />
                            </Select.Root>
                        </div>
                    </Accordion.Content>
                </Accordion.Item>
                <Accordion.Item value="environment">
                    <Accordion.Trigger>Environment Variables</Accordion.Trigger>
                    <Accordion.Content class="px-1">
                        <!-- Environment Variables -->
                        <div>
                            <h2 class="text-base font-semibold leading-7">
                                Environment Variables
                            </h2>
                            <p
                                class="mt-1 text-sm leading-6 text-muted-foreground"
                            >
                                What environment variables should be used for
                                this app? .env
                            </p>

                            {#each variables as variable, index}
                                <div class="my-2 flex space-x-4">
                                    <Input
                                        type="name"
                                        placeholder="VARIABLE_NAME"
                                        bind:value={variable.key}
                                        class=""
                                    />
                                    <Input
                                        type="name"
                                        placeholder="Key..."
                                        bind:value={variable.value}
                                        class=""
                                    />
                                    <Button
                                        on:click={() => {
                                            if (variables.length > 1) {
                                                variables.splice(index, 1);
                                                variables = variables;
                                            }
                                        }}><X class="w-4 h-4" /></Button
                                    >
                                </div>
                            {/each}
                            <Button
                                on:click={() => {
                                    variables = [
                                        ...variables,
                                        {
                                            key: "",
                                            value: "",
                                        },
                                    ];
                                }}><Plus class="w-4 h-4 mr-2" />Add More</Button
                            >
                        </div>
                    </Accordion.Content>
                </Accordion.Item>
                <Accordion.Item value="regions">
                    <Accordion.Trigger>Regions</Accordion.Trigger>
                    <Accordion.Content class="px-1">
                        <!-- Regions -->
                        <div>
                            <p
                                class="mt-1 text-sm leading-6 text-muted-foreground"
                            >
                                Where should the points of presence for this app
                                be? We automatically load balancer to the
                                nearest app hosting region from the Packetware
                                Load Balancer when a request is made.
                            </p>
                        </div>
                        <Regions bind:regions />
                    </Accordion.Content>
                </Accordion.Item>
            </Accordion.Root>
        </Card.Content>
        <Card.Footer class="flex justify-end">
            <Button on:click={createApp}>Create App</Button>
        </Card.Footer>
    {/if}
</Card.Root>
