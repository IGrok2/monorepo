<script>
    import { page } from "$app/stores";
    
    import { Separator } from "$lib/components/ui/separator";

    import SideNav from "$lib/SideNav.svelte";

    import { ArrowLeft } from "lucide-svelte";

    /** @type {import('./$types').PageData} */
    export let data;

    const project = data.project;
    let app = data.app;

    export let items = [
        {
            title: "Overview",
            href: `/i/dash/${$page.params.project}/${$page.params.environment}/${$page.params.app}`,
        },
        {
            title: "Deployments",
            href: `/i/dash/${$page.params.project}/${$page.params.environment}/${$page.params.app}/deployments`,
        },
        /*{
            title: "Pods",
            href: `/i/dash/${$page.params.project}/${$page.params.environment}/${$page.params.app}/pods`,
        },*/
        {
            title: "Logs",
            href: `/i/dash/${$page.params.project}/${$page.params.environment}/${$page.params.app}/logs`,
        },
        /*{
            title: "Analytics",
            href: `/i/dash/${$page.params.project}/${$page.params.environment}/${$page.params.app}/analytics`,
        },*/
        {
            title: "Source",
            href: `/i/dash/${$page.params.project}/${$page.params.environment}/${$page.params.app}/source`,
        },
        {
            title: "Environment Variables",
            href: `/i/dash/${$page.params.project}/${$page.params.environment}/${$page.params.app}/variables`,
        },
        {
            title: "Networking",
            href: `/i/dash/${$page.params.project}/${$page.params.environment}/${$page.params.app}/networking`,
        },
        {
            title: "Compute",
            href: `/i/dash/${$page.params.project}/${$page.params.environment}/${$page.params.app}/compute`,
        },
        {
            title: "Storage",
            href: `/i/dash/${$page.params.project}/${$page.params.environment}/${$page.params.app}/storage`,
        },
    ];
</script>
<!-- 
<TopNav items={items} />

<slot />
-->

<div class="my-4 space-y-6 px-2 lg:px-10 pb-16 md:block">
	<div class="space-y-0.5">
		<h2 class="text-4xl font-bold">{app.Slug}</h2>
		<a href="/i/dash/{$page.params.project}" class="text-muted-foreground hover:underline flex items-center">
			<ArrowLeft class="w-4 h-4"/> {project.Slug}
        </a>
	</div>
	<div class="flex flex-col space-y-8 lg:flex-row lg:space-x-12 lg:space-y-0">
		<aside class="lg:w-1/6 overflow-auto">
			<SideNav items={items} />
		</aside>
		<div class="flex-1">
			<slot />
		</div>
	</div>
</div>