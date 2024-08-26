<script>
    import { page } from "$app/stores";
    import { onMount } from "svelte";

    import * as Accordion from "$lib/components/ui/accordion";

    import { GitCommit, ExternalLink } from "lucide-svelte";

    import Preview from "$lib/assets/human_engine.png";

    import { PUBLIC_API } from "$env/static/public";

    import { getCookie } from "$lib/utils/auth";

    const token = getCookie("jwt");

    /** @type {import('./$types').PageData} */
    export let data;

    let container = data.container;
    let deployment;

    let durationInSeconds = 0;

    async function fetchDeployment() {
        const response = await fetch(
            `${PUBLIC_API}/@/project/${$page.params.project}/container/${$page.params.id}/deployment/${$page.params.deployment}`,
            {
                method: "GET",
                headers: new Headers({
                    "content-type": "application/json",
                    Authorization: token,
                }),
            },
        );

        const res = await response.json();
        console.log(res);

        deployment = res.deployment;

        durationInSeconds = Math.floor(deployment.build.duration / 1000);

        return res.deployment;
    }

    onMount(() => {
        fetchDeployment();
    });
</script>

{#if deployment}
    <div class="mt-6 flex justify-content space-x-6">
        <div class="space-y-6">
            <div class="flex justify-content space-x-6">
                <div>
                    <p class="text-muted-foreground">Status</p>
                    <p class="text.primary">{deployment.build.status}</p>
                </div>
                <div>
                    <p class="text-muted-foreground">Created</p>
                    <p class="text.primary">
                        {deployment.created}
                    </p>
                </div>
                <div>
                    <p class="text-muted-foreground">Duration</p>
                    {#if !deployment.build.in_progress}
                        <p class="text.primary">
                            {durationInSeconds} Seconds
                        </p>
                    {:else}
                        <p class="text.primary">In Progress</p>
                    {/if}
                </div>
            </div>
            <div>
                <p class="text-muted-foreground">Source</p>
                <a
                    href="https://github.com/{container.source.github.login}/{container.source
                        .github.repo.name}/commit/{deployment.build.revision}"
                    class="text.primary"
                    ><GitCommit class="w-4 h-4 inline" />
                    {deployment.build.revision.substring(0, 7)}</a
                >
            </div>
        </div>
    </div>

    <div class="my-4 px-4 border rounded-xl">
        <Accordion.Root class="w-full">
            <Accordion.Item value="item-1">
                <Accordion.Trigger>Build Logs</Accordion.Trigger>
                <Accordion.Content>
                    <div class="overflow-auto max-h-96">
                        {#each deployment.build.logs.split("\n") as line, index}
                            <p class="flex">
                                <span class="w-10 text-right mr-4 overflow-auto"
                                    >{index}</span
                                >
                                <span
                                    class="overflow-auto whitespace-pre-wrap break-words"
                                    >{line}</span
                                >
                            </p>
                        {/each}
                    </div>
                </Accordion.Content>
            </Accordion.Item>
        </Accordion.Root>
    </div>
{/if}

<style>
    .line-number {
        display: inline-block;
        width: 2em; /* adjust as needed */
        background-color: #f0f0f0; /* adjust as needed */
        text-align: right;
        margin-right: 1em;
        overflow: auto;
    }
    .log-content {
        overflow: auto;
        white-space: pre-wrap; /* preserve spaces and line breaks */
        word-wrap: break-word; /* break words for long lines */
    }
</style>
