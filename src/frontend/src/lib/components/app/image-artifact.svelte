<script>
    import * as RadioGroup from "$lib/components/ui/radio-group";
    import * as Select from "$lib/components/ui/select";
    import { Switch } from "$lib/components/ui/switch";
    import { Label } from "$lib/components/ui/label";
    import { Input } from "$lib/components/ui/input";

    import { ExternalLink } from "lucide-svelte";

    import { PUBLIC_API } from "$env/static/public";
    import APIClient from "$lib/utils/api";

    // Retrieved information
    //export let git_namespaces;
    export let namespace_repos;
    export let branches;

    // options
    export let channel = "GITHUB";
    export let selectedNamespace;
    export let selectedRepo;
    export let selectedBranch;
    export let subdirectory;

    export let image_url;
    export let container_registries;

    let public_source = false;

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
            console.log("Repos: ", repos);

            return repos;
        } catch (err) {
            console.log("err: ", err);
        }
    }

    async function fetchBranches(namespaceId, repoId) {}
</script>

<div class="space-y-4">
    {#if channel === "GITHUB"}
        <div class="flex items-center space-x-2">
            <Switch bind:checked={public_source} id="public-respository" />
            <Label for="public-respository">Public Repository</Label>
        </div>
        {#if public_source === false}
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
            {#await fetchNamespaces() then namespaces}
                <div>
                    <h2 class="text-base font-semibold leading-7">Namespace</h2>
                    <p class="mt-1 text-sm leading-6 text-muted-foreground">
                        What Github App install should we use?
                    </p>
                    <Select.Root bind:selected={selectedNamespace}>
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
                {#await fetchRepos(selectedNamespace.value) then repos}
                    <div>
                        <h2 class="text-base font-semibold leading-7">Repo</h2>
                        <p class="mt-1 text-sm leading-6 text-muted-foreground">
                            What Github repo should we clone?
                        </p>

                        <Select.Root bind:selected={selectedRepo}>
                            <Select.Trigger>
                                <Select.Value />
                            </Select.Trigger>
                            <Select.Content>
                                {#each repos as repo}
                                    <Select.Item
                                        value={repo.id}
                                        label={repo.name}
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
                    When you use a Public Repo we need to know where to clone
                    from.
                </p>

                <Input
                    type="text"
                    name="git_repo"
                    id="git_repo"
                    placeholder="https://github.com/UltraSive/sveltekit-test.git"
                    bind:value={selectedRepo.URL}
                />
            </div>
        {/if}
        {#if selectedRepo}
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
                                value={branch.value}
                                label={branch.label}>{branch.label}</Select.Item
                            >
                        {/each}
                    </Select.Content>
                </Select.Root>
            </div>
            <div>
                <h2 class="text-base font-semibold leading-7">App Directory</h2>
                <p class="mt-1 text-sm leading-6 text-muted-foreground">
                    Leave this blank if you app is at the root of the
                    repository. This allows you to only deploy a subdirectory of
                    your repo. Very helpful for monorepos.
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
    {:else if channel === "IMAGE"}
        <div class="flex items-center space-x-2">
            <Switch bind:checked={public_source} id="public-image" />
            <Label for="public-image">Public Container Image</Label>
        </div>
        <div>
            <h2 class="text-base font-semibold leading-7">
                Container Image URL
            </h2>
            <p class="mt-1 text-sm leading-6 text-muted-foreground">
                Where can we download your container image?
            </p>

            <Input
                type="text"
                name="image-url"
                id="image-url"
                placeholder="https://hub.docker.com/_/nginx:stable-alpine-perl"
                bind:value={image_url}
            />
        </div>
    {/if}
</div>
