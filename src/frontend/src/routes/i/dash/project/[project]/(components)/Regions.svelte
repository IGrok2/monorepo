<script>
    import { Checkbox } from "$lib/components/ui/checkbox";
    import { Separator } from "$lib/components/ui/separator";

    export let regions = [
        {
            id: "LAX",
            name: "Los Angeles",
            selected: true,
        },
        {
            id: "DAL",
            name: "Dallas",
            selected: true,
        },
        {
            id: "ASH",
            name: "Ashburn",
            selected: true,
        },
        {
            id: "AMS",
            name: "Amsterdam",
            selected: true,
        },
    ];
</script>

<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
    {#each regions as region, i}
        <div
            class="border p-4 rounded-lg shadow-sm flex items-center gap-4 hover:bg-muted"
            on:click={() => (region.selected = !region.selected)}
        >
            <Checkbox id="location-1" bind:checked={region.selected} />

            <div class="flex-1 grid gap-1">
                <div class="font-medium">{region.name}</div>
                <!--<div class="text-sm text-muted-foreground">Ping: 45ms</div>-->
            </div>
        </div>
    {/each}
</div>
