<script>
    import { page } from "$app/stores";

    import { Button } from "$lib/components/ui/button";
    import * as Card from "$lib/components/ui/card";
    import * as Alert from "$lib/components/ui/alert";

    import { Github, GitCommit, ExternalLink, Terminal } from "lucide-svelte";

    /** @type {import('./$types').PageData} */
    export let data;
    $: console.log("data: ", data);

    let app = data.app;
    let deployment = app.Active;
</script>

<!--
{#if deployment}
    <div class="my-4 flex justify-between">
        <div class="text-2xl font-bold">Current Deployment</div>
        <Button href="https://app-{app._id}.onpacketware.net"
            >Visit <ExternalLink class="h-4 w-4 ml-2" /></Button
        >
    </div>
{:else}
    <div class="text-2xl font-bold">No Deployments</div>
{/if}
-->
<section class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
    <Card.Root
        ><Card.Header><Card.Title>App Information</Card.Title></Card.Header
        ><Card.Content class="space-y-2">
            <div>
                <span class="text-sm font-medium text-muted-foreground"
                    >App Name</span
                >
                <div class="text-lg font-semibold">{app.Slug}</div>
            </div>
            <div>
                <span class="text-sm font-medium text-muted-foreground"
                    >Project Name</span
                >
                <div class="text-lg font-semibold">{$page.params.project}</div>
            </div>
            <!--
            <div>
                <span class="text-sm font-medium text-muted-foreground"
                    >Git Repo</span
                >
                <div class="text-lg font-semibold truncate">
                    https://github.com/{app.source.github.login}/{app.source.github.repo.name}
                </div>
            </div>
            -->
            <div>
                <span class="text-sm font-medium text-muted-foreground"
                    >Domains</span
                >
                <div class="text-lg font-semibold truncate">
                    https://{app.Slug}.{$page.params.project}.packetware.app
                </div>
            </div>
        </Card.Content></Card.Root
    >
</section>
<!--
    <Card.Root
        ><Card.Header><Card.Title>Resource Usage</Card.Title></Card.Header>
        <Card.Content class="space-y-2">
            <div>
                <span class="text-sm font-medium text-muted-foreground"
                    >Max Number of Replicas</span
                >
                <div class="text-lg font-semibold">1</div>
            </div>
            <div>
                <span class="text-sm font-medium text-muted-foreground"
                    >CPU Usage</span
                >
                <div class="text-lg font-semibold">12% / 100%</div>
            </div>
            <div>
                <span class="text-sm font-medium text-muted-foreground"
                    >Memory Usage</span
                >
                <div class="text-lg font-semibold">503Mb / 1024Mb</div>
            </div>
            <div>
                <span class="text-sm font-medium text-muted-foreground"
                    >Billing</span
                >
                <div class="text-lg font-semibold wrap">$5 / Month</div>
            </div>
        </Card.Content></Card.Root
    >
-->
{#if app.Active.Channel === "GITHUB"}
    <Card.Root
        ><Card.Header><Card.Title>Deployment Details</Card.Title></Card.Header
        ><Card.Content class="space-y-2">
            <div>
                <span class="text-sm font-medium text-muted-foreground"
                    >Build Logs</span
                >
                <div class="text-lg font-semibold">
                    <a
                        href="{$page.params.app}/deployments/{app.Active.ID}"
                        class="href"
                        >View logs<ExternalLink
                            class="ml-1 w-4 h-4 inline"
                        /></a
                    >
                </div>
            </div>
        </Card.Content></Card.Root
    >
{/if}
<!--
</section>
    <div class="my-4">
        <Card.Root>
            <Card.Content>
                <div class="mt-6 flex justify-content space-x-6">
                    <img
                        src="https://www.google.com/url?sa=i&url=https%3A%2F%2Fdeveloper.mozilla.org%2Fen-US%2Fdocs%2FLearn%2FCommon_questions%2FWeb_mechanics%2FPages_sites_servers_and_search_engines&psig=AOvVaw1bsuzfLZ755afuDPrLI5Rm&ust=1710121637387000&source=images&cd=vfe&opi=89978449&ved=0CBMQjRxqFwoTCNi6tdfJ6IQDFQAAAAAdAAAAABAE"
                        alt="Site preview of {app.github.repo_name}."
                        class="rounded-md max-w-md"
                    />
                    <div class="space-y-6">
                        <div>
                            <p class="text-muted-foreground">Revision Link</p>
                            <a
                                href="http://app-{app._id}.onpacketware.net:31080"
                                class=""
                                >app-{app._id}.onpacketware.net
                                <ExternalLink class="w-4 h-4 inline" /></a
                            >
                        </div>
                        <div class="flex justify-content space-x-6">
                            <div>
                                <p class="text-muted-foreground">Status</p>
                                <p class="">
                                    {deployment.status}
                                </p>
                            </div>
                        </div>
                        <div>
                            <p class="text-muted-foreground">Source</p>
                            <a
                                href="https://github.com/{app.github
                                    .login}/{app.github
                                    .repo_name}/commit/{deployment.commit_id}"
                                class=""
                                ><GitCommit class="w-4 h-4 inline" />
                                {deployment.commit_id.substring(0, 7)} - {deployment.message}</a
                            >
                        </div>
                    </div>
                </div>
            </Card.Content>
        </Card.Root>
    </div>->

<Alert.Root class="my-4 py-4">
    <Terminal class="h-4 w-4" />
    <Alert.Title>Heads up!</Alert.Title>
    <Alert.Description
        >You can create new deployments to your project by pushing commits to
        your main branch.</Alert.Description
    >
</Alert.Root>
-->
