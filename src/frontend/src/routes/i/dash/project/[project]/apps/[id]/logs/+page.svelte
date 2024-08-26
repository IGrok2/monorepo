<script>
    import * as Tabs from "$lib/components/ui/tabs";

    import PodLogs from "./PodLogs.svelte";
</script>

<Tabs.Root value="pod" class="">
    <Tabs.List>
        <Tabs.Trigger value="pod">Pod</Tabs.Trigger>
        <Tabs.Trigger value="http">HTTP</Tabs.Trigger>
    </Tabs.List>
    <Tabs.Content value="pod">
        <PodLogs />
    </Tabs.Content>
    <Tabs.Content value="http">HTTP Logs Here</Tabs.Content>
</Tabs.Root>
