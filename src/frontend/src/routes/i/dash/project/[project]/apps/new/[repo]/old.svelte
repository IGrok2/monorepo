<div
    class="grid grid-cols-1 gap-x-8 gap-y-10 border-gray-900/10 pb-12 md:grid-cols-3"
>
    <!-- Name of App -->
    <div>
        <h2 class="text-base font-semibold leading-7">Name</h2>
        <p class="mt-1 text-sm leading-6 text-muted-foreground">
            Enter the name you'd like to use for this Packetware App.
        </p>
    </div>

    <div
        class="grid max-w-2xl grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-6 md:col-span-2"
    >
        <div class="sm:col-span-4">
            <div class="mt-2">
                <Input type="text" name="name" id="name" bind:value={name} />
            </div>
        </div>
    </div>

    <!-- Location 
            <div>
                <h2 class="text-base font-semibold leading-7">Location</h2>
                <p class="mt-1 text-sm leading-6 text-muted-foreground">
                    Where should this app be hosted?
                </p>
            </div>

            <div
                class="grid max-w-2xl grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-6 md:col-span-2"
            >
                <div class="sm:col-span-4">
                    <div class="mt-2">
                        <Select.Root bind:selected={location}>
                            <Select.Trigger class="">
                                <Select.Value />
                            </Select.Trigger>
                            <Select.Content>
                                <Select.Item value="ashburn"
                                    >Ashburn, VA</Select.Item
                                >
                            </Select.Content>
                        </Select.Root>
                    </div>
                </div>
            </div>
            -->

    <!-- Runtime
            <div>
                <h2 class="text-base font-semibold leading-7">Runtime</h2>
                <p class="mt-1 text-sm leading-6 text-muted-foreground">
                    What runtime would you like to use for this app?
                </p>
            </div>

            <div
                class="grid max-w-2xl grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-6 md:col-span-2"
            >
                <div class="sm:col-span-4">
                    <RadioGroup.Root id="runtime" bind:value={runtime}>
                        <div class="flex items-center space-x-2">
                            <RadioGroup.Item value="node" id="r1" />
                            <Label for="r1">Node18.x</Label>
                        </div>
                        <RadioGroup.Input name="spacing" />
                    </RadioGroup.Root>
                </div>
            </div>
            -->

    <!-- Branch -->
    <div>
        <h2 class="text-base font-semibold leading-7">Branch</h2>
        <p class="mt-1 text-sm leading-6 text-muted-foreground">
            What Git branch should we listen for pushes on?
        </p>
    </div>

    <div
        class="grid max-w-2xl grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-6 md:col-span-2"
    >
        <div class="sm:col-span-4">
            <Select.Root bind:selected={branch}>
                <Select.Trigger>
                    <Select.Value />
                </Select.Trigger>
                <Select.Content>
                    {#each branches as branch}
                        <Select.Item value={branch.value} label={branch.label}
                            >{branch.label}</Select.Item
                        >
                    {/each}
                </Select.Content>
            </Select.Root>
        </div>
    </div>

    <!-- Subdirectory -->
    <div>
        <h2 class="text-base font-semibold leading-7">App Directory</h2>
        <p class="mt-1 text-sm leading-6 text-muted-foreground">
            Leave this blank if you app is at the root of the repository. This
            allows you to only deploy a subdirectory of your repo. Very helpful
            for monorepos.
        </p>
    </div>

    <div
        class="grid max-w-2xl grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-6 md:col-span-2"
    >
        <div class="sm:col-span-4">
            <div class="mt-2">
                <Input
                    type="text"
                    name="subdirectory"
                    id="subdirectory"
                    placeholder="/"
                    bind:value={subdirectory}
                />
            </div>
        </div>
    </div>

    <!-- App Port -->
    <div>
        <h2 class="text-base font-semibold leading-7">App Port</h2>
        <p class="mt-1 text-sm leading-6 text-muted-foreground">
            What port should your app recieve traffic on?
        </p>
    </div>

    <div
        class="grid max-w-2xl grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-6 md:col-span-2"
    >
        <div class="sm:col-span-4">
            <div class="mt-2">
                <Input type="int" name="port" id="port" bind:value={port} />
            </div>
        </div>
    </div>

    <!-- App Port -->
    <div>
        <h2 class="text-base font-semibold leading-7">Protocol</h2>
        <p class="mt-1 text-sm leading-6 text-muted-foreground">
            What protocol should we expose your app with?
        </p>
    </div>

    <div
        class="grid max-w-2xl grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-6 md:col-span-2"
    >
        <div class="sm:col-span-4">
            <div class="mt-2">
                <Select.Root bind:selected={protocol} portal={null}>
                    <Select.Trigger>
                        <Select.Value placeholder="Select a protocol" />
                    </Select.Trigger>
                    <Select.Content>
                        <Select.Group>
                            <!-- <Select.Label>Protocols</Select.Label> -->
                            {#each protocols as protocol}
                                <Select.Item
                                    value={protocol.value}
                                    label={protocol.label}
                                    >{protocol.label}</Select.Item
                                >
                            {/each}
                        </Select.Group>
                    </Select.Content>
                    <Select.Input name="protocol" />
                </Select.Root>
            </div>
        </div>
    </div>

    <!-- Environment Variables -->
    <div>
        <h2 class="text-base font-semibold leading-7">Environment Variables</h2>
        <p class="mt-1 text-sm leading-6 text-muted-foreground">
            What environment variables should be used for this app? .env
        </p>
    </div>

    <div
        class="grid max-w-2xl grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-6 md:col-span-2"
    >
        <div class="sm:col-span-4">
            {#each variables as variable, index}
                <div class="my-2 flex space-x-4">
                    <Input
                        type="name"
                        placeholder="VARIABLE_NAME"
                        bind:value={variable.key}
                        class=""
                    />
                    <Input
                        type="name"
                        placeholder="Key..."
                        bind:value={variable.value}
                        class=""
                    />
                    <Button
                        on:click={() => {
                            if (variables.length > 1) {
                                variables.splice(index, 1);
                                variables = variables;
                            }
                        }}><X class="w-4 h-4" /></Button
                    >
                </div>
            {/each}
            <Button
                on:click={() => {
                    variables = [
                        ...variables,
                        {
                            key: "",
                            value: "",
                        },
                    ];
                }}><Plus class="w-4 h-4 mr-2" />Add More</Button
            >
        </div>
    </div>
</div>
