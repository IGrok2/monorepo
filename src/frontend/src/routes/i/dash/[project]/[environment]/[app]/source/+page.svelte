<script>
    import * as RadioGroup from "$lib/components/ui/radio-group";
    import * as Card from "$lib/components/ui/card";
    import { Button } from "$lib/components/ui/button";
    import { Separator } from "$lib/components/ui/separator";

    import { GitRepo, Builder } from "$lib/components/app";

    /** @type {import('./$types').PageData} */
    export let data;
    const { app } = data;

    let channel = app.Source.Channel;

    // Git
    let namespace;
    let repo;
    let branch;
    let subdirectory = "";

    // TODO Image
</script>

<div class="space-y-4">
    <Card.Root class="w-full mx-auto">
        <Card.Header>
            <Card.Title>Manage Source</Card.Title>
            <Card.Description>How should we get your app</Card.Description>
        </Card.Header>
        <Card.Content class="space-y-6">
            <RadioGroup.Root bind:value={channel}>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <Card.Root>
                        <div class="flex h-full items-center">
                            <div
                                class="ml-4 h-full flex items-center space-x-4"
                            >
                                <RadioGroup.Item value="GITHUB" id="GITHUB" />
                                <Separator orientation="vertical" />
                            </div>
                            <Card.Header>
                                <Card.Title>Code Repository</Card.Title>
                                <Card.Description
                                    >Pull and build your code into a app from
                                    Github.</Card.Description
                                >
                            </Card.Header>
                        </div>
                    </Card.Root>
                    <Card.Root>
                        <div class="flex h-full items-center">
                            <div
                                class="ml-4 h-full flex items-center space-x-4"
                            >
                                <!-- Radio Button -->
                                <RadioGroup.Item value="IMAGE" id="IMAGE" />
                                <!-- Visible Vertical Separator -->
                                <Separator orientation="vertical" />
                            </div>
                            <Card.Header>
                                <Card.Title>Container Image</Card.Title>
                                <Card.Description
                                    >Pull a artifact from a repository and we
                                    will run it.</Card.Description
                                >
                            </Card.Header>
                        </div>
                    </Card.Root>
                </div>
            </RadioGroup.Root>
            {#if channel === "GITHUB"}
                <GitRepo />
                <Button class="w-full">Save Settings</Button>
            {:else if channel === "IMAGE"}
                <!-- <ImageSource bind:channel bind:image_url />-->
                <Button class="w-full">Save Settings</Button>
            {/if}
        </Card.Content>
    </Card.Root>
    {#if channel === "GITHUB"}
        <Card.Root class="w-full mx-auto">
            <Card.Header>
                <Card.Title>Builder Settings</Card.Title>
                <Card.Description
                    >If we need to build your code into an image how should we
                    do it.</Card.Description
                >
            </Card.Header>
            <Card.Content class="space-y-6">
                <Builder />
                <Button class="w-full">Save Settings</Button>
            </Card.Content>
        </Card.Root>
    {/if}
</div>
