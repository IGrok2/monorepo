<script>
    import * as RadioGroup from "$lib/components/ui/radio-group";
    import * as Select from "$lib/components/ui/select";
    import { Switch } from "$lib/components/ui/switch";
    import { Label } from "$lib/components/ui/label";
    import { Input } from "$lib/components/ui/input";

    import Spinner from "$lib/components/Spinner.svelte";

    import { ExternalLink } from "lucide-svelte";

    import { PUBLIC_API } from "$env/static/public";
    import APIClient from "$lib/utils/api";

    // options
    export let public_repo = false;
    export let repo_url;
    export let selectedNamespace;
    export let selectedRepo;
    export let selectedBranch;
    export let subdirectory;

    async function fetchNamespaces() {
        console.log("Getting installs");
        try {
            let res = await APIClient.get(`/integration/github/installs`);
            let installs = res.data.data.git_namespaces;

            return installs;
        } catch (err) {
            console.log("err: ", err);
        }
    }

    async function fetchRepos(namespaceId) {
        console.log("Getting repos");
        try {
            let res = await APIClient.get(
                `/integration/github/install/${namespaceId}/repos`,
            );
            let repos = res.data.data.repositories;

            return repos;
        } catch (err) {
            console.log("err: ", err);
        }
    }

    async function fetchBranches(namespaceId, repoId) {
        console.log("Getting branches");
        try {
            let res = await APIClient.get(
                `/integration/github/install/${namespaceId}/repo/${repoId}/branches`,
            );
            let branches = res.data.data.branches;

            return branches;
        } catch (err) {
            console.log("err: ", err);
        }
    }

    // On change
    function onNamespaceChange() {
        selectedRepo = undefined;
        selectedBranch = undefined;
    }

    function onRepoChange() {
        selectedBranch = undefined;
    }
</script>

<div class="space-y-4">
    <div class="flex items-center space-x-2">
        <Switch bind:checked={public_repo} id="public-respository" />
        <Label for="public-respository">Public Repository</Label>
    </div>
    {#if public_repo === false}
        <div class="border-s-4 bg-muted/25 border-fuchsia-600 ps-3 text-sm">
            <a
                target="_blank"
                rel="noopener noreferrer"
                href="{PUBLIC_API}/integration/github/install"
                class="inline-flex items-center underline"
                >Change GitHub App Namespace Permissions<ExternalLink
                    class="ml-2 h-4 w-4"
                /></a
            >
        </div>
        {#await fetchNamespaces()}
        <div class="flex justify-center my-2"><Spinner /></div>
        {:then namespaces}
            <div>
                <h2 class="text-base font-semibold leading-7">Namespace</h2>
                <p class="mt-1 text-sm leading-6 text-muted-foreground">
                    What Github App install should we use?
                </p>
                <Select.Root bind:selected={selectedNamespace} onSelectedChange={onNamespaceChange}>
                    <Select.Trigger>
                        <Select.Value />
                    </Select.Trigger>
                    <Select.Content>
                        {#each namespaces as namespace}
                            <Select.Item
                                value={namespace.installation_id}
                                label={namespace.account.login}
                                >{namespace.account.login}</Select.Item
                            >
                        {/each}
                    </Select.Content>
                </Select.Root>
            </div>
        {/await}
        {#if selectedNamespace}
            {#await fetchRepos(selectedNamespace.value)}
            <div class="flex justify-center my-2"><Spinner /></div>
            {:then repos}
                <div>
                    <h2 class="text-base font-semibold leading-7">Repo</h2>
                    <p class="mt-1 text-sm leading-6 text-muted-foreground">
                        What Github repo should we clone?
                    </p>

                    <Select.Root bind:selected={selectedRepo} onSelectedChange={onRepoChange}>
                        <Select.Trigger>
                            <Select.Value />
                        </Select.Trigger>
                        <Select.Content>
                            {#each repos as repo}
                                <Select.Item value={repo.id} label={repo.name}
                                    >{repo.name}</Select.Item
                                >
                            {/each}
                        </Select.Content>
                    </Select.Root>
                </div>
            {/await}
        {/if}
    {:else}
        <div>
            <h2 class="text-base font-semibold leading-7">Git Repo URL</h2>
            <p class="mt-1 text-sm leading-6 text-muted-foreground">
                When you use a Public Repo we need to know where to clone from.
            </p>

            <Input
                type="text"
                name="git_repo"
                id="git_repo"
                placeholder="https://github.com/namespace/repo.git"
                bind:value={repo_url}
            />
        </div>
    {/if}
    {#if selectedRepo}
        {#await fetchBranches(selectedNamespace.value, selectedRepo.value)}
        <div class="flex justify-center my-2"><Spinner /></div>
        {:then branches}
            <div>
                <h2 class="text-base font-semibold leading-7">Branch</h2>
                <p class="mt-1 text-sm leading-6 text-muted-foreground">
                    What Git branch should we listen for pushes on?
                </p>

                <Select.Root bind:selected={selectedBranch}>
                    <Select.Trigger>
                        <Select.Value />
                    </Select.Trigger>
                    <Select.Content>
                        {#each branches as branch}
                            <Select.Item
                                value={branch.name}
                                label={branch.name}>{branch.name}</Select.Item
                            >
                        {/each}
                    </Select.Content>
                </Select.Root>
            </div>
        {/await}
        <div>
            <h2 class="text-base font-semibold leading-7">App Directory</h2>
            <p class="mt-1 text-sm leading-6 text-muted-foreground">
                Leave this blank if you app is at the root of the repository.
                This allows you to only deploy a subdirectory of your repo. Very
                helpful for monorepos.
            </p>

            <Input
                type="text"
                name="subdirectory"
                id="subdirectory"
                placeholder="/"
                bind:value={subdirectory}
            />
        </div>
    {/if}
</div>
