<script>
    import { page } from "$app/stores";

    import appsAPIClient from "$lib/utils/apps";

    import * as Card from "$lib/components/ui/card";
    import { Button } from "$lib/components/ui/button";
    import { Separator } from "$lib/components/ui/separator";
    import { Input } from "$lib/components/ui/input";

    import DataTable from "./(components)/data-table.svelte";

    import { Download, X, Plus } from "lucide-svelte";

    import { toast } from "svelte-sonner";

    let id = $page.params.id;

    /** @type {import('./$types').PageData} */
    export let data;

    let env = data.container.variables;

    let pending_env = [
        {
            key: "",
            value: "",
        },
    ];

    async function createEnvironmentVariables(variables) {
        const response = await appsAPIClient.post(`/@/project/${$page.params.project}/container/${$page.params.id}/variable`, {
            variables
        });

        let res = response.data;
        console.log(res.message)

        if (res.success === true) {
            pending_env = [
                {
                    key: "",
                    value: "",
                },
            ];

            env.push(...variables);
            console.log(env);

            toast.success(res.message);
        } else {
            toast.error(res.message);
        }
        
        window.location.reload();
    }
</script>

<form
    method="POST"
    on:submit|preventDefault={createEnvironmentVariables(pending_env)}
>
    <Card.Root>
        <Card.Header>
            <Card.Title>New Enviromnent Variables</Card.Title>
            <Card.Description
                >Environment variables specified for your project will be used
                at the instant your revision to your production git branch is
                pushed and we build your deployment.</Card.Description
            >
        </Card.Header>
        <Card.Content>
            {#each pending_env as env, index}
                <div class="my-4 flex space-x-4">
                    <Input
                        type="name"
                        placeholder="VARIABLE_NAME"
                        bind:value={env.key}
                        class=""
                    />
                    <Input
                        type="name"
                        placeholder="Key..."
                        bind:value={env.value}
                        class=""
                    />
                    <Button
                        on:click={() => {
                            if (pending_env.length > 1) {
                                pending_env.splice(index, 1);
                                pending_env = pending_env;
                            }
                        }}><X class="w-4 h-4" /></Button
                    >
                </div>
            {/each}
            <Button
                on:click={() => {
                    pending_env = [
                        ...pending_env,
                        {
                            key: "",
                            value: "",
                        },
                    ];
                }}><Plus class="w-4 h-4 mr-2" />Add More</Button
            >
        </Card.Content>
        <Separator class="my-4" />
        <Card.Footer class="flex justify-between">
            <Button><Download class="w-4 h-4 mr-2" />Import</Button>
            <Button type="submit">Save</Button>
        </Card.Footer>
    </Card.Root>
</form>

<DataTable bind:data={env} />
