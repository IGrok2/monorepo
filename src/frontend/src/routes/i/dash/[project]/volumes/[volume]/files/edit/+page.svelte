<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { enhance } from '$app/forms';

	import { toast } from 'svelte-sonner';
	import MonacoEditor from '$lib/components/files/MonacoEditor.svelte';
	import { getLanguageFromExtension } from '$lib/components/files/helpers';

	import Breadcrumbs from '$lib/components/files/Breadcrumbs.svelte';
	import { Button } from '$lib/components/ui/button';

    /** @type {import('./$types').PageData} */
	export let data;

	let content: string = data.content || '';
	let type: string = 'html';
	const fileName: string = 'document.html';
	let path = $page.url.searchParams.get('path');

	function getFileExtension(fileName: string): string {
		return fileName.split('.').pop() || '';
	}

	onMount(async () => {
		type = getLanguageFromExtension(getFileExtension(fileName));
	});
</script>

<div class="mb-2 flex justify-between items-center">
	<Breadcrumbs />
	<!-- Controles -->
	<form
		method="POST"
		action="?/save"
		use:enhance={() => {
			toast.success(`Attempting to save ${path}...`);
			return async ({ result }) => {
				toast.success(result.data.message);
				$page.url.searchParams.set('path', String(path));
			};
		}}
	>
		<Button type="submit">Save File</Button>
		<input name="content" type="string" bind:value={content} class="hidden" />
		<input name="path" type="string" bind:value={path} class="hidden" />
	</form>
</div>

<MonacoEditor bind:contents={content} bind:type />
