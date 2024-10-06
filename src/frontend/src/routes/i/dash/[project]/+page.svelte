<script>
    import { page } from "$app/stores";
    import APIClient from "$lib/utils/api";

    import { toast } from "svelte-sonner";

    import { Separator } from "$lib/components/ui/separator";
    import * as Tabs from "$lib/components/ui/tabs";
    import { Button } from "$lib/components/ui/button";
    import * as Dialog from "$lib/components/ui/dialog";
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
    import { Input } from "$lib/components/ui/input";
    import { Label } from "$lib/components/ui/label";
    import {
        Container,
        ChevronDown,
        PlusCircle,
        Check,
        Pencil,
    } from "lucide-svelte";

    import { Apps, Volumes } from "./(components)";

    import Spinner from "$lib/components/Spinner.svelte";
    import DropdownMenuSeparator from "$lib/components/ui/dropdown-menu/dropdown-menu-separator.svelte";

    /** @type {import('./$types').PageData} */
    export let data;

    let project = data.project;
    let selectedEnvironment = project.Environments[0];

    let createEnvironmentDialog = false;

    /** @param {{ currentTarget: EventTarget & HTMLFormElement}} event */
    async function createEnvironment(event) {
        toast.loading("Attempting to create a new environment...");
        const data = new FormData(event.currentTarget);
        let slug = data.get("slug");
        let body = {
            slug: slug,
        };

        try {
            let res = await APIClient.post(
                `/project/${$page.params.project}/environment`,
                body,
            );
            createEnvironmentDialog = false;
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
</script>

<div class="container py-4">
    <div class="md:flex md:justify-between items-center space-y-2">
        <h1 class="text-4xl font-bold flex items-center">
            {project.Slug}
        </h1>
        <div class="flex justify-end space-x-2">
            <Button variant="ghost"><Pencil class="size-4" /></Button>
            <DropdownMenu.Root>
                <DropdownMenu.Trigger
                    ><Button variant="outline"
                        ><Container
                            class="mr-2 w-4 h-4"
                        />{selectedEnvironment.Slug}<ChevronDown
                            class="ml-2 w-4 h-4"
                        /></Button
                    ></DropdownMenu.Trigger
                >
                <DropdownMenu.Content>
                    <DropdownMenu.Group>
                        {#each project.Environments as environment}
                            <DropdownMenu.Item
                                on:click={() =>
                                    (selectedEnvironment = environment)}
                                >{environment.Slug}{#if selectedEnvironment.ID === environment.ID}<Check
                                        class="ml-2w-4 h-4"
                                    />{/if}</DropdownMenu.Item
                            >
                        {/each}
                    </DropdownMenu.Group>
                    <DropdownMenu.Separator />
                    <DropdownMenu.Item
                        on:click={() => (createEnvironmentDialog = true)}
                        ><div class="flex items-center justify-between">
                            <span>Create Environment</span>
                            <PlusCircle class="ml-2 h-4 w-4" />
                        </div></DropdownMenu.Item
                    >
                </DropdownMenu.Content>
            </DropdownMenu.Root>
        </div>
    </div>
</div>

<Dialog.Root bind:open={createEnvironmentDialog}>
    <Dialog.Content>
        <Dialog.Header>
            <Dialog.Title>Create A Environment</Dialog.Title>
            <Dialog.Description>^[a-z0-9]+(?:-[a-z0-9]+)*$</Dialog.Description>
        </Dialog.Header>
        <form method="POST" on:submit|preventDefault={createEnvironment}>
            <div class="space-y-2">
                <Label for="slug">Name / Slug</Label>
                <Input
                    type="slug"
                    name="slug"
                    id="slug"
                    placeholder="environment-slug"
                />
                <div class="flex justify-end">
                    <Button type="submit">Submit</Button>
                </div>
            </div>
        </form>
    </Dialog.Content>
</Dialog.Root>

<div class="container">
    <div class="overflow-auto">
        <Tabs.Root value="apps">
            <Tabs.List>
                <Tabs.Trigger value="apps">Apps</Tabs.Trigger>
                <Tabs.Trigger value="volumes">Volumes</Tabs.Trigger>
                <Tabs.Trigger value="members">Members</Tabs.Trigger>
                <Tabs.Trigger value="secrets">Secrets</Tabs.Trigger>
                <Tabs.Trigger value="activity">Activity</Tabs.Trigger>
            </Tabs.List>
            <Tabs.Content value="apps">
                <Apps bind:environment={selectedEnvironment} />
            </Tabs.Content>
            <Tabs.Content value="volumes">
                <Volumes bind:environment={selectedEnvironment} />
            </Tabs.Content>
        </Tabs.Root>
    </div>
</div>
