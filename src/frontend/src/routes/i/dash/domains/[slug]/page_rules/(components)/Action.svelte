<script>
    import * as Card from "$lib/components/ui/card";
    import { Button } from "$lib/components/ui/button";
    import { Input } from "$lib/components/ui/input";

    import { Trash2 } from "lucide-svelte";

    export let trustbusting;

    // potential fields
    export let monopoly;
    export let redirect;
    export let backend_host;
    export let bucket_name;
    export let cache_level;
    export let cache_level_ttl;
</script>

<Card.Root class="w-[300px]">
    <Card.Header>
        <Card.Title class="flex justify-between items-center"><h4>{trustbusting}</h4><Button variant='destructive'><Trash2 /></Button></Card.Title>
    </Card.Header>
    <Card.Content>
        {#if trustbusting === "Smart Challenge"}
        {/if}
        {#if trustbusting === "Captcha"}
        {/if}
        {#if trustbusting === "Cache"}
        {/if}
        {#if trustbusting === "Redirect"}
            <Input id="redirect" name="redirect" bind:value={redirect} />
        {/if}
        {#if trustbusting === "Use Origin Setting"}
        {/if}
        {#if trustbusting === "Skip Human Engine"}
        {/if}
    </Card.Content>
</Card.Root>
