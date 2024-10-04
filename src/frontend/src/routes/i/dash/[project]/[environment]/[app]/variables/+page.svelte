<script>
    import { page } from "$app/stores";
    import APIClient from "$lib/utils/api";
    import { toast } from "svelte-sonner";

    import { variables, pending_variables } from "./variables";

    import * as Card from "$lib/components/ui/card";
    import { Button } from "$lib/components/ui/button";
    import { Separator } from "$lib/components/ui/separator";
    import { Input } from "$lib/components/ui/input";

    import DataTable from "./(components)/data-table.svelte";

    import { Download, X, Plus } from "lucide-svelte";

    /** @type {import('./$types').PageData} */
    export let data;

    $:console.log($variables);

    variables.set(data.app.Variables);

    async function addVariables() {
        toast.loading("Attempting to add new variables...");

        const body = {
            variables: $pending_variables,
        };

        try {
            let res = await APIClient.post(
                `/project/${$page.params.project}/environment/${$page.params.environment}/app/${$page.params.app}/variables`,
                body,
            );

            toast.success(res.data.data.message);

            variables.set(res.data.data.variables);
            pending_variables.set([{name: "", value: ""}]);
        } catch (err) {
            console.log(err);
            toast.error(err.response.data.data.message);
        }
    }
</script>

<Card.Root>
    <Card.Header>
        <Card.Title>New Enviromnent Variables</Card.Title>
        <Card.Description
            >Environment variables specified for your project will be used at
            the instant your revision to your production git branch is pushed
            and we build your deployment.</Card.Description
        >
    </Card.Header>
    <Card.Content>
        {#each $pending_variables as variable, index}
            <div class="my-4 flex space-x-4">
                <Input
                    type="name"
                    placeholder="VARIABLE_NAME"
                    bind:value={$pending_variables[index].name}
                    class=""
                />
                <Input
                    type="name"
                    placeholder="Value..."
                    bind:value={$pending_variables[index].value}
                    class=""
                />
                <Button
                    on:click={() => {
                        // Get the current array value
                        $pending_variables = $pending_variables.filter(
                            (_, i) => i !== index,
                        );
                    }}><X class="w-4 h-4" /></Button
                >
            </div>
        {/each}
        <Button
            on:click={() => {
                pending_variables.set([
                    ...$pending_variables,
                    {
                        name: "",
                        value: "",
                    },
                ]);
            }}><Plus class="w-4 h-4 mr-2" />Add More</Button
        >
    </Card.Content>
    <Separator class="my-4" />
    <Card.Footer class="flex justify-between">
        <Button><Download class="w-4 h-4 mr-2" />Import</Button>
        <Button type="submit" on:click={async () => await addVariables()}
            >Save</Button
        >
    </Card.Footer>
</Card.Root>

<DataTable bind:data={$variables} />
