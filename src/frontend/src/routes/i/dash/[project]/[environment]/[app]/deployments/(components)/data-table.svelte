<script lang="ts">
	import { goto } from "$app/navigation";

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
	import {
		ArrowUpDown,
		ChevronDown,
		ArrowLeftFromLine,
		ArrowRightToLine,
		ChevronLeft,
		ChevronRight,
		Settings2,
	} from "lucide-svelte";
	import { Input } from "$lib/components/ui/input";
	import * as Select from "$lib/components/ui/select";
	import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
	import DataTableCheckbox from "./data-table-checkbox.svelte";

	import { page } from "$app/stores";
	/*type Payment = {
		id: string;
		amount: number;
		status: 'pending' | 'processing' | 'success' | 'failed';
		email: string;
	};*/

	export let data = [];

	const table = createTable(readable(data), {
		page: addPagination({}),
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
			header: "ID",
			plugins: {
				filter: {
					exclude: false,
				},
			},
		}),
		table.column({
			accessor: "Channel",
			header: "Channel",
			plugins: {
				filter: {
					exclude: false,
				},
			},
		}),
		/*table.column({
			accessor: "message",
			header: "Message",
			plugins: {
				filter: {
					exclude: false,
				},
			},
		}),*/
		table.column({
			accessor: "CreatedAt",
			header: "Created",
			plugins: {
				filter: {
					exclude: false,
				},
			},
		}),
		table.column({
			accessor: ({ ID }) => ID,
			header: "",
			cell: ({ value }) => {
				return createRender(DataTableActions, { id: value });
			},
			plugins: {
				sort: {
					disable: true,
				},
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
	const { pageIndex, hasNextPage, hasPreviousPage, pageCount, pageSize } =
		pluginStates.page;
	const { filterValue } = pluginStates.filter;
	const { hiddenColumnIds } = pluginStates.hide;
	const { selectedDataIds } = pluginStates.select;
	const ids = flatColumns.map((col) => col.id);
	let hideForId = Object.fromEntries(ids.map((id) => [id, true]));
	$: $hiddenColumnIds = Object.entries(hideForId)
		.filter(([, hide]) => !hide)
		.map(([id]) => id);
	const hidableCols = ["name"];
</script>

<div>
	<div class="flex justify-between py-4">
		<Input
			class="max-w-sm"
			placeholder="Filter deployments..."
			type="text"
			bind:value={$filterValue}
		/>
		<div class="ml-2">
			<DropdownMenu.Root>
				<DropdownMenu.Trigger asChild let:builder>
					<Button
						variant="outline"
						class="ml-auto"
						builders={[builder]}
					>
						Columns <Settings2 class="ml-2 h-4 w-4" />
					</Button>
				</DropdownMenu.Trigger>
				<DropdownMenu.Content>
					{#each flatColumns as col}
						{#if hidableCols.includes(col.id)}
							<DropdownMenu.CheckboxItem
								bind:checked={hideForId[col.id]}
							>
								{col.header}
							</DropdownMenu.CheckboxItem>
						{/if}
					{/each}
				</DropdownMenu.Content>
			</DropdownMenu.Root>
		</div>
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
										<div class="flex justify-center">
											<Render of={cell.render()} />
										</div>
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
										<div class="flex justify-center">
											{#if cell.id === "CreatedAt"}
												{new Date(
													cell.value,
												).toLocaleString()}
											{:else}<Render of={cell.render()} />
											{/if}
										</div>
									</Table.Cell>
								</Subscribe>
							{/each}
						</Table.Row>
					</Subscribe>
				{/each}
			</Table.Body>
		</Table.Root>
	</div>
	<!-- Pagination -->
	<div class="flex flex-wrap items-center justify-between px-2 py-4">
		<div class="flex-1 text-sm text-muted-foreground">
			{Object.keys($selectedDataIds).length} of{" "}
			{$rows.length} row(s) selected.
		</div>
		<div class="flex flex-wrap items-center space-x-6 lg:space-x-8">
			<div class="flex items-center space-x-2">
				<p class="text-sm font-medium">Rows per page</p>
				<Select.Root
					onSelectedChange={(selected) =>
						pageSize.set(Number(selected?.value))}
					selected={{ value: 10, label: "10" }}
				>
					<Select.Trigger class="w-[180px]">
						<Select.Value placeholder="Select page size" />
					</Select.Trigger>
					<Select.Content>
						<Select.Item value="10">10</Select.Item>
						<Select.Item value="25">25</Select.Item>
						<Select.Item value="50">50</Select.Item>
						<Select.Item value="100">100</Select.Item>
						<Select.Item value="500">500</Select.Item>
					</Select.Content>
				</Select.Root>
			</div>
			<div
				class="flex w-[100px] items-center justify-center text-sm font-medium"
			>
				Page {$pageIndex + 1} of {$pageCount}
			</div>
			<div class="flex items-center space-x-2">
				<Button
					variant="outline"
					class="hidden h-8 w-8 p-0 lg:flex"
					on:click={() => ($pageIndex = 0)}
					disabled={!$hasPreviousPage}
				>
					<span class="sr-only">Go to first page</span>
					<ArrowLeftFromLine size={15} />
				</Button>
				<Button
					variant="outline"
					class="p-0 w-8 h-8"
					on:click={() => ($pageIndex = $pageIndex - 1)}
					disabled={!$hasPreviousPage}
				>
					<span class="sr-only">Go to previous page</span>
					<ChevronLeft size={15} />
				</Button>
				<Button
					variant="outline"
					class="p-0 w-8 h-8"
					disabled={!$hasNextPage}
					on:click={() => ($pageIndex = $pageIndex + 1)}
				>
					<span class="sr-only">Go to next page</span>
					<ChevronRight size={15} />
				</Button>
				<Button
					variant="outline"
					class="hidden h-8 w-8 p-0 lg:flex"
					disabled={!$hasNextPage}
					on:click={() =>
						($pageIndex =
							Math.ceil($rows.length / $pageRows.length) - 1)}
				>
					<span class="sr-only">Go to last page</span>
					<ArrowRightToLine size={15} />
				</Button>
			</div>
		</div>
	</div>
</div>
