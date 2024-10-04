<script>
    import { page } from "$app/stores";
    import { Button } from "$lib/components/ui/button";
    import * as Card from "$lib/components/ui/card";

    import { Database } from "lucide-svelte";

    export let environment;
</script>

<div class="flex justify-end mb-2">
    <Button href="{$page.params.project}/{environment.Slug}/create/volume"
        >Create Volume</Button
    >
</div>
<div class="my-4 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
    {#each environment.Volumes as volume}
        <Card.Root>
            <div class="flex items center space-x-2">
                <Database />
                <Card.Header>
                    <Card.Title
                        ><a
                            href="{$page.params
                                .project}/{environment.Slug}/{volume.Slug}"
                            class="hover:underline">{volume.Slug}</a
                        ></Card.Title
                    >
                    <Card.Description></Card.Description>
                </Card.Header>
            </div>
            <Card.Footer>{volume.Size} GB</Card.Footer>
        </Card.Root>
    {/each}
</div>
