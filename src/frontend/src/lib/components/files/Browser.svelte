<script>
	import { page } from '$app/stores';
	import Breadcrumbs from './Breadcrumbs.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Checkbox } from '$lib/components/ui/checkbox';

	import { File, Folder, Download, MoveHorizontal } from 'lucide-svelte';

	export let files;

	function navigate(file) {
		let currentPath = $page.url.searchParams.get('path');
		if (!currentPath) {
			currentPath = '';
		}
		let newPath = currentPath + '/' + file.name;
		$page.url.searchParams.set('path', newPath);

		if (file.type === 'folder' || file.type === 'directory') {
			document.location = $page.url.href;
		} else {
			document.location = $page.url.pathname + '/edit' + $page.url.search;
		}
	}
</script>

<div class="mb-2 flex justify-between items-center">
	<div class="flex items-center space-x-4">
		<Checkbox />
		<Breadcrumbs />
	</div>
	<!-- Controles -->
	<div class="flex space-x-2">
		<Button variant="outline">Create Directory</Button>
		<Button>Upload</Button>
		<Button>New File</Button>
	</div>
</div>

<div
	class="my-2 grid grid-cols-4 items-center rounded-md bg-muted px-4 py-3 text-sm font-medium text-muted-foreground"
>
	<span class="col-span-1 sr-only">Type</span>
	<span class="col-span-1">Name</span>
	<span class="col-span-1">Size</span>
	<span class="col-span-1">Modified</span>
	<span class="col-span-1 text-right">Actions</span>
</div>

{#each files as file}
	<a
		on:click={() => navigate(file)}
		class="mb-0.5 grid grid-cols-4 items-center rounded-md bg-background border px-4 py-1 text-sm transition-colors gap-4 hover:bg-muted"
	>
		<div class="flex items-center space-x-1">
			<Checkbox class="mr-4"/>

			<!-- Icon -->
			{#if file.type === 'folder' || file.type === 'directory'}
				<Folder class="h-5 w-5" />
			{:else}
				<File class="h-5 w-5" />
			{/if}

			<!-- Name -->
			<a class="col-span-1 font-medium transition-colors hover:text-primary">
				{file.name}
			</a>
		</div>

		<!-- Size -->
		<span class="col-span-1">
			{file.size}
		</span>

		<!-- Modified -->
		<span class="col-span-1">
			{file.modified}
		</span>

		<!-- Actions -->
		<div class="col-span-1 flex items-center justify-end gap-2">
			<Button variant="ghost">
				<Download class="h-4 w-4" />
				<span class="sr-only">Download</span>
			</Button>
			<Button variant="ghost">
				<MoveHorizontal class="h-4 w-4" />
				<span class="sr-only">More</span>
			</Button>
		</div>
	</a>
{/each}
