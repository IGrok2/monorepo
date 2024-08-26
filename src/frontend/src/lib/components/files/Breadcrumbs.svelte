<script>
	import { page } from '$app/stores';
	import * as Breadcrumb from '$lib/components/ui/breadcrumb';

	let sections = [];

	$: console.log($page.url.searchParams.get('path'));

	function splitUrl(url) {
		const parts = url.split('/');
		sections = parts.slice(1).map((name, index) => ({
			path: '/' + parts.slice(1, index + 2).join('/'),
			name
		}));
		return sections;
	}

	$: {
		if ($page.url.searchParams.get('path')) {
			const url = $page.url.searchParams.get('path');
			const sections = splitUrl(url);
			console.log('Sections:', sections);
		}
	}
</script>

<Breadcrumb.Root>
	<Breadcrumb.List>
		<Breadcrumb.Item>
			<Breadcrumb.Link>mnt</Breadcrumb.Link>
		</Breadcrumb.Item>
		<Breadcrumb.Separator />
		<Breadcrumb.Item>
			<Breadcrumb.Link href="/i/dash/project/{$page.params.project}/volumes/{$page.params.id}/files"
				>{$page.params.id}</Breadcrumb.Link
			>
		</Breadcrumb.Item>
		{#each sections as section, index}
			{#if index !== sections.length - 1}
				<Breadcrumb.Separator />
				<Breadcrumb.Item>
					<Breadcrumb.Link href="/i/dash/project/{$page.params.project}/volumes/{$page.params.id}/files?path={section.path}"
						>{section.name}</Breadcrumb.Link
					>
				</Breadcrumb.Item>
			{/if}
		{/each}
		{#if $page.url.searchParams.get('path')}
			<Breadcrumb.Separator />
			<Breadcrumb.Item>
				<Breadcrumb.Page>{sections[sections.length - 1].name}</Breadcrumb.Page>
			</Breadcrumb.Item>
		{/if}
	</Breadcrumb.List>
</Breadcrumb.Root>

<slot />
