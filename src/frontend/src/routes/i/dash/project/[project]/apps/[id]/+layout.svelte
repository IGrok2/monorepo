<script>
    import { page } from "$app/stores";
    
    import { Separator } from "$lib/components/ui/separator";

    import TopNav from "$lib/components/TopNav.svelte";
    import SideNav from "$lib/SideNav.svelte";

    import { ArrowLeft } from "lucide-svelte";

    /** @type {import('./$types').PageData} */
    export let data;

    const project = data.project;
    let container = data.container;

    export let items = [
        {
            title: "Overview",
            href: `/i/dash/project/${$page.params.project}/apps/${$page.params.id}`,
        },
        {
            title: "Deployments",
            href: `/i/dash/project/${$page.params.project}/apps/${$page.params.id}/deployments`,
        },
        {
            title: "Pods",
            href: `/i/dash/project/${$page.params.project}/apps/${$page.params.id}/pods`,
        },
        /*{
            title: "Logs",
            href: `/i/dash/project/${$page.params.project}/apps/${$page.params.id}/logs`,
        },
        {
            title: "Analytics",
            href: `/i/dash/project/${$page.params.project}/apps/${$page.params.id}/analytics`,
        },
        {
            title: "Provider",
            href: `/i/dash/project/${$page.params.project}/apps/${$page.params.id}/settings`,
        },*/
        {
            title: "Environment Variables",
            href: `/i/dash/project/${$page.params.project}/apps/${$page.params.id}/settings/variables`,
        },
        {
            title: "Compute",
            href: `/i/dash/project/${$page.params.project}/apps/${$page.params.id}/settings/compute`,
        },
        {
            title: "Volumes",
            href: `/i/dash/project/${$page.params.project}/apps/${$page.params.id}/volumes`,
        },
    ];
</script>
<!-- 
<TopNav items={items} />

<slot />
-->

<div class="space-y-6 px-2 lg:px-10 pb-16 md:block">
	<div class="space-y-0.5">
		<h2 class="text-4xl font-bold">{container.name}</h2>
		<a href="/i/dash/project/{$page.params.project}" class="text-muted-foreground hover:underline flex items-center">
			<ArrowLeft class="w-4 h-4"/> {project.name}
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