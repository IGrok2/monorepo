<script>
    import { page } from "$app/stores";
    import SideNav from "$lib/SideNav.svelte";
    import { Separator } from "$lib/components/ui/separator";
    import { toast } from "svelte-sonner";
    import { getCookie } from "$lib/utils/auth";

    import { ArrowLeft } from "lucide-svelte";

    import { PUBLIC_API } from "$env/static/public";

    /** @type {import('./$types').LayoutData} */
    export let data;

    export let items = [
        {
            title: "At a glance",
            href: `/i/dash/project/${$page.params.project}/domains/${$page.params.slug}`,
        },
        {
            title: "Origins",
            href: `/i/dash/project/${$page.params.project}/domains/${$page.params.slug}/origins`,
        },
        {
            title: "Human Engine",
            href: `/i/dash/project/${$page.params.project}/domains/${$page.params.slug}/human_engine`,
        },
        {
            title: "Optimization",
            href: `/i/dash/project/${$page.params.project}/domains/${$page.params.slug}/optimization`,
        },
        {
            title: "Page Rules",
            href: `/i/dash/project/${$page.params.project}/domains/${$page.params.slug}/page_rules`,
        },
        {
            title: "Bot Management",
            href: `/i/dash/project/${$page.params.project}/domains/${$page.params.slug}/bot_management`,
        },
        {
            title: "Buckets",
            href: `/i/dash/project/${$page.params.project}/domains/${$page.params.slug}/buckets`,
        },
    ];

    let no_origins = data.resp.no_origins;

    let verification_required = data.verification.required;
    let verification_key = data.verification.key;
    let verification_date = data.verification.date;

    let project = data.resp.project;

    async function verifyDomain() {
        toast.success(`Started verification for ${$page.params.slug}`);
        await fetch(
            `${PUBLIC_API}/@/project/${$page.params.project}/domain/${$page.params.slug}/verify`,
            {
                method: "POST",
                headers: new Headers({
                    "content-type": "application/json",
                    Authorization: getCookie("jwt"),
                }),
                body: JSON.stringify({
                    domain: $page.params.slug,
                }),
            },
        )
            .then(async (res) => {
                let response = await res.json();

                if (res.status !== 200) {
                    toast.error(`${response.message}`);
                } else {
                    toast.success("Domain verified, Welcome!");
                }

                document.location.reload();
            })
            .catch(async (err) => {
                toast.error(
                    `Failed to verify domain, Try refreshing the page? ${err}`,
                );
            });
    }
</script>

<div class="space-y-6 px-2 lg:px-10 pb-16 md:block">
    <div class="space-y-0.5">
        <h2 class="text-4xl font-bold">{$page.params.slug}</h2>
        <a
            href="/i/dash/project/{$page.params.project}"
            class="text-muted-foreground hover:underline flex items-center"
        >
            <ArrowLeft class="w-4 h-4" />
            {project.name}
        </a>
    </div>
    <Separator class="my-6" />
    <div class="flex flex-col space-y-8 lg:flex-row lg:space-x-12 lg:space-y-0">
        <aside class="lg:w-1/6 overflow-auto">
            <SideNav {items} />
        </aside>
        <div class="flex-1">
            {#if verification_required}
                <!-- they need to add a text record to begin using their domain -->
                <div class="flex h-full w-full pt-3">
                    <div class="w-full p-5">
                        <p class="uppercase text-xl text-muted-foreground">
                            verification required
                        </p>
                        <p class="text-4xl pt-3 text-foreground">
                            To begin using {$page.params.slug}, you'll have to
                            create a TXT record<br />
                            <span class="text-muted-foreground text-xl"
                                >This is to verify that you own the domain. If
                                you're not sure where to go, create this record
                                at your domain registrar.</span
                            >
                        </p>
                        <div class="p-4 my-10 border-2 border-dotted">
                            <p class="uppercase text-muted-foreground pb-4">
                                TXT record to place at root domain ({$page
                                    .params.slug})
                            </p>
                            <div
                                class="flex justify-center flex-1 text-muted-foreground text-xl"
                            >
                                <div class="">
                                    <p class="text-sm uppercase text-center">
                                        TXT record value
                                    </p>
                                    <p class="select-all text-foreground">
                                        packetware-verification={verification_key}
                                    </p>
                                </div>
                            </div>
                        </div>
                        <p class="text-muted-foreground">
                            Once you've added these records, have us <button
                                on:click={async () => {
                                    verifyDomain();
                                }}
                                class="text-fuchsia-500">check them</button
                            >
                        </p>
                        <p class="text-muted-foreground pt-3">
                            Last checked: {new Date(
                                verification_date,
                            ).toLocaleDateString("en-us", {
                                weekday: "long",
                                month: "short",
                                day: "numeric",
                                hour: "numeric",
                                minute: "numeric",
                            })}
                        </p>
                    </div>
                </div>
            {:else}
                <slot />
            {/if}
        </div>
    </div>
</div>
