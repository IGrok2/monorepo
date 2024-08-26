<script>
    import { page } from "$app/stores";

    import { toast } from "svelte-sonner";

    import * as Card from "$lib/components/ui/card";
    import { Button } from "$lib/components/ui/button";
    import { Separator } from "$lib/components/ui/separator";
    import { Label } from "$lib/components/ui/label";
    import { Input } from "$lib/components/ui/input";
    import * as Select from "$lib/components/ui/select";
    import * as AlertDialog from "$lib/components/ui/alert-dialog";
    import { Switch } from "$lib/components/ui/switch";
    import * as Drawer from "$lib/components/ui/drawer";

    import Spinner from "$lib/components/Spinner.svelte";

    import { PUBLIC_API } from "$env/static/public";
    import { getCookie } from "$lib/utils/auth";

    import { Copy, GitBranch, Github } from "lucide-svelte";

    import { updateApp, deleteApp } from "$lib/utils/apps";
    import { dataTool } from "echarts";

    const token = getCookie(`jwt`);

    /** @type {import('./$types').PageData} */
    export let data;

    let loading = true;

    let old_container = { ...data.container };

    let container = data.container;
    let repo;

    let branches = [];
    let branch = {
        value: container.github.branch,
        label: container.github.branch,
    };

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

    let showDeleteDialog = false;

    function copyToClipboard(text) {
        navigator.clipboard
            .writeText(text)
            .then(() => {
                toast.success("Copying to clipboard was successful!");
            })
            .catch((err) => {
                toast.error("Could not copy text: ", err);
            });
    }

    async function handleUpdate() {
        console.log("Saving app changes.");
        toast.loading("Saving app changes.");
        let res = await updateApp(
            container._id,
            container.name,
            container.app_port,
            branch.value,
            container.github.subdirectory,
            container.auto_deploy,
        );
        console.log(res);

        if (res.success === true) {
            toast.success(res.message);
        } else {
            toast.error(res.message);
        }
    }

    async function handleDelete() {
        let res = await deleteApp(container._id);
        console.log(res);

        if (res.success === true) {
            toast.success(res.message);
        } else {
            toast.error(res.message);
        }
    }
</script>

{#if loading}
    <Spinner />
{:else}
    <div class="space-y-4">
        <Card.Root>
            <Card.Header>
                <Card.Title>General Settings</Card.Title>
                <Card.Description
                    >Generic settings that all applications utilize.</Card.Description
                >
            </Card.Header>
            <Card.Content class="space-y-4">
                <div>
                    <Label for="provider">Name of Application</Label>
                    <Input type="name" bind:value={container.name} />
                    <p class="text-sm text-muted-foreground">
                        This is the name used for your serive or app.
                    </p>
                </div>
                <div>
                    <Label for="provider">App Port</Label>
                    <Input type="number" bind:value={container.app_port} />
                    <p class="text-sm text-muted-foreground">
                        What port does your application socket bind to?
                    </p>
                </div>
            </Card.Content>
        </Card.Root>

        <Card.Root>
            <Card.Header>
                <Card.Title>Source</Card.Title>
                <Card.Description
                    >What settings should we use for getting your application</Card.Description
                >
            </Card.Header>
            <Card.Content class="space-y-4">
                <div>
                    <Label for="provider" class="flex items-center"
                        ><Github class="w-4 h-4 mr-1" />Repository</Label
                    >
                    <div class="bg-muted p-2 rounded-md border">
                        {repo.repoDetails.full_name}
                    </div>
                    <p class="text-sm text-muted-foreground">
                        Which repository we are watching.
                    </p>
                </div>
                <div>
                    <Label for="provider" class="flex items-center"
                        ><GitBranch class="w-4 h-4 mr-1" />Branch</Label
                    >
                    <Select.Root bind:selected={branch} portal={null}>
                        <Select.Trigger>
                            <Select.Value placeholder="Select a branch"
                                ><GitBranch class="w-4" />
                            </Select.Value>
                        </Select.Trigger>
                        <Select.Content>
                            <Select.Group>
                                {#each branches as branch}
                                    <Select.Item
                                        value={branch.value}
                                        label={branch.label}
                                        >{branch.label}</Select.Item
                                    >
                                {/each}
                            </Select.Group>
                        </Select.Content>
                        <Select.Input name="branch" />
                    </Select.Root>
                    <p class="text-sm text-muted-foreground">
                        What branch should we build your app from?
                    </p>
                </div>
                <div>
                    <Label for="provider">Subdirectory</Label>
                    <Input
                        type="name"
                        placeholder="/"
                        bind:value={container.github.subdirectory}
                    />
                    <p class="text-sm text-muted-foreground">
                        Does your app exist in a subdirectory of your root
                        repository?
                    </p>
                </div>

                <div
                    class="flex flex-row items-center justify-between rounded-lg border p-4"
                >
                    <div class="space-y-0.5">
                        <Label>Auto Deploy</Label>
                        <p class="text-sm text-muted-foreground">
                            Should we automatically deploy your app when a new
                            commit is pushed to the watched branch?
                        </p>
                    </div>
                    <Switch bind:checked={container.auto_deploy} />
                </div>
            </Card.Content>
        </Card.Root>

        <Card.Root>
            <Card.Header>
                <Card.Title>Identifier</Card.Title>
                <Card.Description
                    >This is the ID used for your service with our backend</Card.Description
                >
            </Card.Header>
            <Card.Content class="flex items-center"
                >{container._id}<Copy
                    on:click={() => {
                        copyToClipboard(container._id);
                    }}
                    class="w-4 h-4 ml-2"
                /></Card.Content
            >
        </Card.Root>

        <div class="flex items-center justify-end space-x-2">
            <Drawer.Root>
                <Drawer.Trigger><Button>Save Changes</Button></Drawer.Trigger>
                <Drawer.Content>
                    <Drawer.Header class="flex-block justify-center">
                        <Drawer.Title>Changes ready to commit.</Drawer.Title>
                        {#if old_container.name !== container.name}
                            <Drawer.Description
                                >Name "{old_container.app_port}" changed to "{container.app_port}".</Drawer.Description
                            >
                        {/if}
                        {#if old_container.app_port !== container.app_port}
                            <Drawer.Description
                                >App port {old_container.app_port} changed to {container.app_port}.</Drawer.Description
                            >
                        {/if}
                        {#if old_container.github.branch !== branch.value}
                            <Drawer.Description
                                >Branch {old_container.github.branch} changed to
                                {branch.value}.</Drawer.Description
                            >
                        {/if}
                        {#if old_container.github.subdirectory !== container.github.subdirectory}
                            <Drawer.Description
                                >Subdirectory {old_container.github
                                    .subdirectory} changed to {container.github
                                    .subdirectory}.</Drawer.Description
                            >
                        {/if}
                        {#if old_container.auto_deploy !== container.auto_deploy}
                            <Drawer.Description
                                >Auto deploy status changed from {old_container.auto_deploy}
                                to {container.auto_deploy}.</Drawer.Description
                            >
                        {/if}
                    </Drawer.Header>
                    <Drawer.Footer>
                        <Button on:click={handleUpdate}>Submit</Button>
                        <Drawer.Close>Cancel</Drawer.Close>
                    </Drawer.Footer>
                </Drawer.Content>
            </Drawer.Root>

            <AlertDialog.Root bind:open={showDeleteDialog}>
                <AlertDialog.Trigger asChild let:builder>
                    <Button builders={[builder]} variant="destructive"
                        >Delete App</Button
                    >
                </AlertDialog.Trigger>
                <AlertDialog.Content>
                    <AlertDialog.Header>
                        <AlertDialog.Title
                            >Are you absolutely sure?</AlertDialog.Title
                        >
                        <AlertDialog.Description>
                            This action cannot be undone. This will permanently
                            delete your account and remove your data from our
                            servers.
                        </AlertDialog.Description>
                    </AlertDialog.Header>
                    <AlertDialog.Footer>
                        <AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
                        <AlertDialog.Action
                            on:click={() =>
                                toast.error(
                                    "Deletions aren't allowed in beta.",
                                )}>Delete</AlertDialog.Action
                        >
                    </AlertDialog.Footer>
                </AlertDialog.Content>
            </AlertDialog.Root>
        </div>
    </div>
{/if}
