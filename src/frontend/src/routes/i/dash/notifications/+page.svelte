<script>
    import * as Table from "$lib/components/ui/table";
    import { Button } from "$lib/components/ui/button";
    import { Checkbox } from "$lib/components/ui/checkbox";
    import * as Pagination from "$lib/components/ui/pagination";

    import Spinner from "$lib/components/Spinner.svelte";

    import { toast } from "svelte-sonner";

    import meAPIClient from "$lib/utils/me";

    let loaded = false;

    let page = 1;
    let pageSize = 10;
    let count = 0;

    let notifications = [];

    async function getNotifications() {
        const start = (page * pageSize) - pageSize; // Calculate the start index
        const end = start + pageSize; // Calculate the end index

        const response = await meAPIClient.post(`/notifications`, {
            start,
            end,
        });

        let res = response.data;

        if (res.success) {
            notifications = res.notifications;
            count = res.count;
            loaded = true;
        }
    }

    getNotifications();

    function changePage() {
        console.log(page);
        loaded = false;
        getNotifications();
    }

    async function dismissNotifications() {
        toast.loading("Marking selected notifications as seen...");

        let selectedNotifications = notifications.filter(
            (notification) => notification.selected === true,
        );

        const response = await meAPIClient.post(`/notifications/seen`, {
            seen: selectedNotifications.map(obj => obj._id),
        });

        const res = response.data;

        if (res.success) {
            toast.success("Notifications marked as seen.")
            loaded = false;
            getNotifications();
        } else {
            toast.error("Error marking notifications as seen.");
        }
    }
</script>

<div class="container">
    <div class="mb-8">
        <h1 class="text-3xl font-bold">User Notifications</h1>
        <p class="text-muted-foreground">Review and clear notifications.</p>
        <div class="flex justify-end">
            <Button on:click={dismissNotifications}>Mark Selected As Seen</Button>
        </div>
    </div>
    <div class="space-y-4">
        {#if !loaded}
            <Spinner />
        {:else}
            <Table.Root>
                <Table.Header>
                    <Table.Row>
                        <Table.Head class="w-[100px]">Select</Table.Head>
                        <Table.Head>Message</Table.Head>
                        <Table.Head>Seen</Table.Head>
                        <Table.Head class="text-right">Link</Table.Head>
                    </Table.Row>
                </Table.Header>
                <Table.Body>
                    {#each notifications as notification}
                        <Table.Row>
                            <Table.Cell class="font-medium"
                                ><Checkbox bind:checked={notification.selected} /></Table.Cell
                            >
                            <Table.Cell>{notification.message}</Table.Cell>
                            <Table.Cell>{notification.seen}</Table.Cell>
                            <Table.Cell class="text-right"
                                ><a href={notification.link} class="underline">View</a></Table.Cell
                            >
                        </Table.Row>
                    {/each}
                </Table.Body>
            </Table.Root>
        {/if}
    </div>
    <Pagination.Root bind:page={page} count={count} perPage={pageSize} let:pages let:currentPage onPageChange={changePage()}>
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
                <Pagination.Link {page} isActive={currentPage == page.value}>
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
</div>
