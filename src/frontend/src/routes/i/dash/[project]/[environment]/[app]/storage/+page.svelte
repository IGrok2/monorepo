<script>
    import { page } from "$app/stores";

    import * as Card from "$lib/components/ui/card";
    import { Button } from "$lib/components/ui/button";
    import * as Dialog from "$lib/components/ui/dialog";
    import { Input } from "$lib/components/ui/input";
    import { Label } from "$lib/components/ui/label";

    import { ExternalLink } from "lucide-svelte";

    /** @type {import('./$types').PageData} */
    export let data;
    let { app, volumes } = data;
</script>

<div class="mb-2 flex justify-end">
    <Dialog.Root>
        <Dialog.Trigger><Button>Attach Volume</Button></Dialog.Trigger>
        <Dialog.Content>
            <Dialog.Header>
                <Dialog.Title>Create A Persistent Volume</Dialog.Title>
                <Dialog.Description>
                    A persistent volume allows you to store data across
                    container reboots or multiple replicas. This only exists
                    within a single location. <a
                        href="/i/dash/{$page.params.project}/{$page.params.environment}/create/volume"
                        class="flex items-center underline"
                        >Create a volume <ExternalLink
                            class="ml-1 w-4 h-4"
                        /></a
                    >
                </Dialog.Description>
            </Dialog.Header>
            <div class="space-y-0.25">
                <Label for="name">Name</Label>
                <Input type="string" id="name" placeholder="Volume Name" />
                <p class="text-sm text-muted-foreground">
                    What should we call this volume, arbitrary effect on use.
                </p>
            </div>
            <div class="space-y-0.25">
                <Label for="mountPath">Mount Path</Label>
                <Input type="string" id="mountPath" placeholder="/data" />
                <p class="text-sm text-muted-foreground">
                    What directory should be mounted to the container of the
                    volume?
                </p>
            </div>
            <div class="space-y-0.25">
                <Label for="size">Size (GB)</Label>
                <Input type="number" id="size" placeholder="5" />
                <p class="text-sm text-muted-foreground">
                    How large should the volume be in gigabytes. Volumes are
                    billed at $0.10 / GB / month.
                </p>
            </div>
        </Dialog.Content>
    </Dialog.Root>
</div>
<div class="grid grid-cols-1 md:grid-cols-3 gap-4">
    {#each app.Storage as storage, i}
        <Card.Root>
            <Card.Header>
                <Card.Title>Attached #{i}</Card.Title>
                <Card.Description>{storage.Volume.Slug}</Card.Description>
                <Card.Description>{storage.MountPoint}</Card.Description>
            </Card.Header>
        </Card.Root>
    {/each}
</div>
