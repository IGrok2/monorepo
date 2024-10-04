<script>
    import { page } from "$app/stores";
    import { Button } from "$lib/components/ui/button";
    import * as Card from "$lib/components/ui/card";

    import { SquareArrowOutUpRight } from "lucide-svelte";

    export let environment;
</script>

<div class="flex justify-end mb-2">
    <Button href="{$page.params.project}/{environment.Slug}/create/app"
        >Create App</Button
    >
</div>
<div class="my-4 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
    {#each environment.Apps as app}
        <Card.Root>
            <Card.Header>
                <Card.Title
                    ><a
                        href="{$page.params
                            .project}/{environment.Slug}/{app.Slug}"
                        class="hover:underline">{app.Slug}</a
                    ></Card.Title
                >
                <Card.Description
                    ><a
                        href="https://{app.Slug}.{$page.params
                            .project}.packetware.app"
                        class="hover:underline flex items-center text-xs"
                        >https://{app.Slug}.{$page.params
                            .project}.packetware.app<SquareArrowOutUpRight
                            class="ml-1 w-3 h-3"
                        /></a
                    ></Card.Description
                >
            </Card.Header>
            <Card.Footer>{app.Source.Channel}</Card.Footer>
        </Card.Root>
    {/each}
</div>
