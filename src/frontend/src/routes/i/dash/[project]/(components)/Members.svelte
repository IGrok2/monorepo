<script>
    import * as Table from "$lib/components/ui/table";
    import { Button } from "$lib/components/ui/button";

    import { User } from "lucide-svelte"

    export let members = [];
    $: console.log(members);
</script>

<Table.Root>
    <Table.Header>
        <Table.Row>
            <Table.Head>ID</Table.Head>
            <Table.Head>Since</Table.Head>
            <Table.Head>Role</Table.Head>
            <Table.Head>Edit</Table.Head>
            <Table.Head></Table.Head>
        </Table.Row>
    </Table.Header>
    <Table.Body>
        {#each members as member}
            <Table.Row>
                <Table.Cell class="font-medium">{member.user_id}</Table.Cell>
                <Table.Cell>Date</Table.Cell>
                <Table.Cell>{member.role}</Table.Cell>
                <Table.Cell class="hover:underline">Edit</Table.Cell>
            </Table.Row>
        {/each}
    </Table.Body>
</Table.Root>
