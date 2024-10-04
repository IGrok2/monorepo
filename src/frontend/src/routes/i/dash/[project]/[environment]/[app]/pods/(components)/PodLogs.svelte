<script lang="ts">
    import { page } from "$app/stores";

    import * as Select from "$lib/components/ui/select/index.js";

    import { Terminal, Container, Clock } from "lucide-svelte";
    import * as Alert from "$lib/components/ui/alert/index.js";

    import Spinner from "$lib/components/Spinner.svelte";

    import { getCookie } from "$lib/utils/auth";
    import { PUBLIC_API } from "$env/static/public";

    const token = getCookie("jwt");

    export let selectedPod;
    export let selectedRegion;
    $: console.log(selectedRegion);

    let logsLoading = true;

    let logs;

    let timeSince = [
        { value: 1 * 60, label: "1 Minute" },
        { value: 10 * 60, label: "10 Minutes" },
        { value: 60 * 60, label: "1 Hour" },
    ];

    let selectedSince = timeSince[1];

    async function fetchPodLogs(regionName, podName, sinceSeconds) {
        const response = await fetch(
            `${PUBLIC_API}/@/project/${$page.params.project}/container/${$page.params.id}/pod/${podName}/logs`,
            {
                method: "POST",
                headers: new Headers({
                    "content-type": "application/json",
                    Authorization: token,
                }),
                body: JSON.stringify({ regionName, sinceSeconds }),
            },
        );

        let res = await response.json();
        console.log("logs res: ", res);

        if (res.success === true) {
            logs = res.logs.slice(0, res.logs.length - 1);
            logsLoading = false;
        }
    }

    fetchPodLogs(selectedRegion, selectedPod, selectedSince.value);
</script>

<div class="px-4 border rounded">
    <div class="py-3 border-b flex items-center justify-end">
        <div class="flex items-center space-x-2">
            <Clock class="w-5 h-5 text-muted-foreground" />
            <Select.Root bind:selected={selectedSince} portal={null}>
                <Select.Trigger class="w-[150px]">
                    <Select.Value placeholder="Select since" />
                </Select.Trigger>
                <Select.Content>
                    <Select.Group>
                        {#each timeSince as since}
                            <Select.Item
                                value={since.value}
                                label={since.label}
                                on:click={() => {
                                    logsLoading = true;
                                    fetchPodLogs(
                                        selectedRegion,
                                        selectedPod,
                                        since.value,
                                    );
                                }}>{since.label}</Select.Item
                            >
                        {/each}
                    </Select.Group>
                </Select.Content>
                <Select.Input name="selectedSince" />
            </Select.Root>
        </div>
    </div>

    <div class="mt-2 max-h-96 overflow-y-auto">
        {#if logsLoading}
            <div class="p-4">
                <Spinner />
            </div>
        {:else if logs}
            {#each logs as log}
                <div class="flex items-start space-x-4 flex-wrap">
                    <div
                        class="w-[170px] text-sm font-medium text-muted-foreground"
                    >
                        {new Date(log.timestamp).toLocaleString()}
                    </div>
                    <div class="flex-1 text-sm text-foreground">
                        {log.message}
                    </div>
                </div>
            {/each}
        {/if}
    </div>
</div>
