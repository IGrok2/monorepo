<script>
    import { page } from "$app/stores";

    import APIClient from "$lib/utils/api";

    import { toast } from "svelte-sonner";

    import { Button } from "$lib/components/ui/button";
    import * as Card from "$lib/components/ui/card";
    import * as Select from "$lib/components/ui/select";
    import { Input } from "$lib/components/ui/input";
    import { Label } from "$lib/components/ui/label";
    import { Separator } from "$lib/components/ui/separator";
    import * as Accordion from "$lib/components/ui/accordion";
    import * as RadioGroup from "$lib/components/ui/radio-group";

    import Spinner from "$lib/components/Spinner.svelte";
    import Info from "$lib/components/Info.svelte";

    import { Progress, Variables } from "./(components)";

    import {
        GitRepo,
        ImageSource,
        Networking,
        Builder,
    } from "$lib/components/app";

    import Docker from "$lib/assets/Docker.webp";
    import Github from "$lib/assets/Github.png";

    let steps = ["Source", "Builder", "Variables", "Networking", "Details"];
    let stepNumber = 4;

    let channel = "GITHUB";

    let public_repo;
    let repo_url;
    let namespace;
    let repo;
    let branch;
    let subdirectory = "";

    let builder = "NIXPACKS";

    let variables = [{ name: "", value: "" }];

    let ports = [
        {
            name: "Node",
            number: 3000,
            protocol: { value: 6, label: "TCP" },
            http: true,
        },
    ];
    let network_type = "PUBLIC";

    let regions = ["dallas"];

    let quota = { cpu: 2, memory: 1, network: 10 };

    let slug = "";

    async function createApp() {
        let body = {
            slug: slug,
            channel: channel,
            git_repo: {
                public: public_repo,
                repo_id: repo.value,
                branch: branch.value,
                subdirectory: subdirectory,
                git_namespace_id: namespace.value,
            },
            commands: {
                install: "",
                build: "",
                run: "",
            },
            variables: variables,
            networking: {
                public: network_type === "PUBLIC",
                ports: ports.map((port) => ({
                    name: port.name,
                    port: port.number,
                    l4_protocol: 6,
                    l7_protocol: "HTTP",
                })),
            },
            regions: regions,
            quota: quota,
        };

        try {
            let res = await APIClient.post(
                `/project/${$page.params.project}/environment/${$page.params.environment}/app`,
                body,
            );
            toast.success(res.data.data.message);
            // Wait for 1 second before reloading the page
            setTimeout(() => {
                document.location = `/i/dash/project/${$page.params.project}/${$page.params.environment}/${slug}`;
            }, 1000);
        } catch (err) {
            console.log(err);
            toast.error(err.response.data.data.message);
        }
    }
</script>

<div class="my-8 space-y-2">
    <div class="mb-4 pb-4 border-b-2 border-dotted border-muted-foreground">
        <Progress bind:steps bind:stepNumber />
    </div>
    {#if steps[stepNumber] === "Source"}
        <RadioGroup.Root bind:value={channel}>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <Card.Root>
                    <div class="flex h-full items-center">
                        <div class="ml-4 h-full flex items-center space-x-4">
                            <RadioGroup.Item value="GITHUB" id="GITHUB" />
                            <Separator orientation="vertical" />
                        </div>
                        <img
                            src={Github}
                            alt="Github logo"
                            class="ml-4 h-12 w-12 dark:filter dark:invert dark:brightness-0"
                        />
                        <Card.Header>
                            <Card.Title>Code Repository</Card.Title>
                            <Card.Description
                                >Pull and build your code into a app from
                                Github.</Card.Description
                            >
                        </Card.Header>
                    </div>
                </Card.Root>
                <Card.Root>
                    <div class="flex h-full items-center">
                        <div class="ml-4 h-full flex items-center space-x-4">
                            <!-- Radio Button -->
                            <RadioGroup.Item value="IMAGE" id="IMAGE" />
                            <!-- Visible Vertical Separator -->
                            <Separator orientation="vertical" />
                        </div>
                        <img
                            src={Docker}
                            alt="Docker logo"
                            class="ml-4 h-12 w-12 dark:filter dark:invert dark:brightness-0"
                        />
                        <Card.Header>
                            <Card.Title>Container Image</Card.Title>
                            <Card.Description
                                >Pull a artifact from a repository and we will
                                run it.</Card.Description
                            >
                        </Card.Header>
                    </div>
                </Card.Root>
            </div>
        </RadioGroup.Root>
        {#if channel === "GITHUB"}
            <GitRepo
                bind:public_repo
                bind:repo_url
                bind:selectedNamespace={namespace}
                bind:selectedRepo={repo}
                bind:selectedBranch={branch}
                bind:subdirectory
            />
            <div class="flex justify-end">
                <Button on:click={() => (stepNumber += 1)}>Next Step</Button>
            </div>
        {:else if channel === "IMAGE"}
            <!-- <ImageSource bind:channel bind:image_url />-->
            <div class="flex justify-end">
                <Button on:click={() => (stepNumber += 2)}>Next Step</Button>
            </div>
        {/if}
    {:else if steps[stepNumber] === "Builder"}
        <Builder bind:builder />
        <div class="flex justify-end">
            <Button on:click={() => (stepNumber += 1)}>Next Step</Button>
        </div>
    {:else if steps[stepNumber] === "Variables"}
        <Variables bind:variables />
        <div class="flex justify-end">
            <Button on:click={() => (stepNumber += 1)}>Next Step</Button>
        </div>
    {:else if steps[stepNumber] === "Networking"}
        <Networking bind:network_type bind:ports />
        <div class="flex justify-end">
            <Button on:click={() => (stepNumber += 1)}>Next Step</Button>
        </div>
    {:else if steps[stepNumber] === "Details"}
        <div class="space-y-2">
            <div class="flex items-center">
                <Label for="Slug">App Name</Label>
                <Info
                    content="The name can only contain lowercase characters and dashes."
                />
            </div>
            <Input
                type="string"
                name="slug"
                id="slug"
                bind:value={slug}
                placeholder="app-name-slug"
            />
        </div>
        <div class="flex justify-end">
            <Button
                on:click={async () => {
                    if (channel === "GITHUB") {
                        await createApp();
                    } else if (channel === "IMAGE") {
                    }
                }}>Create App</Button
            >
        </div>
    {/if}
</div>
