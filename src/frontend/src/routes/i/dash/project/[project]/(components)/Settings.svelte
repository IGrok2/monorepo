<script>
    import { page } from "$app/stores";
    import { goto } from "$app/navigation";

    import { PUBLIC_API } from "$env/static/public";
    import { getCookie } from "$lib/utils/auth";

    import * as Card from "$lib/components/ui/card";
    import { Button } from "$lib/components/ui/button";

    const token = getCookie(`jwt`);

    const manageGitHubApp = async () => {
        const response = await fetch(`${PUBLIC_API}/@/project/${$page.params.project}/github/begin-auth`, {
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

<Card.Root
    data-x-chunk-name="dashboard-04-chunk-1"
    data-x-chunk-description="A form to update the store name."
>
    <Card.Header>
        <Card.Title>Manage Github Install</Card.Title>
        <Card.Description>
            This can be used to change the permissions of what this
            Packetware app can access in your Github repositories.
        </Card.Description>
    </Card.Header>
    <Card.Footer class="border-t px-6 py-4">
        <Button on:click={manageGitHubApp}>Manage</Button>
    </Card.Footer>
</Card.Root>
