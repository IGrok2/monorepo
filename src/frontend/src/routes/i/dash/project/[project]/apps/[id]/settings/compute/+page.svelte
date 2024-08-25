<script>
    import * as Card from "$lib/components/ui/card";
    import { Button } from "$lib/components/ui/button";
    import { Separator } from "$lib/components/ui/separator";
    import * as Select from "$lib/components/ui/select";
    import { Slider } from "$lib/components/ui/slider";

    let replicas = [1];
    console.log(replicas);

    let selectedCompute = { label: "Micro", cpu: 10, memory: 256, price: 150 };

    function selectCompute(plan) {
        selectedCompute = plan;
        console.log(plan);
    }

    const compute = [
        { label: "Micro", cpu: 10, memory: 256, price: 150 },
        { label: "Small", cpu: 25, memory: 512, price: 250 },
        { label: "Medium", cpu: 50, memory: 1024, price: 400 },
        { label: "Roomy", cpu: 100, memory: 4096, price: 900 },
    ];
</script>

<Card.Root>
    <Card.Header>
        <Card.Title>Compute Resources</Card.Title>
        <Card.Description
            >Choose the instance size of your application.</Card.Description
        >
    </Card.Header>
    <Card.Content>
        {#each compute as plan}
            <div
                class="border rounded-lg p-4 m-2 cursor-pointer hover:border-primary hover:shadow-md transition duration-300"
                on:click={() => selectCompute(plan)}
            >
                <p class="font-semibold">{plan.label}</p>
                <p class="text-muted-foreground">
                    CPU: {plan.cpu} Memory: {plan.memory} Price: ${(plan.price / 100).toFixed(2)}
                </p>
            </div>
        {/each}
    </Card.Content>
    <Card.Header>
        <Card.Title>Replicas</Card.Title>
        <Card.Description
            >Choose the quantity of replicas of your application. We
            automatically balance the incoming connections between the
            instances.</Card.Description
        >
    </Card.Header>
    <Card.Content>
        <p>Ashburn, VA Replicas: {replicas}</p>
        <Slider bind:value={replicas} min={1} max={5} step={1} />
    </Card.Content>
    <Separator />
    <Card.Footer class="mt-2 flex justify-end space-x-4">
        <Card.Title>Compute Cost: ${((selectedCompute.price * replicas[0]) / 100).toFixed(2)} / Month</Card.Title>
        <Button type="submit">Save</Button>
    </Card.Footer>
</Card.Root>
