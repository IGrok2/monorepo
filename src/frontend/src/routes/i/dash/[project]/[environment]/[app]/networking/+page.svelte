<script>
    import * as Card from "$lib/components/ui/card";
    import { Button } from "$lib/components/ui/button";

    import { Networking } from "$lib/components/app";
</script>

<Card.Root class="w-full mx-auto">
    <Card.Header>
        <Card.Title>Manage Network Sockets</Card.Title>
        <Card.Description
            >Configure your app's communication preferences.</Card.Description
        >
    </Card.Header>
    <Card.Content class="space-y-6">
        <Networking />
        <Button class="w-full">Save Settings</Button>
    </Card.Content>
</Card.Root>
