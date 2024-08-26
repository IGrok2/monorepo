<script>
    import { formatDistanceToNow } from "date-fns";

    import * as Popover from "$lib/components/ui/popover";
    import { Separator } from "$lib/components/ui/separator";
    import { Button } from "$lib/components/ui/button";
    //import { ScrollArea } from "$lib/components/ui/scroll-area";

    import { Bell, BellDot } from "lucide-svelte";
    import Spinner from "$lib/components/Spinner.svelte";

    import meAPIClient from "$lib/utils/me";

    let loading = true;

    function formatDate(date) {
        return formatDistanceToNow(date, { addSuffix: true });
    }

    let alert = false;
    let notificationsLoaded = false;
    let notifications;

    async function fetchNotifications() {
        const response = await meAPIClient.post(`/notifications`, {
                start: 0,
                end: 9,
            }
        );

        let res = await response.data;
        console.log(res);

        if (res.success) {
            notifications = res.notifications;
            notificationsLoaded = true;

            loading = false;
        } else {
            toast.error("Error loading account notifications.");
        }
    }

    fetchNotifications();
</script>

{#if !loading}
    <Popover.Root>
        <Popover.Trigger
            ><Button variant="outline"
                >{#if alert}<Bell class="h-[1.2rem] w-[1.2rem]" />{:else}<Bell
                        class="h-[1.2rem] w-[1.2rem]"
                    />{/if}</Button
            ></Popover.Trigger
        >
        <Popover.Content class="h-72 w-72 overflow-y-auto mt-2">
            <div class="flex space-x-2 items-center">
                <Bell class="h-[2rem] w-[2rem]" />
                <h4 class="text-3xl font-bold">Notifications</h4>
            </div>
            <Separator class="my-2" />
            {#if !notificationsLoaded}
                <Spinner />
            {:else if notifications.length > 0}
                {#each notifications as notification}
                    <div class="flex items-center space-x-2">
                        <svg
                            class="h-4 w-4 text-destructive"
                            fill="currentColor"
                            viewBox="0 0 20 20"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <circle cx="10" cy="10" r="10" />
                        </svg>
                        <div class="flex-1">
                            <p>{notification.message}</p>
                            <p class="text-muted-foreground text-xs">
                                {formatDate(notification.date)}
                            </p>
                        </div>
                        {#if notification.link}
                            <a
                                href={notification.link}
                                class="text-xs hover:underline">View</a
                            >
                        {/if}
                    </div>
                {/each}
            {:else}
                <p class="text-muted-foreground">No new notifications</p>
            {/if}
            <Separator class="my-2" />
            <div class="flex justify-center">
                <Button href="/i/dash/notifications" variant="ghost"
                    ><span class="underline">See All Notifications</span
                    ></Button
                >
            </div>
        </Popover.Content>
    </Popover.Root>
{/if}
