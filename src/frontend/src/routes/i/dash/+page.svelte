<script>
    import APIClient from "$lib/utils/api";

    //import * as Dialog from "$lib/components/ui/dialog";
    import { Button } from "$lib/components/ui/button";
    import * as Card from "$lib/components/ui/card";
    import { Input } from "$lib/components/ui/input";
    import { Label } from "$lib/components/ui/label";
    import * as Dialog from "$lib/components/ui/dialog";
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
    import { Separator } from "$lib/components/ui/separator";
    import * as Tooltip from "$lib/components/ui/tooltip";
    import { Folder } from "lucide-svelte";
    import * as Avatar from "$lib/components/ui/avatar";

    import Spinner from "$lib/components/Spinner.svelte";

    import { toast } from "svelte-sonner";

    let loaded = false;

    /** @type {import('./$types').PageData} */
    export let data;
    console.log(data);
    let user = data.user;
    let projects = data.projects;

    let createProjectDialog = false;

    let newProject = {
        slug: "pro",
    };

    async function createProject() {
        toast.loading("Attempting to create a new project...");
        try {
            let res = await APIClient.post(`/user/project`, newProject);
            createProjectDialog = false;
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
    <p class="text-4xl text-primary text-right font-krona py-5">
        👋 Welcome, {user.Name}
    </p>
    <div class="flex justify-between">
        <h1 class="text-4xl font-bold flex items-center">My Projects</h1>
        <Dialog.Root bind:open={createProjectDialog}>
            <Dialog.Trigger><Button>Create Project</Button></Dialog.Trigger>
            <Dialog.Content>
                <Dialog.Header>
                    <Dialog.Title>Start a new project</Dialog.Title>
                    <Dialog.Description>
                        A Packetware project is intended to allow for small
                        teams to collaborate on multiple services called "Apps"
                        that interact with each other.
                    </Dialog.Description>
                </Dialog.Header>
                <Input bind:value={newProject.slug} />
                <Dialog.Footer>
                    <Button type="submit" on:click={() => createProject()}
                        >Save changes</Button
                    >
                </Dialog.Footer>
            </Dialog.Content>
        </Dialog.Root>
    </div>
</div>

<Separator class="mb-4" />

<div class="container">
    <div class="my-4 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {#each projects as project}
            <a href="dash/{project.Slug}">
                <Card.Root>
                    <Card.Header>
                        <!--
                            <div class="flex -space-x-2 overflow-hidden pb-2">
                                {#each project.environments[0].apps as container}
                                    <Tooltip.Root>
                                        <Tooltip.Trigger asChild let:builder>
                                        <Avatar.Root>
                                            <Avatar.Image
                                                    src="{container.favicon}"
                                            />
                                            <Avatar.Fallback
                                            >{container.name.charAt(
                                                0,
                                            )}{container.name.charAt(
                                                container.name.split(".")[0]
                                                    .length - 1,
                                            )}</Avatar.Fallback
                                            >
                                        </Avatar.Root>
                                        </Tooltip.Trigger>
                                        <Tooltip.Content>{container}</Tooltip.Content>

                                    </Tooltip.Root>
                                {/each}
                                {#each project.domains as domain}
                                    <Avatar.Root>
                                        <Avatar.Image
                                                src="{domain.favicon}"
                                        />
                                        <Avatar.Fallback
                                        >{domain.name.charAt(
                                            0,
                                        )}{domain.name.charAt(
                                            domain.name.split(".")[0]
                                                .length - 1,
                                        )}</Avatar.Fallback
                                        >
                                    </Avatar.Root>
                                {/each}
                            </div>
                            -->
                        <Card.Title
                            ><span class="hover:underline">{project.Slug}</span
                            ></Card.Title
                        >
                    </Card.Header>

                    <!--
                    <Card.Content>
                        
                            <Card.Description>
                                <span class="text-sm text-gray-500">
                                    {project.total} requests{(project.security !== 0) ? `, ${project.security} security events` : ``}{(project.errors !== 0) ? `, ${project.errors} errors` : ``}{(project.environments[0].apps.length !== 0 || project.domains.length !== 0) ? `: ` : ``}{project.environments[0].apps.length !== 0 ? (project.environments[0].apps.length === 1 ? `1 app` : `${project.environments[0].apps.length} apps`) : ``} {(project.domains.length !== 0 && project.environments[0].apps.length !== 0) ? `&` : ``} {project.domains.length !== 0 ? (project.domains.length === 1 ? `1 domain` : `${project.domains.length} domains`) : ``}
                                </span>
                            </Card.Description>
                            
                    </Card.Content>-->
                </Card.Root>
            </a>
        {/each}
    </div>
</div>
