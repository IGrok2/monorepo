<script>
    import { page } from "$app/stores";
    import { fade } from "svelte/transition";

    import { Button } from "$lib/components/ui/button";

    import { X } from "lucide-svelte";

    import Logo from "$lib/assets/Asset 19.png";

    export let mobileSidebarOpen = false;

    export const links = [
        { name: "At a glance", link: `/i/dash/project/${$page.params.project}/domains/${$page.params.slug}` },
        {
            name: "Origins",
            link: `/i/dash/project/${$page.params.project}/domains/${$page.params.slug}/origins`,
        },
        {
            name: "Human Engine",
            link: `/i/dash/project/${$page.params.project}/domains/${$page.params.slug}/human_engine`,
        },
        {
            name: "Optimization",
            link: `/i/dash/project/${$page.params.project}/domains/${$page.params.slug}/optimization`,
        },
        {
            name: "Page Rules",
            link: `/i/dash/project/${$page.params.project}/domains/${$page.params.slug}/page_rules`,
        },
        {
            name: "Bot Management",
            link: `/i/dash/project/${$page.params.project}/domains/${$page.params.slug}/bot_management`,
        },
        {
            name: "Buckets",
            link: `/i/dash/project/${$page.params.project}/domains/${$page.params.slug}/buckets`,
        },

        /*{
            name: "Settings",
            link: `/i/dash/project/${$page.params.project}/domains/${$page.params.slug}/settings`,
        },*/
    ];
</script>

{#if mobileSidebarOpen}
    <!-- Off-canvas menu for mobile, show/hide based on off-canvas menu state. -->
    <div class="relative z-50 lg:hidden" role="dialog" aria-modal="true">
        <div class="fixed inset-0 bg-gray-900/80"></div>

        <div class="fixed inset-0 flex">
            <div class="relative mr-16 flex w-full max-w-xs flex-1">
                <div
                    class="absolute left-full top-0 flex w-16 justify-center pt-5"
                >
                    <Button
                        class="-m-2.5"
                        variant="ghost"
                        on:click={() => {
                            mobileSidebarOpen = !mobileSidebarOpen;
                        }}
                    >
                        <span class="sr-only">Close sidebar</span>
                        <X />
                    </Button>
                </div>

                <!-- Sidebar component, swap this element with another sidebar if you like -->
                <div
                    transition:fade={{ duration: 500 }}
                    class="flex grow flex-col gap-y-5 overflow-y-auto bg-background px-6 pb-2 ring-1 ring-white/10"
                >
                    <div class="flex h-16 shrink-0 items-center">
                        <img class="h-8 w-auto" src={Logo} alt="Your Company" />
                    </div>
                    <nav class="flex flex-1 flex-col">
                        <ul role="list" class="flex flex-1 flex-col gap-y-7">
                            <li>
                                <ul role="list" class="-mx-2 space-y-1">
                                    {#each links as link}
                                        <li>
                                            <!-- {#if link.link === $page.url.pathname} -->
                                            <Button
                                                href={link.link}
                                                variant="ghost"
                                                on:click={() => {
                                                    mobileSidebarOpen = false;
                                                }}
                                            >
                                                {link.name}
                                            </Button>
                                            <!-- {:else}
                                            {/if} -->
                                        </li>
                                    {/each}
                                </ul>
                            </li>
                        </ul>
                    </nav>
                </div>
            </div>
        </div>
    </div>
{/if}

<!-- Static sidebar for desktop -->
<div class="hidden py-10 lg:fixe lg:z-50 lg:flex lg:w-72 lg:flex-col">
    <!-- Sidebar component, swap this element with another sidebar if you like -->
    <div class="flex grow flex-col gap-y-5 overflow-y-auto px-6">
        <nav class="flex flex-1 flex-col">
            <ul role="list" class="flex flex-1 flex-col gap-y-7">
                <li>
                    <ul role="list" class="-mx-2 space-y-1">
                        {#each links as link}
                            <li>
                                <!-- {#if link.link === $page.url.pathname} -->
                                <Button href={link.link} variant="ghost">
                                    {link.name}
                                </Button>
                                <!-- {:else}

                                {/if} -->
                            </li>
                        {/each}
                    </ul>
                </li>
            </ul>
        </nav>
    </div>
</div>
