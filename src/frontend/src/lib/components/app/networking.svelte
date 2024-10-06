<script>
    import * as RadioGroup from "$lib/components/ui/radio-group";
    import * as Card from "$lib/components/ui/card";
    import { Label } from "$lib/components/ui/label";
    import { Input } from "$lib/components/ui/input";
    import { Separator } from "$lib/components/ui/separator";
    import * as Select from "$lib/components/ui/select";
    import { Switch } from "$lib/components/ui/switch";
    import { Button } from "$lib/components/ui/button";

    import { Minus, Plus } from "lucide-svelte";

    const protocols = [
        { value: 6, label: "TCP" },
        //{ value: "UDP", label: "UDP" },
        //{ value: "PRIVATE", label: "Private" },
        //{ value: "NONE", label: "None" },
    ];

    export let network_type = "PUBLIC";
    export let ports = [
        {
            name: "node",
            number: 3000,
            protocol: { value: 6, label: "TCP" },
            http: true,
        },
    ];
</script>

<RadioGroup.Root bind:value={network_type}>
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        <Card.Root>
            <div class="flex h-full items-center">
                <div class="ml-4 h-full flex items-center space-x-4">
                    <RadioGroup.Item value="PUBLIC" id="PUBLIC" />
                    <Separator orientation="vertical" />
                </div>
                <Card.Header>
                    <Card.Title>Public</Card.Title>
                    <Card.Description class="text-xs"
                        >These ports will be exposed to the world. Useful for
                        things like web apps.</Card.Description
                    >
                </Card.Header>
            </div>
        </Card.Root>
        <Card.Root>
            <div class="flex h-full items-center">
                <div class="ml-4 h-full flex items-center space-x-4">
                    <RadioGroup.Item value="PRIVATE" id="PRIVATE" />
                    <Separator orientation="vertical" />
                </div>
                <Card.Header>
                    <Card.Title>Private</Card.Title>
                    <Card.Description class="text-xs"
                        >These ports will be open to other apps within the same
                        project. This is useful for databases that interact with
                        other applications.</Card.Description
                    >
                </Card.Header>
            </div>
        </Card.Root>
        <Card.Root>
            <div class="flex h-full items-center">
                <div class="ml-4 h-full flex items-center space-x-4">
                    <RadioGroup.Item value="NONE" id="NONE" />
                    <Separator orientation="vertical" />
                </div>
                <Card.Header>
                    <Card.Title>None</Card.Title>
                    <Card.Description class="text-xs"
                        >There will be no open ports and this app will only make
                        outbound initiated connections. This is useful for
                        background workers.</Card.Description
                    >
                </Card.Header>
            </div>
        </Card.Root>
    </div>
</RadioGroup.Root>
<div class="items-center grid grid-cols-8 gap-4">
    {#each ports as port, i}
        <div class="flex items-center space-x-2 col-span-2">
            <Label for="port-name">Name</Label>
            <Input id="port-name" type="text" bind:value={port.name} />
        </div>
        <div class="flex items-center space-x-2 col-span-2">
            <Label for="port-number">Number</Label>
            <Input id="port-number" type="number" bind:value={port.number} />
        </div>
        <div class="flex items-center space-x-2 col-span-2">
            <Label for="http">Protocol</Label>
            <Select.Root bind:selected={port.protocol}>
                <Select.Trigger>
                    <Select.Value />
                </Select.Trigger>
                <Select.Content>
                    {#each protocols as protocol}
                        <Select.Item value={protocol.value}
                            >{protocol.label}</Select.Item
                        >
                    {/each}
                </Select.Content>
            </Select.Root>
        </div>
        <div class="flex items-center justify-end space-x-2">
            <Label for="http">HTTP</Label>
            <Switch id="http" bind:checked={port.http} />
        </div>
        <div class="flex justify-end">
            <Button
                variant="destructive"
                on:click={() =>
                    (ports = ports.filter((_, index) => index !== i))}
                ><Minus class="w-4 h-4" /></Button
            >
        </div>
    {/each}
</div>

<div class="flex justify-end">
    <Button
        on:click={() => {
            ports = [
                ...ports,
                {
                    name: "Extra Port",
                    number: 3000,
                    protocol: { value: 6, label: "TCP" },
                    http: true,
                },
            ];
        }}><Plus class="w-4 h-4" /></Button
    >
</div>
