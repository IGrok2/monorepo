<script>
    import { page } from "$app/stores";
    
    import { Separator } from "$lib/components/ui/separator";

    import TopNav from "$lib/components/TopNav.svelte";
    import SideNav from "$lib/SideNav.svelte";

    import { ArrowLeft } from "lucide-svelte";

    export let items = [
        {
            title: "Overview",
            href: `/i/dash/project/${$page.params.project}/volumes/${$page.params.id}`,
        },
        {
            title: "File Browser",
            href: `/i/dash/project/${$page.params.project}/volumes/${$page.params.id}/files`,
        },
    ];
</script>
<!-- 
<TopNav items={items} />

<slot />
-->

<div class="space-y-6 px-2 lg:px-10 pb-16 md:block">
	<div class="space-y-0.5">
		<h2 class="text-4xl font-bold">Volumes</h2>
		<a href="/i/dash/project/{$page.params.project}" class="text-muted-foreground hover:underline flex items-center">
			<ArrowLeft class="w-4 h-4"/> ${$page.params.project}
        </a>
	</div>
	<Separator class="my-6" />
	<div class="flex flex-col space-y-8 lg:flex-row lg:space-x-12 lg:space-y-0">
		<aside class="lg:w-1/6 overflow-auto">
			<SideNav items={items} />
		</aside>
		<div class="flex-1">
			<slot />
		</div>
	</div>
</div>