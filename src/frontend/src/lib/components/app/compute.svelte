<script>
    import { Input } from "$lib/components/ui/input";
    import { Label } from "$lib/components/ui/label";
    import * as Select from "$lib/components/ui/select";

    export const resourcePlans = [
        { name: "Basic", cpu: "1 vCPU", memory: "1 GB" },
        { name: "Standard", cpu: "2 vCPU", memory: "2 GB" },
        { name: "Premium", cpu: "4 vCPU", memory: "4 GB" },
        { name: "Enterprise", cpu: "8 vCPU", memory: "16 GB" },
    ];

    export let regions;

    export let selectedPlan = {
        value: resourcePlans[0].name,
        label: resourcePlans[0].name,
    };
</script>

<div class="space-y-2">
    <Label for="resource-plan">Resource Plan</Label>
    <Select.Root bind:selected={selectedPlan}>
        <Select.Trigger id="resource-plan" class="w-full">
            <Select.Value placeholder="Select a resource plan" />
        </Select.Trigger>
        <Select.Content>
            {#each resourcePlans as plan}
                <Select.Item value={plan.name}>
                    {plan.name} ({plan.cpu}, {plan.memory})
                </Select.Item>
            {/each}
        </Select.Content>
    </Select.Root>
</div>

<div class="space-y-4">
    <Label>Replicas per Region</Label>
    <div class="grid gap-4 sm:grid-cols-2">
        {#each regions as region}
            <div class="flex items-center space-x-2">
                <Label for={region.Slug} class="w-full">{region.Name}</Label>
                <Input
                    id={region.Slug}
                    type="number"
                    min="0"
                    value="1"
                    class="w-20"
                />
            </div>
        {/each}
    </div>
</div>
