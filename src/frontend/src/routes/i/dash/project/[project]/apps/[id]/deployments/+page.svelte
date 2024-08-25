<script lang="ts">
    import { page } from "$app/stores";
    import { onMount } from "svelte";

    /** @type {import('./$types').PageData} */
    export let data;

    import Spinner from "$lib/components/Spinner.svelte";

    import { Button } from "$lib/components/ui/button";
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
    import * as Dialog from "$lib/components/ui/dialog";
    import { Input } from "$lib/components/ui/input/index.js";
    import { Separator } from "$lib/components/ui/separator/index.js";

    import DataTable from "./(components)/data-table.svelte";

    import { toast } from "svelte-sonner";

    import { ChevronDown, GitCommit } from "lucide-svelte";

    import { PUBLIC_API } from "$env/static/public";

    import { getCookie } from "$lib/utils/auth";

    const token = getCookie("jwt");

    const container = data.container;

    async function fetchDeployments() {
        const response = await fetch(
            `${PUBLIC_API}/@/project/${$page.params.project}/container/${$page.params.id}/deployment/list`,
            {
                method: "GET",
                headers: new Headers({
                    "content-type": "application/json",
                    Authorization: token,
                }),
            },
        );

        console.log(response);

        const res = await response.json();
        console.log(res);

        let data = []
        for (const deployment of res.deployments) {
            data.push({
                _id: deployment._id,
                revision: deployment.build.revision,
                status: deployment.build.status
            })
        }
        console.log(data);

        return data;
    }

    async function fetchCommits() {
        const response = await fetch(
            `${PUBLIC_API}/@/github/commits/${container.github.git_repo_id}/${container.github.branch}`,
            {
                method: "GET",
                headers: new Headers({
                    "content-type": "application/json",
                    Authorization: token,
                }),
            },
        );

        const res = await response.json();
        console.log(res);

        return res.commits;
    }

    let dialogDeployCommit = false;
    let searchCommitSHA = "";
    let commitSHA = "";

    async function handleManualDeployment() {
        dialogDeployCommit = false;
        toast.success("Deployment started");

        const response = await fetch(
            `${PUBLIC_API}/@/project/${$page.params.project}/container/${$page.params.id}/deployment`,
            {
                method: "POST",
                headers: new Headers({
                    "content-type": "application/json",
                    Authorization: token,
                }),
                body: JSON.stringify({
                    provider: "github",
                    git: {
                        repo_id: container.github.git_repo_id,
                        branch: container.github.branch,
                        commit_sha: commitSHA,
                    },
                }),
            },
        );

        const res = await response.json();
        console.log(res);

        /*setTimeout(() => {
            window.location.reload();
        }, 1000);*/
    }

    async function handleRestart() {
        const response = await fetch(
            `${PUBLIC_API}/@/project/${$page.params.project}/container/${$page.params.id}/restart`,
            {
                method: "POST",
                headers: new Headers({
                    "content-type": "application/json",
                    Authorization: token,
                }),
                body: JSON.stringify({}),
            },
        );

        const res = await response.json();
        console.log(res);

        if (res.success === true) {
            toast.success("App restarted");
        }
    }
</script>

{#await fetchDeployments()}
    <Spinner />
{:then deployments}
    <div class="flex justify-end">
        <DropdownMenu.Root>
            <DropdownMenu.Trigger
                ><Button variant="outline"
                    >Deploy <ChevronDown class="h-4 w-4 ml-2" /></Button
                >
            </DropdownMenu.Trigger>
            <DropdownMenu.Content>
                <DropdownMenu.Group>
                    <DropdownMenu.Item
                        on:click={() => {
                            dialogDeployCommit = true;
                        }}>Deploy Commit</DropdownMenu.Item
                    >
                    <DropdownMenu.Separator />
                    <DropdownMenu.Item on:click={() => handleRestart()}
                        >Restart app</DropdownMenu.Item
                    >
                </DropdownMenu.Group>
            </DropdownMenu.Content>
        </DropdownMenu.Root>
    </div>
    <DataTable data={deployments.slice().reverse()} />
{/await}

<!-- Deploy from specific commit -->
<Dialog.Root bind:open={dialogDeployCommit}>
    <Dialog.Content>
        <Dialog.Header>
            <Dialog.Title>Chose a Git Commit to Deploy</Dialog.Title>
            <Dialog.Description>
                You can only chose commits from the branch you have selected for
                this app. Currently <span class="font-bold"
                    >{container.github.branch}</span
                > is the watched branch, you may change this in the project settings.
            </Dialog.Description>
        </Dialog.Header>
        <input type="hidden" name="sha" bind:value={commitSHA} />
        <Input
            name="search-sha"
            type="string"
            placeholder="Commit SHA..."
            bind:value={searchCommitSHA}
        />
        {#await fetchCommits()}
            <Spinner />
        {:then commits}
            <div class="">
                {#each commits as commit}
                    <button
                        class="p-2 flex items-center w-full hover:bg-secondary"
                        on:click={() => {
                            searchCommitSHA = commit.sha;
                            commitSHA = commit.sha;
                        }}
                        ><GitCommit class="w-4 h-4 mr-2" /><span
                            class="w-[100px]">{commit.sha.substring(0, 7)}</span
                        >{commit.commit.message}...</button
                    >
                    <Separator />
                {/each}
            </div>
        {/await}
        <Dialog.Footer>
            <form on:submit|preventDefault={handleManualDeployment}>
                <Button variant="outline">Cancel</Button>
                <Button type="submit">Deploy</Button>
            </form>
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>
