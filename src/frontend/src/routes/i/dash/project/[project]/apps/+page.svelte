<script>
    import { Button } from "$lib/components/ui/button";
    import * as Card from "$lib/components/ui/card";
    import { Input } from "$lib/components/ui/input";
    import * as Avatar from "$lib/components/ui/avatar";
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
    import { MoreHorizontal, GitPullRequest } from "lucide-svelte";
    import { getCookie } from "$lib/utils/auth";

    import { PUBLIC_API } from "$env/static/public";

    let appMenu = "";

    const token = getCookie("jwt");

    const fetchContainers = async () => {
        const response = await fetch(`${PUBLIC_API}/@/container/list`, {
            method: "GET",
            headers: new Headers({
                "content-type": "application/json",
                Authorization: token,
            }),
        });

        const res = await response.json();

        return res.containers;
    };
</script>

<div class="my-4 flex justify-center space-x-4">
    <Input type="search" placeholder="search" class="max-w-xs" />

    <Button href="apps/new" variant="outline">New</Button>
</div>
<div class="flex justify-center">
    <div class="container">
        {#await fetchContainers()}
            <p>Loading...</p>
        {:then containers}
            <ul
                role="list"
                class="grid grid-cols-1 gap-x-6 gap-y-8 lg:grid-cols-3 xl:gap-x-8"
            >
                {#each containers as container}
                    <li>
                        <Card.Root class="w-[250px] md:w-[350px]">
                            <Card.Header>
                                <div class="flex justify-between">
                                    <div class="flex">
                                        <Avatar.Root>
                                            <Avatar.Image
                                                src="http://app-{container._id}.onpacketware.net:31080/favicon.ico"
                                            />
                                            <Avatar.Fallback>PW</Avatar.Fallback
                                            >
                                        </Avatar.Root>
                                        <div class="flex items-center">
                                            <Card.Title>
                                                <Button
                                                    href="apps/{container._id}"
                                                    variant="link"
                                                    >{container.name}</Button
                                                ></Card.Title
                                            >
                                            <!-- <Card.Description
                                class="truncate text-ellipsis overflow-hidden ..."
                                >{app.url}</Card.Description
                            >-->
                                        </div>
                                    </div>
                                    <DropdownMenu.Root>
                                        <DropdownMenu.Trigger
                                            asChild
                                            let:builder
                                        >
                                            <Button
                                                variant="ghost"
                                                builders={[builder]}
                                                ><MoreHorizontal /></Button
                                            >
                                        </DropdownMenu.Trigger>
                                        <DropdownMenu.Content>
                                            <DropdownMenu.Group>
                                                <!-- <DropdownMenu.Item
                                            href="apps/{container._id}/builds"
                                            >Builds</DropdownMenu.Item
                                        > -->
                                                <DropdownMenu.Item
                                                    >Logs</DropdownMenu.Item
                                                >
                                                <DropdownMenu.Item
                                                    >Settings</DropdownMenu.Item
                                                >
                                            </DropdownMenu.Group>
                                        </DropdownMenu.Content>
                                    </DropdownMenu.Root>
                                </div>
                            </Card.Header>
                            <!-- <Card.Content>
                                <p>{container.deployment.commit_message}</p>
                                <p>
                                    {container.deployment.deployedAt} on <GitPullRequest
                                        class="w-4 h-4 inline"
                                    />
                                    {container.deployment.branch}
                                </p>
                            </Card.Content>-->
                        </Card.Root>
                    </li>
                {/each}
            </ul>
        {:catch error}
            <p>{error.message}</p>
        {/await}
    </div>
</div>
