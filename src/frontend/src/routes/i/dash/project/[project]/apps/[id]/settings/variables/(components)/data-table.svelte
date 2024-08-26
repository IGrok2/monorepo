<script lang="ts">
	import { page } from "$app/stores";
	import appsAPIClient from "$lib/utils/apps";

	import {
		createTable,
		Render,
		Subscribe,
		createRender,
	} from "svelte-headless-table";
	import {
		addPagination,
		addSortBy,
		addTableFilter,
		addHiddenColumns,
		addSelectedRows,
	} from "svelte-headless-table/plugins";
	import { readable } from "svelte/store";
	import * as Table from "$lib/components/ui/table";
	import DataTableActions from "./data-table-actions.svelte";
	import { Button, buttonVariants } from "$lib/components/ui/button";
	import { Eye, EyeOff } from "lucide-svelte";
	import { Input } from "$lib/components/ui/input";
	import * as Select from "$lib/components/ui/select";
	import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
	import DataTableCheckbox from "./data-table-checkbox.svelte";

    import { toast } from "svelte-sonner";

	export let data = [
		{
			_id: 1,
			key: "key",
			value: "value",
		},
		{
			_id: 2,
			key: "key",
			value: "value",
		},
	];

	const table = createTable(readable(data), {
		sort: addSortBy({ disableMultiSort: true }),
		filter: addTableFilter({
			fn: ({ filterValue, value }) => value.includes(filterValue),
		}),
		hide: addHiddenColumns(),
		select: addSelectedRows(),
	});
	const columns = table.createColumns([
		table.column({
			accessor: "_id",
			header: (_, { pluginStates }) => {
				const { allPageRowsSelected } = pluginStates.select;
				return createRender(DataTableCheckbox, {
					checked: allPageRowsSelected,
				});
			},
			cell: ({ row }, { pluginStates }) => {
				const { getRowState } = pluginStates.select;
				const { isSelected } = getRowState(row);

				return createRender(DataTableCheckbox, {
					checked: isSelected,
				});
			},
			plugins: {
				filter: {
					exclude: true,
				},
			},
		}),
		table.column({
			accessor: "key",
			header: "Key",
			plugins: {
				filter: {
					exclude: false,
				},
			},
		}),
		table.column({
			accessor: "value",
			header: "Value",
			plugins: {
				filter: {
					exclude: true,
				},
			},
		}),
		table.column({
			accessor: ({ _id }) => _id,
			header: "",
			cell: ({ value }) => {
				return createRender(DataTableActions, { _id: value });
			},
		}),
	]);

	const {
		headerRows,
		pageRows,
		tableAttrs,
		tableBodyAttrs,
		pluginStates,
		flatColumns,
		rows,
	} = table.createViewModel(columns);
	const { filterValue } = pluginStates.filter;
	const { selectedDataIds } = pluginStates.select;

	export async function deleteSelectedVariables(selected) {
		let indexes = Object.keys(selected);
		let variables = indexes.map((index) => data[index]._id);
		console.log(variables);

		let reponse = await appsAPIClient.delete(`/@/project/${$page.params.project}/container/${$page.params.id}/variable`, {
			data: { variables }
		});
		let deleted = response.data;
		console.log(deleted);

		console.log(data);

		if (deleted.success === true) {
			// Filter the data array to remove the deleted variables
			data = data.filter((item) => !variables.includes(item._id));

			toast.success(deleted.message);
		}

		console.log(data)

		toast.error(deleted.message);

		window.location.reload();
	}
</script>

<div>
	<div class="flex justify-between py-4">
		<Input
			class="max-w-sm"
			placeholder="Filter variables..."
			type="text"
			bind:value={$filterValue}
		/>
		{#if Object.keys($selectedDataIds).length > 0}
			<Button
				variant="destructive"
				on:click={deleteSelectedVariables($selectedDataIds)}
				>Delete Selected</Button
			>
		{/if}
	</div>
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
									let:props
								>
									<Table.Head
										{...attrs}
										class="[&:has([role=checkbox])]:pl-3"
									>
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
							data-state={$selectedDataIds[row.id] && "selected"}
						>
							{#each row.cells as cell (cell.id)}
								<Subscribe attrs={cell.attrs()} let:attrs>
									<Table.Cell {...attrs}>
										{#if cell.id === "value"}
											<span>********</span>
										{:else if cell.id === "show"}<Eye
											/>{:else}
											<Render of={cell.render()} />
										{/if}
									</Table.Cell>
								</Subscribe>
							{/each}
						</Table.Row>
					</Subscribe>
				{/each}
			</Table.Body>
		</Table.Root>
	</div>
	<div class="flex items-center justify-end space-x-4 py-4">
		<div class="flex-1 text-sm text-muted-foreground">
			{Object.keys($selectedDataIds).length} of{" "}
			{$rows.length} row(s) selected.
		</div>
	</div>
</div>
