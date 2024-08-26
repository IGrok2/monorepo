<script>
    import { page } from "$app/stores";

    import { Button } from "$lib/components/ui/button";
    import * as Accordion from "$lib/components/ui/accordion";
    import { Input } from "$lib/components/ui/input";
    import { Label } from "$lib/components/ui/label";
    import { Switch } from "$lib/components/ui/switch";

    let privateRegistry = false;
</script>

<div class="space-y-2">
    <Label for="registry-url">Conatiner Image URL</Label>
    <Input
        type="string"
        id="registry-url"
        placeholder="ghcr.io/pterodactyl/games:rust"
    />

    <Label for="private">Private Registry</Label>
    <div>
        <Switch id="private" bind:checked={privateRegistry} />
    </div>
    {#if privateRegistry}
        <Label for="registry-username">Username</Label>
        <Input type="string" id="registry-username" />
        <Label for="registry-password">Password</Label>
        <Input type="string" id="registry-password" />
    {/if}
</div>
<div class="flex justify-end">
    <Button disabled>Next</Button>
</div>
