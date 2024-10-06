<script>
    import { page } from "$app/stores";

    import APIClient from "$lib/utils/api";

    import * as Card from "$lib/components/ui/card";
    import * as Dialog from "$lib/components/ui/dialog";

    import Spinner from "$lib/components/Spinner.svelte";

    import PodLogs from "./(components)/PodLogs.svelte";

    /** @type {import('./$types').PageData} */
    export let data;

    const container = data.container;

    let dialog = false;

    let podsLoading = true;
    let regionPods = [];
    $: console.log("Pods: ", regionPods);

    let selectedPod = "app-667900933da4c5a1f2390cb0-855c468966-bv8tx";
    let selectedRegion = "ASH";

    async function fetchPods() {
        const response = await APIClient.get(
            `@/project/${$page.params.project}/container/${$page.params.id}/pod/list`,
        );

        let res = await response.data;
        console.log("pods res: ", res);

        if (res.success === true) {
            regionPods = res.regionPods;
            podsLoading = false;
        }
    }

    fetchPods();

    // Helpers
    function convertCpuUsage(cpu) {
        let value;
        if (cpu.endsWith("n")) {
            value = parseInt(cpu.slice(0, -1), 10) / 1000000;
        } else if (cpu.endsWith("u")) {
            value = parseInt(cpu.slice(0, -1), 10) / 1000;
        } else if (cpu.endsWith("m")) {
            value = parseInt(cpu.slice(0, -1), 10);
        } else {
            value = parseInt(cpu, 10) * 1000;
        }
        return Math.trunc(value);
    }

    function convertMemoryUsage(memory) {
        let value;
        if (memory.endsWith("Ki")) {
            value = parseInt(memory.slice(0, -2), 10) / 1024;
        } else if (memory.endsWith("Mi")) {
            value = parseInt(memory.slice(0, -2), 10);
        } else if (memory.endsWith("Gi")) {
            value = parseInt(memory.slice(0, -2), 10) * 1024;
        } else if (memory.endsWith("Ti")) {
            value = parseInt(memory.slice(0, -2), 10) * 1024 * 1024;
        } else {
            // Assuming the value is in bytes
            value = parseInt(memory, 10) / (1024 * 1024);
        }
        return Math.trunc(value);
    }
</script>

{#if podsLoading}
    <Spinner />
{:else}
    <section class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
        <Dialog.Root bind:open={dialog}>
            <Dialog.Content class="max-w-[1000px]">
                <Dialog.Header>
                    <Dialog.Title>Pod information</Dialog.Title>
                    <Dialog.Description>
                        Usage statistics and logs for each pod running your
                        application.
                    </Dialog.Description>
                </Dialog.Header>
                <PodLogs bind:selectedPod bind:selectedRegion />
            </Dialog.Content>
        </Dialog.Root>
        {#each regionPods as region}
            {#each region.instances as pod}
                <div
                    on:click={() => {
                        dialog = true;
                        selectedPod = pod.name;
                        selectedRegion = region.regionName;
                    }}
                >
                    <Card.Root class="hover:bg-secondary">
                        <Card.Header>
                            <Card.Title>{pod.name}</Card.Title>
                            <Card.Description
                                >{region.regionName}</Card.Description
                            >
                        </Card.Header>
                        <Card.Content>
                            <p>
                                CPU: {convertCpuUsage(
                                    pod.metrics.cpu,
                                )}/{container.quota.cpu}%
                            </p>
                        </Card.Content>
                        <Card.Footer>
                            <p>
                                Memory: {convertMemoryUsage(
                                    pod.metrics.memory,
                                )}MB/{container.quota.memory}MB
                            </p>
                        </Card.Footer>
                    </Card.Root>
                </div>
            {/each}
        {/each}
    </section>
{/if}
