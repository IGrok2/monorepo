<script>
    import * as RadioGroup from "$lib/components/ui/radio-group";
    import * as Card from "$lib/components/ui/card";
    import { Separator } from "$lib/components/ui/separator";
    import { Input } from "$lib/components/ui/input";
    import { Label } from "$lib/components/ui/label";

    import { Box, Container } from "lucide-svelte";

    export let builder = "NIXPACKS";
    export let installCommand = "";
    export let buildCommand = "";
    export let startCommand = "";
</script>

<RadioGroup.Root bind:value={builder}>
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        <Card.Root>
            <div class="flex h-full items-center">
                <div class="ml-4 h-full flex items-center space-x-4">
                    <RadioGroup.Item value="NIXPACKS" id="NIXPACKS" />
                    <Separator orientation="vertical" />
                </div>
                <Box class="ml-4 h-12 w-12" />
                <Card.Header>
                    <Card.Title>Nixpacks</Card.Title>
                    <Card.Description class="text-xs"
                        >Nixpacks takes a source directory and produces an OCI
                        compliant image that can be deployed anywhere. The
                        project was started by Railway.app and is open source on
                        GitHub.</Card.Description
                    >
                </Card.Header>
            </div>
        </Card.Root>
        <Card.Root>
            <div class="flex h-full items-center">
                <div class="ml-4 h-full flex items-center space-x-4">
                    <RadioGroup.Item value="DOCKERFILE" id="DOCKERFILE" />
                    <Separator orientation="vertical" />
                </div>
                <Container class="ml-4 h-12 w-12" />
                <Card.Header>
                    <Card.Title>Dockerfile</Card.Title>
                    <Card.Description class="text-xs"
                        >A Dockerfile is a text document that contains all the
                        commands a user could call on the command line to
                        assemble an image.</Card.Description
                    >
                </Card.Header>
            </div>
        </Card.Root>
        <Card.Root>
            <div class="flex h-full items-center">
                <div class="ml-4 h-full flex items-center space-x-4">
                    <RadioGroup.Item value="BUILDPACKS" id="BUILDPACKS" />
                    <Separator orientation="vertical" />
                </div>
                <Container class="ml-4 h-12 w-12" />
                <Card.Header>
                    <Card.Title>Buildpacks</Card.Title>
                    <Card.Description class="text-xs"
                        >Contact Packetware support if you'd like to use
                        Buildpacks</Card.Description
                    >
                </Card.Header>
            </div>
        </Card.Root>
    </div>
</RadioGroup.Root>
{#if builder === "NIXPACKS"}
    <h4 class="text-md">Settings</h4>
    <div class="grid grid-cols-3 gap-4">
        <div>
            <Label for="install">Install Command</Label>
            <Input type="string" id="install" placeholder="npm ci" />
        </div>
        <div>
            <Label for="build">Build Command</Label>
            <Input type="string" id="build" placeholder="npm run build" />
        </div>
        <div>
            <Label for="start">Start Command</Label>
            <Input type="string" id="start" placeholder="npm run start" />
        </div>
    </div>
{:else if builder === "Dockerfile"}{/if}
