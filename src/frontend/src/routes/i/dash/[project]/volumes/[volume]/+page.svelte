<script>
    import { page } from "$app/stores";

    import APIClient from "$lib/utils/api";
    import { toast } from "svelte-sonner";

    import * as Card from "$lib/components/ui/card";
    import Spinner from "$lib/components/Spinner.svelte";
    import { getColumnedBodyRows } from "svelte-headless-table";
    import Separator from "$lib/components/ui/separator/separator.svelte";

    import { Container } from "lucide-svelte";

    let loading = true;
    let volume;
    let containers;

    async function fetchVolume() {
        const res = await APIClient.get(
            `@/project/${$page.params.project}/volume/${$page.params.id}`,
        );
        let response = res.data;
        if (response.success) {
            volume = response.volume;
            containers = response.containers;
            console.log(volume);

            loading = false;
        } else {
            toast.error("Error loading volume.");
        }
    }

    fetchVolume();
</script>

{#if loading}
    <Spinner />
{:else}
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
        <Card.Root>
            <Card.Header>
                <Card.Title>{volume.name} - {volume.cluster}</Card.Title>
                <Card.Description>Size: {volume.size}GB</Card.Description>
            </Card.Header>
        </Card.Root>
        <Card.Root>
            <Card.Header>
                <Card.Title>Volume Usage</Card.Title>
            </Card.Header>
            <Card.Content>
                <div class="space-y-4">
                    <div class="flex items-center justify-between">
                        <span class="text-muted-foreground">Used</span>
                        <div
                            class="bg-secondary px-2 py-1 rounded-md text-sm font-medium"
                        >
                            6GB
                        </div>
                    </div>
                    <div class="flex items-center justify-between">
                        <span class="text-muted-foreground">Available</span>
                        <div
                            class="bg-secondary px-2 py-1 rounded-md text-sm font-medium"
                        >
                            4GB
                        </div>
                    </div>
                    <div class="flex items-center justify-between">
                        <span class="text-muted-foreground">Utilization</span>
                        <div
                            class="bg-secondary px-2 py-1 rounded-md text-sm font-medium"
                        >
                            60%
                        </div>
                    </div>
                </div>
            </Card.Content>
        </Card.Root>
    </div>
    <div class="mt-4 flex items-center">
        <Container class="mr-2" />
        <h4 class="text-2xl">Attached Containers</h4>
    </div>
    <Separator class="mb-4" />
    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <!-- {#each containers as container} -->
            <Card.Root>
                <Card.Header>
                    <Card.Title>Attached Container #1</Card.Title>
                    <Card.Description>Mount Point</Card.Description>
                </Card.Header>
            </Card.Root>
        <!-- {/each} -->
    </div>
{/if}
