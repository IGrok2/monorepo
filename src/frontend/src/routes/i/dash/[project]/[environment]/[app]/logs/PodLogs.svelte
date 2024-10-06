<script lang="ts">
    import { page } from "$app/stores";

    import * as Select from "$lib/components/ui/select/index.js";

    import { Terminal, Container, Clock } from "lucide-svelte";
    import * as Alert from "$lib/components/ui/alert/index.js";

    import Spinner from "$lib/components/Spinner.svelte";

    import { getCookie } from "$lib/utils/auth";
    import { PUBLIC_API } from "$env/static/public";

    const token = getCookie("jwt");

    let podsLoading = true;
    let logsLoading = true;

    let logs;
    let pods = [];
    let selectedPod;

    let timeSince = [
        { value: 1 * 60, label: "1 Minute" },
        { value: 10 * 60, label: "10 Minutes" },
        { value: 60 * 60, label: "1 Hour" },
    ];

    let selectedSince = timeSince[1];

    async function fetchPodLogs(podName, sinceSeconds) {
        const response = await fetch(
            `${PUBLIC_API}/@/project/${$page.params.project}/container/${$page.params.id}/pod/${podName}/logs`,
            {
                method: "POST",
                headers: new Headers({
                    "content-type": "application/json",
                    Authorization: token,
                }),
                body: JSON.stringify({ sinceSeconds }),
            },
        );

        let res = await response.json();
        console.log("logs res", res);

        if (res.success === true) {
            logs = res.logs.slice(0, res.logs.length - 1);
;

            logsLoading = false;
        }
    }

    async function fetchPods() {
        const response = await fetch(
            `${PUBLIC_API}/@/project/${$page.params.project}/container/${$page.params.id}/pod/list`,
            {
                method: "GET",
                headers: new Headers({
                    "content-type": "application/json",
                    Authorization: token,
                }),
            },
        );

        let res = await response.json();
        console.log("pods res", res);

        if (res.success === true) {
            for (let i = 0; i < res.pods.length; i++) {
                const pod = res.pods[i];
                pods.push({ value: pod, label: `Pod ${i}` });
            }

            if (pods.length > 0) {
                selectedPod = pods[0];
                podsLoading = false;
                fetchPodLogs(selectedPod.value, selectedSince.value);
            }
        }
    }

    fetchPods();
</script>

{#if pods.length <= 0 && !podsLoading}
        <Alert.Root>
            <Terminal class="h-4 w-4" />
            <Alert.Title>Heads up!</Alert.Title>
            <Alert.Description
                >Your app is scaled down to <u>0</u> containers because no requests
                are being recieved.</Alert.Description
            >
        </Alert.Root>
{/if}

    <div class="px-4 border rounded">
        {#if podsLoading}
            <div class="p-4">
                <Spinner />
            </div>
        {:else}
            <div class="py-3 border-b flex items-center justify-between">
                <div class="flex items-center space-x-2">
                    <Container class="w-5 h-5 text-muted-foreground" />
                    <Select.Root bind:selected={selectedPod} portal={null}>
                        <Select.Trigger class="w-[100px]">
                            <Select.Value placeholder="Select a pod" />
                        </Select.Trigger>
                        <Select.Content>
                            <Select.Group>
                                {#each pods as pod}
                                    <Select.Item
                                        value={pod.value}
                                        label={pod.label}
                                        on:click={() => {
                                            selectedPod = pod;
                                            logsLoading = true;
                                            fetchPodLogs(
                                                pod.value,
                                                selectedSince.value,
                                            );
                                        }}>{pod.label}</Select.Item
                                    >
                                {/each}
                            </Select.Group>
                        </Select.Content>
                        <Select.Input name="selectedPod" />
                    </Select.Root>
                </div>
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
                                                selectedPod.value,
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
        {/if}
    </div>
