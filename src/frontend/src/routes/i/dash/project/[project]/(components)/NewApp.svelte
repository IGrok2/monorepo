<script>
    import { page } from "$app/stores";

    import { tick } from "svelte";
    import { cn } from "$lib/utils";
    import { goto } from "$app/navigation";
    import { getCookie } from "$lib/utils/auth";

    import { Button } from "$lib/components/ui/button";
    import * as Card from "$lib/components/ui/card";
    import * as Select from "$lib/components/ui/select";
    import { Input } from "$lib/components/ui/input";
    import { Label } from "$lib/components/ui/label";
    import { Separator } from "$lib/components/ui/separator";
    import * as Avatar from "$lib/components/ui/avatar";
    import * as RadioGroup from "$lib/components/ui/radio-group";
    import * as Command from "$lib/components/ui/command";
    import * as Popover from "$lib/components/ui/popover";
    import { Switch } from "$lib/components/ui/switch";

    import { Check, ChevronsUpDown } from "lucide-svelte";

    import Spinner from "$lib/components/Spinner.svelte";

    import DeployGitApp from "./DeployGitApp.svelte";
    import DeployImage from "./DeployImage.svelte"

    import { toast } from "svelte-sonner";

    import { PUBLIC_API } from "$env/static/public";

    const token = getCookie(`jwt`);

    let loading = true;

    let installs = [
        {
            login: "Select a login...",
            repos: [],
        },
    ];

    async function github() {
        const token = getCookie(`jwt`);

        const response = await fetch(`${PUBLIC_API}/@/github/repos`, {
            method: "GET",
            headers: new Headers({
                "content-type": "application/json",
                Authorization: token,
            }),
        });

        let res = await response.json();
        console.log(res);

        if (res.success) {
            installs = res.installs;
            loading = false;
        } else {
            toast.error(res.message);
        }
    }

    github();

    let provider = "github";

    let clone_url = "";

    let installIndex = 0;

    let login = {
        disabled: false,
        label: installs[installIndex].login,
        value: installs[installIndex].login,
    };

    let search = "";

    let open = false;
    let repoId = "";

    // String name placeholder of the selected repository.
    $: selectedRepository =
        installs[installIndex].repos.find(
            (repo) => String(repo.id) === String(repoId),
        )?.name ?? "Select a repository...";

    // We want to refocus the trigger button when the user selects
    // an item from the list so users can continue navigating the
    // rest of the form with the keyboard.
    function closeAndFocusTrigger(triggerId) {
        open = false;
        tick().then(() => {
            document.getElementById(triggerId)?.focus();
        });
    }

    const manageGitHubApp = async () => {
        const response = await fetch(`${PUBLIC_API}/@/github/begin-auth`, {
            method: "GET",
            headers: new Headers({
                "content-type": "application/json",
                Authorization: token,
            }),
        });

        const res = await response.json();
        console.log(res.installURL);

        // Perform the redirect
        goto(res.installURL);
    };
</script>

{#if loading}
    <div class="my-4">
        <Spinner />
    </div>
{:else}
    <div class="space-y-2">
        <Label for="provider">Provider</Label>
        <RadioGroup.Root id="provider" bind:value={provider}>
            <div class="flex items-center space-x-2">
                <RadioGroup.Item value="github" id="github" />
                <Label for="github"
                    >Github <a class="underline" on:click={manageGitHubApp}
                        >Manage GitHub Account Access</a
                    ></Label
                >
            </div>
            <div class="flex items-center space-x-2">
                <RadioGroup.Item value="clone" id="clone" />
                <Label for="clone">Git Clone</Label>
            </div>
            <div class="flex items-center space-x-2">
                <RadioGroup.Item value="image" id="image" />
                <Label for="image">Container Image</Label>
            </div>
            <RadioGroup.Input name="spacing" />
        </RadioGroup.Root>
    </div>
    {#if provider === "github"}
        {#if installs.length === 0}
            <p>No GitHub installations found.</p>
        {/if}
        <div class="space-y-2">
            <div class="grid w-full items-center gap-4">
                <div class="flex flex-col space-y-1.5">
                    <Label for="login">Login</Label>
                    <Select.Root bind:selected={login}>
                        <Select.Trigger id="login">
                            <Select.Value />
                        </Select.Trigger>
                        <Select.Content>
                            {#each installs as install, index}
                                <Select.Item
                                    value={install.login}
                                    label={install.login}
                                    on:click={() => {
                                        installIndex = index;
                                        repoId =
                                            installs[installIndex].repos[0].id;
                                        selectedRepository =
                                            installs[installIndex].repos[0]
                                                .name;
                                    }}>{install.login}</Select.Item
                                >
                            {/each}
                        </Select.Content>
                    </Select.Root>
                </div>
            </div>
        </div>
        <div class="space-y-2">
            <Popover.Root bind:open let:ids>
                <Popover.Trigger asChild let:builder>
                    <Button
                        builders={[builder]}
                        variant="outline"
                        role="combobox"
                        aria-expanded={open}
                        class="w-full justify-between"
                    >
                        {selectedRepository}
                        <ChevronsUpDown
                            class="ml-2 h-4 w-4 shrink-0 opacity-50"
                        />
                    </Button>
                </Popover.Trigger>
                <Popover.Content class="p-0">
                    <Command.Root class="overflow-y-auto">
                        <Command.Input placeholder="Search repository..." />
                        <Command.Empty>No repository found.</Command.Empty>
                        <Command.Group>
                            {#each installs as install}
                                {#if install.login === login.value}
                                    {#each install.repos as repository}
                                        <!-- {#each repositories as repository} -->
                                        <Command.Item
                                            value={String(repository.id)}
                                            onSelect={(currentValue) => {
                                                repoId = repository.id;
                                                closeAndFocusTrigger(
                                                    ids.trigger,
                                                );
                                            }}
                                            class="focus"
                                        >
                                            <Check
                                                class={cn(
                                                    "mr-2 h-4 w-4",
                                                    repoId !== repository.id &&
                                                        "text-transparent",
                                                )}
                                            />
                                            {repository.name}
                                        </Command.Item>
                                        <!-- {/each} -->
                                    {/each}
                                {/if}
                            {/each}
                        </Command.Group>
                    </Command.Root>
                </Popover.Content>
            </Popover.Root>
        </div>
        <!-- 
        <Card.Content class="space-y-2">
            <Input bind:value={search} placeholder="search" />
            {#each installs as install}
                {#if install.login === login.value}
                    {#each install.repos as repo}
                        {#if repo.name.includes(search)}
                            <div class="flex justify-between items-center">
                                <div class="flex items-center space-x-4">
                                    <Avatar.Root>
                                        <Avatar.Image
                                            src={repo.avatar}
                                            alt="GitHub Repository Avatar"
                                        />
                                        <Avatar.Fallback>GH</Avatar.Fallback>
                                    </Avatar.Root><span>{repo.name}</span>
                                </div>
                                <Button href="new/{repo.id}">Import</Button>
                            </div>
                            <Separator />
                        {/if}
                    {/each}
                {/if}
            {/each}
        </Card.Content>
        -->
        <!--
        {#if repoId}
            <DeployGitApp bind:repoId />
        {/if}-->
        <div class="flex justify-end">
            <Button
                href="/i/dash/project/{$page.params.project}/apps/new/{repoId}"
                disabled={!repoId}>Next</Button
            >
        </div>
    {:else if provider === "clone"}
        <div class="space-y-2">
            <Label for="clone-url">Public Git Clone URL</Label>
            <Input
                bind:value={clone_url}
                type="string"
                id="clone-url"
                placeholder="https://github.com/packetware/hello-world.git"
            />
        </div>
        <div class="flex justify-end">
            <Button disabled>Next</Button>
        </div>
    {:else if provider === "image"}
        <DeployImage />
    {/if}
{/if}
