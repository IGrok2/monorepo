<script>
    import { Button } from "$lib/components/ui/button";
    import * as Card from "$lib/components/ui/card";

    import { Compute } from "$lib/components/app"

    /** @type {import('./$types').PageData} */
    export let data;
    let { app, regions } = data;

    const resourcePlans = [
        { name: "Basic", cpu: "1 vCPU", memory: "1 GB" },
        { name: "Standard", cpu: "2 vCPU", memory: "2 GB" },
        { name: "Premium", cpu: "4 vCPU", memory: "4 GB" },
        { name: "Enterprise", cpu: "8 vCPU", memory: "16 GB" },
    ];

    let selectedPlan = {
        value: resourcePlans[0].name,
        label: resourcePlans[0].name,
    };
</script>

<Card.Root class="w-full mx-auto">
    <Card.Header>
        <Card.Title>Manage Compute Scaling</Card.Title>
        <Card.Description
            >Configure your app's resources and regional distribution</Card.Description
        >
    </Card.Header>
    <Card.Content class="space-y-6">
        <Compute regions={regions} resourcePlans={resourcePlans} bind:selectedPlan={selectedPlan}/>
        <Button class="w-full">Save Settings</Button>
    </Card.Content>
</Card.Root>
