<script>
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

    import { fade, slide } from "svelte/transition";
    //import Dialog from "$lib/components/Dialog.svelte";

    import { getCookie } from "$lib/utils/auth";
    import { PUBLIC_API } from "$env/static/public";

    import { toast } from "svelte-sonner";
    import Footer from "$lib/Footer.svelte";

    const token = getCookie("jwt");

    let loaded = false;
    let name;

    let projects = [
        {
            name: "Packetware",
        },
    ];

    let noProjectsDialog = false;

    async function fetchProjects() {
        console.log("Loading projects.");
        const response = await fetch(`${PUBLIC_API}/@/me`, {
            method: "GET",
            headers: new Headers({
                "content-type": "application/json",
                Authorization: token,
            }),
        });

        const res = await response.json();
        console.log(res);

        if (res.success) {
            projects = res.projects;
            name = res.name;

            if (projects.length === 0) {
                noProjectsDialog = true;
            } else {
                for (let i = 0; i < projects.length; i++) {
                    for (let j = 0; j < projects[i].environments[0].apps.length; j++) {
                        projects[i].environments[0].apps[j] = {
                            name: projects[i].environments[0].apps[j],
                        };
                    }

                    for (let j = 0; j < projects[i].domains.length; j++) {
                        projects[i].domains[j] = {
                            name: projects[i].domains[j],
                        }
                    }
                }
            }

            setupFavicons();

            loaded = true;
        }
    }

    // iterate through the apps and fetch the favicon
    async function setupFavicons() {
        for (let i = 0; i < projects.length; i++) {
            for (let j = 0; j < projects[i].environments[0].apps.length; j++) {
                projects[i].environments[0].apps[j].favicon = await fetchFavicon(`https://app-${projects[i].environments[0].apps[j].name}.onpacketware.net`);
            }

            for (let j = 0; j < projects[i].domains.length; j++) {
                projects[i].domains[j].favicon = await fetchFavicon(`https://${projects[i].domains[j].name}`);
            }
        }

        projects = projects;
    }

    async function fetchFavicon(url) {
        const response = await fetch('/api/getFavicon', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ url })
        });

        const res = await response.json();

        if (res.success) {
            return res.data;
        } else {
            return "";
        }
    }

    fetchProjects();

    let newProjectDialog = false;
    let newProject = {
        name: "New Project Name",
    };

    async function createProject() {
        const response = await fetch(`${PUBLIC_API}/@/project/create`, {
            method: "POST",
            headers: new Headers({
                "content-type": "application/json",
                Authorization: token,
            }),
            body: JSON.stringify({ name: newProject.name }),
        });

        const res = await response.json();
        console.log(res);

        if (res.success) {
            newProjectDialog = false;
            toast.success(res.message);
            document.location.reload();
        } else {
            toast.error(res.message);
        }
    }
</script>

<div class="container py-4">
    <p class="text-4xl text-primary text-right font-krona py-5">
        👋 Welcome, {name}
    </p>
    <div class="flex justify-between">
        <h1 class="text-4xl font-bold flex items-center">My Projects</h1>
        <Dialog.Root bind:open={newProjectDialog}>
            <Dialog.Trigger><Button>Create Project</Button></Dialog.Trigger>
            <Dialog.Content>
                <Dialog.Header>
                    <Dialog.Title>Start a new project</Dialog.Title>
                    <Dialog.Description>
                        Fill in the details of your project.
                    </Dialog.Description>
                </Dialog.Header>
                <Input bind:value={newProject.name} />
                <Dialog.Footer>
                    <Button type="submit" on:click={() => createProject() }>Save changes</Button>
                </Dialog.Footer>
            </Dialog.Content>
        </Dialog.Root>
    </div>
</div>

<Separator class="mb-4" />

{#if !loaded}<Spinner />{:else}
    <div class="container">
        <div class="my-4 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {#each projects as project}
                <a href="dash/project/{project._id}">
                    <Card.Root>
                        <Card.Header>
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
                            <Card.Title
                                ><span class="hover:underline"
                                    >{project.name}</span
                                ></Card.Title
                            >
                        </Card.Header>
                        <Card.Content>
                            <Card.Description>
                                <span class="text-sm text-gray-500">
                                    {project.total} requests{(project.security !== 0) ? `, ${project.security} security events` : ``}{(project.errors !== 0) ? `, ${project.errors} errors` : ``}{(project.environments[0].apps.length !== 0 || project.domains.length !== 0) ? `: ` : ``}{project.environments[0].apps.length !== 0 ? (project.environments[0].apps.length === 1 ? `1 app` : `${project.environments[0].apps.length} apps`) : ``} {(project.domains.length !== 0 && project.environments[0].apps.length !== 0) ? `&` : ``} {project.domains.length !== 0 ? (project.domains.length === 1 ? `1 domain` : `${project.domains.length} domains`) : ``}
                                </span>
                            </Card.Description>
                        </Card.Content>
                    </Card.Root>
                </a>
            {/each}
        </div>
    </div>
{/if}

<!-- <Dialog bind:dialog={noProjectDialog} title=""></Dialog>-->
