<script>
    import { Input } from "$lib/components/ui/input";
    import { Button } from "$lib/components/ui/button";

    import { X, Plus } from "lucide-svelte";

    export let variables = [{ name: "", value: "" }];
</script>

<div>
    <h2 class="text-base font-semibold leading-7">
        Environment Variables
    </h2>
    <p class="mt-1 text-sm leading-6 text-muted-foreground">
        What environment variables should be used for this
        app? .env
    </p>

    {#each variables as variable, index}
        <div class="my-2 flex space-x-4">
            <Input
                type="string"
                placeholder="VARIABLE_NAME"
                bind:value={variable.name}
                class=""
            />
            <Input
                type="string"
                placeholder="Value..."
                bind:value={variable.value}
                class=""
            />
            <Button
                variant='destructive'
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
                    name: "",
                    value: "",
                },
            ];
        }}><Plus class="w-4 h-4 mr-2" />Add More</Button
    >
</div>
