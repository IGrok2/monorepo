<script>
    import Prism from "prismjs";
    import "prismjs/themes/prism.css"; // Import the CSS for syntax highlighting
    import "prismjs/components/prism-bash.min.js"; // Ensure this line is included

    import * as Accordion from "$lib/components/ui/accordion";

    import { GitCommit, ExternalLink } from "lucide-svelte";

    /** @type {import('./$types').PageData} */
    export let data;

    let { app, deployment } = data;

     // Function to strip ANSI escape codes
   function stripAnsiCodes(str) {
       return str.replace(/(?:\r\n|\r|\n|^)(?:\x1B\[[0-9;]*m|\x1B\[K|\x1B\[0m)/g, '');
   }
</script>

<div class="mt-6 flex justify-content space-x-6">
    <div class="space-y-6">
        <!--
            <div class="flex justify-content space-x-6">
                <div>
                    <p class="text-muted-foreground">Status</p>
                    <p class="text.primary">{deployment.Build.Status}</p>
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
            -->
    </div>
</div>
{#if deployment.Build}
    <div class="my-4 px-4 border rounded-xl">
        <Accordion.Root class="w-full">
            <Accordion.Item value="item-1">
                <Accordion.Trigger>Build Logs</Accordion.Trigger>
                <Accordion.Content>
                    <!--<div class="overflow-auto max-h-96">
                        {#each deployment.Build.Logs.split("\n") as line, index}
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
                    </div>-->
                    <pre
                        class="max-h-[400px] overflow-auto whitespace-pre-wrap">
                        <code>
                            {#await Prism.highlight(stripAnsiCodes(deployment.Build.Logs), Prism.languages["bash"], "bash") then highlightedLogs}
                                {@html highlightedLogs}{/await}</code
                        >
                     </pre>
                </Accordion.Content>
            </Accordion.Item>
        </Accordion.Root>
    </div>
{/if}
