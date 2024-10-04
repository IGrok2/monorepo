<script>
    import { page } from "$app/stores";
    import * as Table from "$lib/components/ui/table";
    import * as Pagination from "$lib/components/ui/pagination";

    import Spinner from "$lib/components/Spinner.svelte";

    import { toast } from "svelte-sonner";

    import projectsAPIClient from "$lib/utils/projects";

    export let activity = [];

    let pageNumber = 1;
    let pageSize = 10;
    let count = activity.length;
    $: console.log(pageNumber);

    // Calculate the start index of the page
    $: startIndex = (pageNumber - 1) * pageSize;

    // Slice the activity array to get the pageActivity
    $: pageActivity = activity.slice(startIndex, startIndex + pageSize);

    /*
    async function getActivity() {
        const start = (pageNumber * pageSize) - pageSize; // Calculate the start index
        const end = start + pageSize; // Calculate the end index

        const response = await projectsAPIClient.post(
            `/@/project/${$page.params.project}/activity`,
            {
                start,
                end,
            },
        );

        let res = response.data;

        if (res.success) {
            activity = res.activity;
            count = res.count;
            loaded = true;
        }
    }

    getActivity();*/

    function changePage() {
        console.log(pageNumber);
    }
</script>

<Table.Root>
    <Table.Caption>A list of recent activities.</Table.Caption>
    <Table.Header>
        <Table.Row>
            <Table.Head>Date</Table.Head>
            <Table.Head>Type</Table.Head>
            <Table.Head>Resource</Table.Head>
            <Table.Head>Message</Table.Head>
            <Table.Head>User</Table.Head>
        </Table.Row>
    </Table.Header>
    <Table.Body>
        {#each pageActivity as log (log.date)}
            <Table.Row>
                <Table.Cell class="font-medium"
                    >{log.date.toLocaleString()}</Table.Cell
                >
                <Table.Cell>{log.type}</Table.Cell>
                <Table.Cell>{log.resource}</Table.Cell>
                <Table.Cell>{log.message}</Table.Cell>
                <Table.Cell>{log.user.name}</Table.Cell>
            </Table.Row>
        {/each}
    </Table.Body>
</Table.Root>
<Pagination.Root
    bind:page={pageNumber}
    {count}
    perPage={pageSize}
    let:pages
    let:currentPage
    onPageChange={changePage()}
>
    <Pagination.Content>
        <Pagination.Item>
            <Pagination.PrevButton />
        </Pagination.Item>
        {#each pages as page (page.key)}
            {#if page.type === "ellipsis"}
                <Pagination.Item>
                    <Pagination.Ellipsis />
                </Pagination.Item>
            {:else}
                <Pagination.Item isVisible={currentPage == page.value}>
                    <Pagination.Link
                        {page}
                        isActive={currentPage == page.value}
                    >
                        {page.value}
                    </Pagination.Link>
                </Pagination.Item>
            {/if}
        {/each}
        <Pagination.Item>
            <Pagination.NextButton />
        </Pagination.Item>
    </Pagination.Content>
</Pagination.Root>
