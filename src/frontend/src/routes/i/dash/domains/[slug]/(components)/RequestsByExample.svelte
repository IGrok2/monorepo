<script>
    import { createTable, Render, Subscribe } from "svelte-headless-table";
    import { readable } from "svelte/store";
    import * as Table from "$lib/components/ui/table";
    import * as Sheet from "$lib/components/ui/sheet";

    export let data = [
        {
            id: "728ed52f",
            amount: 100,
            status: "pending",
            email: "m@example.com",
        },
        {
            id: "489e1d42",
            amount: 125,
            status: "processing",
            email: "example@gmail.com",
        },
    ];

    let sheetOpen = false;
    let selectedRequest = data[0];

    const table = createTable(readable(data));

    const columns = table.createColumns([
        table.column({
            accessor: "date",
            header: "Time",
        }),
        table.column({
            accessor: "ip",
            header: "IP",
        }),
        table.column({
            accessor: "method",
            header: "Method",
        }),
        table.column({
            accessor: "response_code",
            header: "Status",
        }),
        table.column({
            accessor: "path",
            header: "Path",
        }),
    ]);

    const { headerRows, pageRows, tableAttrs, tableBodyAttrs } =
        table.createViewModel(columns);
</script>

<div class="rounded-md border">
    <Table.Root {...$tableAttrs}>
        <Table.Header>
            {#each $headerRows as headerRow}
                <Subscribe rowAttrs={headerRow.attrs()}>
                    <Table.Row>
                        {#each headerRow.cells as cell (cell.id)}
                            <Subscribe
                                attrs={cell.attrs()}
                                let:attrs
                                props={cell.props()}
                            >
                                <Table.Head {...attrs}>
                                    <Render of={cell.render()} />
                                </Table.Head>
                            </Subscribe>
                        {/each}
                    </Table.Row>
                </Subscribe>
            {/each}
        </Table.Header>
        <Table.Body {...$tableBodyAttrs}>
            {#each $pageRows as row (row.id)}
                <Subscribe rowAttrs={row.attrs()} let:rowAttrs>
                    <Table.Row
                        {...rowAttrs}
                        on:click={() => (sheetOpen = true)}
                    >
                        {#each row.cells as cell (cell.id)}
                            <Subscribe attrs={cell.attrs()} let:attrs>
                                <Table.Cell {...attrs}>
                                    <Render of={cell.render()} />
                                </Table.Cell>
                            </Subscribe>
                        {/each}
                    </Table.Row>
                </Subscribe>
            {/each}
        </Table.Body>
    </Table.Root>
</div>

<Sheet.Root bind:open={sheetOpen}>
    <Sheet.Content>
        <Sheet.Header>
            <Sheet.Title>Are you sure absolutely sure?</Sheet.Title>
            <Sheet.Description>
                This action cannot be undone. This will permanently delete your
                account and remove your data from our servers.
            </Sheet.Description>
        </Sheet.Header>
    </Sheet.Content>
</Sheet.Root>
