<script>
    import { page } from "$app/stores";

    import { Separator } from "$lib/components/ui/separator";
    import { Button } from "$lib/components/ui/button";

    import SideNav from "$lib/SideNav.svelte";

    const items = [
        {
            title: "General",
            href: `/i/dash/project/${$page.params.project}/apps/${$page.params.id}/settings/`,
        },
        {
            title: "Environment Variables",
            href: `/i/dash/project/${$page.params.project}/apps/${$page.params.id}/settings/variables`,
        },
        {
            title: "Secrets",
            href: `/i/dash/project/${$page.params.project}/apps/${$page.params.id}/settings/secrets`,
        },
        {
            title: "Networking",
            href: `/i/dash/project/${$page.params.project}/apps/${$page.params.id}/settings/networking`,
        },
        {
            title: "Compute",
            href: `/i/dash/project/${$page.params.project}/apps/${$page.params.id}/settings/compute`,
        },
        {
            title: "Storage",
            href: `/i/dash/project/${$page.params.project}/apps/${$page.params.id}/settings/storage`,
        },
    ];
</script>
<!--
<div class="space-y-6 p-10 pb-16 md:block">
	<div class="space-y-0.5">
		<h2 class="text-4xl font-bold">Settings</h2>
		<p class="text-muted-foreground">
			Fine tune app hosting details.
		</p>
	</div>
	<Separator class="my-6" />
	<div class="flex flex-col space-y-8 lg:flex-row lg:space-x-12 lg:space-y-0">
		<aside class="-mx-4 lg:w-1/5 overflow-auto">
			<SideNav items={items} />
		</aside>
		<div class="flex-1">
			<slot />
		</div>
	</div>
</div>
-->
<slot />
