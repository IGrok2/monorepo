<script lang="ts">
	import { page } from "$app/stores";
	import APIClient from "$lib/utils/api";

	import { variables } from "../variables";

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
			ID: 1,
			Name: "key",
			Value: "value",
		},
		{
			ID: 2,
			Name: "key",
			Value: "value",
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
			accessor: "ID",
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
			accessor: "Name",
			header: "Name",
			plugins: {
				filter: {
					exclude: false,
				},
			},
		}),
		table.column({
			accessor: "Value",
			header: "Value",
			plugins: {
				filter: {
					exclude: true,
				},
			},
		}),
		table.column({
			accessor: ({ ID }) => ID,
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

	export async function deleteVariables() {
		let indexes = Object.keys($selectedDataIds);
		let variablesForDeletion = indexes.map((index) => $variables[Number(index)].ID);
		const body = {
			variable_ids: variablesForDeletion,
		};

		try {
			let res = await APIClient.delete(
				`/project/${$page.params.project}/environment/${$page.params.environment}/app/${$page.params.app}/variables`,
				{ data: body },
			);
			toast.success(res.data.data.message);

			variables.set(res.data.data.variables);
		} catch (err) {
			console.log(err);
			toast.error(err.response.data.data.message);
		}
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
			<Button variant="destructive" on:click={deleteVariables}
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
										{#if cell.id === "Value"}
											<div class="relative">
												<Input
													type={row.showValue
														? "text"
														: "password"}
													value={cell.render()}
													class=" pr-10"
													disabled
												/><button
													type="button"
													on:click={() =>
														(row.showValue =
															!row.showValue)}
													class="absolute right-2
													top-1/2 transform
													-translate-y-1/2"
												>
													{#if row.showValue}
														<!-- Show the "eye-off" icon when password is visible -->
														<EyeOff
															class="size-4"
														/>
													{:else}
														<!-- Show the "eye" icon when password is hidden -->
														<Eye class="size-4" />
													{/if}
												</button>
											</div>
										{:else}
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
