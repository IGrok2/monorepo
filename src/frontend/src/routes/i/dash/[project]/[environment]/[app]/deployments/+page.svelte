<script lang="ts">
    import { page } from "$app/stores";
    
    import APIClient from "$lib/utils/api";

    /** @type {import('./$types').PageData} */
    export let data;

    import Spinner from "$lib/components/Spinner.svelte";

    import { Button } from "$lib/components/ui/button";
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
    import * as Dialog from "$lib/components/ui/dialog";
    import * as AlertDialog from "$lib/components/ui/alert-dialog";
    import { Input } from "$lib/components/ui/input";
    import { Separator } from "$lib/components/ui/separator";

    import DataTable from "./(components)/data-table.svelte";

    import { toast } from "svelte-sonner";

    import { ChevronDown, GitCommit } from "lucide-svelte";

    const { app, deployments } = data;

    let dialogDeployCommit = false;
    let restartAppDialog = false;

    /** @param {{ currentTarget: EventTarget & HTMLFormElement}} event */
    async function deployCommitSHA(event) {
        toast.loading("Attempting to deploy from git commit...");
        const data = new FormData(event.currentTarget);
        let sha = data.get("sha");

        let body = {
            sha,
        };
        try {
            let res = await APIClient.post(
                `/project/${$page.params.project}/environment/${$page.params.environment}/app/${$page.params.app}/deploy/git`,
                body,
            );
            dialogDeployCommit = false;
            toast.success(res.data.data.message);
            // Wait for 1 second before reloading the page
            setTimeout(() => {
                document.location.reload();
            }, 1000);
        } catch (err) {
            console.log(err);
            toast.error(err.response.data.data.message);
        }
    }

    async function handleRestart() {}
</script>

<div class="flex justify-end">
    <DropdownMenu.Root>
        <DropdownMenu.Trigger
            ><Button variant="outline"
                >Deploy <ChevronDown class="h-4 w-4 ml-2" /></Button
            >
        </DropdownMenu.Trigger>
        <DropdownMenu.Content>
            <DropdownMenu.Group>
                <DropdownMenu.Item on:click={() => dialogDeployCommit = true}
                    >Deploy Commit</DropdownMenu.Item
                >
                <!--<DropdownMenu.Separator />
                <DropdownMenu.Item on:click={() => restartAppDialog = true}
                    >Restart app</DropdownMenu.Item
                >
                    -->
            </DropdownMenu.Group>
        </DropdownMenu.Content>
    </DropdownMenu.Root>
</div>
<DataTable data={deployments} />

<!-- Deploy from specific commit -->
<Dialog.Root bind:open={dialogDeployCommit}>
    <Dialog.Content>
        <form
            method="POST"
            on:submit|preventDefault={deployCommitSHA}
            class="space-y-4"
        >
            <Dialog.Header>
                <Dialog.Title>Chose a Git Commit to Deploy</Dialog.Title>
                <Dialog.Description>
                    You can find a list of commits on your git repository
                    provider and you can copy the hash SHA to enter here and we
                    will deploy with the new code.
                </Dialog.Description>
            </Dialog.Header>
            <Input
                type="string"
                id="sha"
                name="sha"
                placeholder="Commit SHA..."
            />
            <Dialog.Footer>
                <Button variant="outline">Cancel</Button>
                <Button type="submit">Deploy</Button>
            </Dialog.Footer>
        </form>
    </Dialog.Content>
</Dialog.Root>

<AlertDialog.Root bind:open={restartAppDialog}>
    <AlertDialog.Content>
        <AlertDialog.Header>
            <AlertDialog.Title>Are you sure you want to restart?</AlertDialog.Title>
            <AlertDialog.Description>
                This action can result in your application being down for the duration it takes to restart it. 
                This will also apply new environment variable changes to your container when it begins running.
            </AlertDialog.Description>
        </AlertDialog.Header>
        <AlertDialog.Footer>
            <AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
            <AlertDialog.Action>Continue</AlertDialog.Action>
        </AlertDialog.Footer>
    </AlertDialog.Content>
</AlertDialog.Root>
