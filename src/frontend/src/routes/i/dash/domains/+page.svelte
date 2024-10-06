<script>
    import DashNav from "$lib/DashNav.svelte";

    //import * as Dialog from "$lib/components/ui/dialog";
    import { Button } from "$lib/components/ui/button";
    import * as Card from "$lib/components/ui/card";
    import { Input } from "$lib/components/ui/input";
    import { Label } from "$lib/components/ui/label";
    import * as Avatar from "$lib/components/ui/avatar";
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
    import { MoreHorizontal, GitPullRequest } from "lucide-svelte";

    import { onMount } from "svelte";
    import { fade, slide } from "svelte/transition";
    import Popup from "$lib/Popup.svelte";
    import { Confetti } from "svelte-confetti";
    import Dialog from "$lib/components/Dialog.svelte";
    import { getCookie } from "$lib/utils/auth";

    import { PUBLIC_API } from "$env/static/public";
    import Notifications from "$lib/components/notifications/Notifications.svelte";
    import DocsLink from "$lib/base/DocsLink.svelte";
    import BasicLink from "$lib/base/BasicLink.svelte";

    let name;
    let domains;
    let loaded = false;

    let newMsg;
    let sub;

    let classes;
    let message;
    let submessage;

    let error;
    let error_submessage;

    let no_domains = false;
    let noDomainDialog;
    let new_domain;
    let new_domain_name;
    let newDomainDialog;

    onMount(async () => {
        let body;

        try {
            let token = getCookie("jwt");

            console.log(
                `at domains, the cookies in the browser are ${document.cookie}`,
            );

            await fetch(`${PUBLIC_API}/@/me`, {
                method: "GET",
                headers: new Headers({
                    "content-type": "application/json",
                    Authorization: token,
                }),
            }).then(async (rawBody) => {
                body = await rawBody.json();

                name = body.user.name;
                domains = body.user.domains;
                domains.sort((a, b) => b.glance.total - a.glance.total);

                if (domains.length === 0) {
                    no_domains = true;
                    noDomainDialog.showModal();
                }

                loaded = true;
            });
        } catch (err) {
            if (body && body.message === "Email not verified") {
                document.location.href = "/i/auth/verify";

                return;
            }

            document.location.href = "/i/dash/";
        }
    });

    const newDomain = async () => {
        try {
            let token = getCookie("jwt");

            console.log(new_domain_name);

            let body = {
                domain: new_domain_name,
            };

            await fetch(`${PUBLIC_API}/@/me/create-domain`, {
                method: "POST",
                headers: new Headers({
                    "content-type": "application/json",
                    Authorization: token,
                }),
                body: JSON.stringify(body),
            }).then(async (rawBody) => {
                let body = await rawBody.json();

                if (body.success && rawBody.status === 200) {
                    await notification(
                        "Domain created",
                        "Your new domain is ready to go! It may take a few minutes for us to gain approval for an secure socket certificate.",
                    );

                    document.location.href = `/i/dash/domains/${new_domain_name}`;
                } else {
                    if (body.message !== undefined) {
                        await error_notification("Oops", body.message);
                    } else {
                        await error_notification(
                            "Error",
                            "Something went wrong. Please try again.",
                        );
                    }
                }
            });
        } catch (err) {
            await notification(
                "Error",
                "Something went wrong. Please try again.",
            );
        }
    };

    export const notification = async (newMsg, sub) => {
        classes =
            "transform ease-out duration-300 transition translate-y-0 opacity-100 sm:translate-x-0";

        message = newMsg;
        submessage = sub;

        await new Promise((resolve) => setTimeout(resolve, 3000));

        classes = "transition ease-in duration-100 opacity-0";

        message = undefined;
    };

    export const error_notification = async (newMsg, sub) => {
        classes =
            "transform ease-out duration-300 transition translate-y-0 opacity-100 sm:translate-x-0";

        error = newMsg;
        error_submessage = sub;

        await new Promise((resolve) => setTimeout(resolve, 7000));

        classes = "transition ease-in duration-100 opacity-0";

        error = undefined;
    };
</script>

<Notifications
    bind:message
    bind:classes
    bind:submessage
    bind:error
    bind:error_submessage
/>

<!--
{#if no_domains}
    <div
        style="
 position: fixed;
 top: -50px;
 left: 0;
 height: 100vh;
 width: 100vw;
 display: flex;
 justify-content: center;
 overflow: hidden;
 pointer-events: none;"
    >
        <Confetti
            x={[-5, 5]}
            y={[0, 0.1]}
            delay={[500, 2000]}
            infinite
            amount="200"
            fallDistance="100vh"
            colorArray={[
                "url(http://welkinvault.com/favicon.svg)",
                "url(http://welkinvault.com/baby_yoda.svg)",
            ]}
        />
    </div>
    <div in:slide class="z-50 absolute">
    <Popup bg_color="bg-transparent backdrop-blur-lg" no_repeat={false}>
        <div class="w-full flex justify-center">
            <Confetti x={[-1.5, 1.5]} y={[-0, -5]} amount=500 cone delay={[0,750]} />
        </div>
        <div in:fade class="flex flex-1 w-full">
            <div class="pl-3">
                <p class="text-5xl text-primary font-ultrawide">It's great to have you on, {name}.</p>
                <p class="text-primary text-2xl md:indent-10 first-line:tracking-widest
      first-letter:text-4xl first-letter:font-bold first-line:uppercase">
                    Since <a class="font-ultrawide">Packetware</a>'s inception, we've wanted people to share their ambitions without fear of cyberattack.
                    We've used every CDN, but always found they didn't offer excellent solutions.
                    We reached out to them, offering our insights and feedback:
                    they didn't listen.
                    <br>
                    So Packetware began.
                    <br><br>
                </p>
                <p class="text-primary text-2xl md:indent-10 first-line:tracking-widest first-line:uppercase">
                    <a class="font-ultrawide">Packetware</a> aims to be more than a product; but a creation shaped by the community it serves. Your voice is a paintbrush, adding color to our canvas.
                    We are a product that is incrementally improving, hoping to ultimately make change to the landscape of CDN.
                    We ask that by using our beta product, you activate your inner critic, sharing your insights: no matter how small or large.
                </p>
                <p class="pt-10 text-primary text-2xl text-center">Let's get started by adding your first domain.</p>
                <div class="w-full flex justify-center pt-3">
                    <input bind:value={new_domain_name} id="email" name="code" type="text" placeholder="ENTER DOMAIN NAME..." required class="inline-flex w-full text-center rounded-md border-0 py-1.5 text-gray-200 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-muted-foreground focus:ring-2 sm:max-w-sm focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6 bg-gray-500">
                </div>
                <div class="w-full flex justify-center pt-6">
                    <a on:click={newDomain} class="hover:cursor-[url(/svelte.config.cur),_copy] inline-flex w-full justify-center rounded-md bg-gradient-to-br from-blue-500 via-amber-500 to-fuchsia-500 px-3 py-2 text-sm font-semibold text-white shadow-sm sm:w-auto font-krona">Blast off</a>
                </div>

                <div class="mt-2 w-full flex justify-center">
                    <p class="text-slate-300 pt-6">
                        Packetware may be different from what you're used to due to our secure-by-default philosophy. Some modifications may be necessary for your site to function excellently.<br><br>

                        When a new domain is formed, our infrastructure presence around the world is quickly updated and available for your site. It will probably take a few moments for us to fetch approval for a secure socket certificate. Please create a CNAME, alias, or A record to packetware.net (we have some guides <a href="/docs/setup" class="text-indigo-500 hover:text-indigo-400">here</a>).<br><br>
                        Our <a href="/support" class="text-indigo-500 hover:text-indigo-400">support team</a> stands by, ready to lend a hand if you have any questions. It is staffed by the same people who built Packetware.
                        The <a href="/docs" class="text-indigo-500 hover:text-indigo-400">documentation</a> is a great place to learn about our products, our philosophy, and how we're able to provide the next level of security, visibility, and performance to our clients.
                    </p>
            </div>
        </Popup>
    </div>
{/if}
-->

<Dialog bind:dialog={noDomainDialog} title="">
    <div class="max-w-7xl w-full flex justify-center">
        <Confetti
            x={[-1.5, 1.5]}
            y={[-0, -5]}
            amount="500"
            cone
            delay={[0, 1750]}
        />
    </div>
    <div in:fade class="flex flex-1 w-full">
        <div class="pl-3">
            <p class="text-3xl text-primary font-ultrawide">
                It's great to have you on, {name}.
            </p>
            <p
                class="text-primary text-xl md:indent-10 first-line:tracking-widest
  first-letter:text-4xl first-letter:font-bold first-line:uppercase"
            >
                Since <a class="font-ultrawide">Packetware</a>'s inception,
                we've wanted people to share their ambitions without fear of
                cyberattack or scale. We've used every CDN and serverless
                platform, but always found they didn't offer excellent
                solutions. We reached out to them, offering our insights and
                feedback: they didn't listen.
                <br />
                So Packetware began.
                <br /><br />
            </p>
            <p
                class="text-primary text-xl md:indent-10 first-line:tracking-widest first-line:uppercase"
            >
                <a class="font-ultrawide">Packetware</a> aims to give your users
                excellent experiences through incredible cybersecurity, top-notch
                performance, and engineering based off better first principles. We're
                excited to have you join our user base that loves our products..
                Your obligation to us is that you activate your inner critic, sharing
                your insights of dislike and like.
            </p>
            <p class="pt-10 text-primary text-xl text-center">
                Welcome to a new way to do Internet infrastructure. Let's get
                started by adding your first domain.
            </p>
            <div class="w-full flex justify-center pt-3">
                <input
                    bind:value={new_domain_name}
                    id="email"
                    name="code"
                    type="text"
                    placeholder="ENTER DOMAIN NAME..."
                    required
                    class="inline-flex w-full text-center rounded-md border-0 py-1.5 text-gray-200 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-muted-foreground focus:ring-2 sm:max-w-sm focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6 bg-gray-500"
                />
            </div>
            <div class="w-full flex justify-center pt-6">
                <a
                    on:click={newDomain}
                    class="hover:cursor-[url(/svelte.config.cur),_copy] inline-flex w-full justify-center rounded-md bg-gradient-to-br from-blue-500 via-amber-500 to-fuchsia-500 px-3 py-2 text-sm font-semibold text-white shadow-sm sm:w-auto font-krona"
                    >Blast off</a
                >
            </div>

            <div class="mt-2 w-full flex justify-center">
                <p class="text-slate-300 pt-6">
                    Packetware may be different from what you're used to due to
                    our secure-by-default philosophy. Some modifications may be
                    necessary for your site to function excellently.<br /><br />

                    When a new domain is formed, our infrastructure presence
                    around the world is quickly updated and available for your
                    site. It will probably take a few moments for us to fetch
                    approval for a secure socket certificate. Please create a
                    CNAME, alias, or A record to packetware.net (we have some
                    guides
                    <a
                        href="/docs/setup"
                        class="text-indigo-500 hover:text-indigo-400">here</a
                    >).<br /><br />
                    Our
                    <a
                        href="/support"
                        class="text-indigo-500 hover:text-indigo-400"
                        >support team</a
                    >
                    stands by, ready to lend a hand if you have any questions. It
                    is staffed by the same people who built Packetware. The
                    <a
                        href="/docs"
                        class="text-indigo-500 hover:text-indigo-400"
                        >documentation</a
                    > is a great place to learn about our products, our philosophy,
                    and how we're able to provide the next level of security, visibility,
                    and performance to our clients.
                </p>
            </div>
        </div>
    </div></Dialog
>

{#if loaded}
    <!-- {#if new_domain} -->
    <Dialog bind:dialog={newDomainDialog} title="Add New Domain">
        <form class="relative mt-6 flex-1" on:submit|preventDefault={newDomain}>
            <p>
                <Label for="domain">Domain name</Label>
                <Input
                    bind:value={new_domain_name}
                    type="text"
                    name="domain"
                    id="domain"
                    placeholder="diamondcdn.com"
                />
            </p>
            <p class="pt-3 text-sm text-muted-foreground">
                Welcome to the Packetware rodeo! Our platform is designed to be
                comprehensive and secure-by-default, so it may be different from
                what you're used to. Some modifications may be necessary for
                your site to function excellently.<br /><br />

                When a new domain is formed, our infrastructure presence around
                the world is instantly updated and available for your site. It
                will probably take a few moments for us to fetch approval for a
                secure socket certificate. Please create a CNAME, alias, or A
                record to packetware.net (we have some guides
                <a
                    href="/docs/setup"
                    class="text-indigo-500 hover:text-indigo-400">here</a
                >).<br /><br />

                We're excited you're ready to take this journey on with us. Our
                <a
                    href="/docs/overview/point-u-to-packetware"
                    class="text-indigo-500 hover:text-indigo-400"
                    >support team</a
                >
                stands by, ready to lend a hand if you have any questions. It is
                staffed by the same people who built Packetware. The
                <a href="/docs" class="text-indigo-500 hover:text-indigo-400"
                    >documentation</a
                >
                is a great place to learn about our products, our philosophy, and
                how we're able to provide the next level of security, visibility,
                and performance to our clients.
                <br /><br />
            </p>
            <div class="flex justify-center pt-8">
                <button
                    type="submit"
                    class="mt-3 inline-flex w-full items-center justify-center rounded-md bg-red-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 sm:ml-3 sm:mt-0 sm:w-auto"
                    >Add Domain</button
                >
            </div>
        </form>
    </Dialog>
    <!--
        <div
            in:slide
            class="relative z-50"
            aria-labelledby="slide-over-title"
            role="dialog"
            aria-modal="true"
        >
            <div
                class="fixed inset-0 bg-gray-700 bg-[url('/baby-yoda.jpeg')] bg-opacity-75 transition-opacity bg-no-repeat bg-center"
            />
            <div class="fixed inset-0 overflow-hidden">
                <div class="absolute inset-0 overflow-hidden">
                    <div
                        class="pointer-events-none fixed inset-y-0 right-0 flex max-w-full pl-10"
                    >
                        <div class="pointer-events-auto w-screen max-w-md">
                            <div
                                class="flex h-full flex-col overflow-y-scroll bg-background py-6 shadow-xl"
                            >
                                <div class="px-4 sm:px-6">
                                    <div
                                        class="flex items-start justify-between"
                                    >
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            fill="none"
                                            viewBox="0 0 24 24"
                                            stroke-width="1.5"
                                            stroke="currentColor"
                                            class="w-6 h-6"
                                        >
                                            <path
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                d="M12 21a9.004 9.004 0 008.716-6.747M12 21a9.004 9.004 0 01-8.716-6.747M12 21c2.485 0 4.5-4.03 4.5-9S14.485 3 12 3m0 18c-2.485 0-4.5-4.03-4.5-9S9.515 3 12 3m0 0a8.997 8.997 0 017.843 4.582M12 3a8.997 8.997 0 00-7.843 4.582m15.686 0A11.953 11.953 0 0112 10.5c-2.998 0-5.74-1.1-7.843-2.918m15.686 0A8.959 8.959 0 0121 12c0 .778-.099 1.533-.284 2.253m0 0A17.919 17.919 0 0112 16.5c-3.162 0-6.133-.815-8.716-2.247m0 0A9.015 9.015 0 013 12c0-1.605.42-3.113 1.157-4.418"
                                            />
                                        </svg>
                                        <h2
                                            class="font-semibold leading-6 text-gray-900 text-xl"
                                            id="slide-over-title"
                                        >
                                            New domain
                                        </h2>
                                        <div class="ml-3 flex h-7 items-center">
                                            <button
                                                on:click={() => {
                                                    new_domain = false;
                                                }}
                                                type="button"
                                                class="rounded-md bg-background text-muted-foreground hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
                                            >
                                                <span class="sr-only"
                                                    >Close panel</span
                                                >
                                                <svg
                                                    class="h-6 w-6"
                                                    fill="none"
                                                    viewBox="0 0 24 24"
                                                    stroke-width="1.5"
                                                    stroke="currentColor"
                                                    aria-hidden="true"
                                                >
                                                    <path
                                                        stroke-linecap="round"
                                                        stroke-linejoin="round"
                                                        d="M6 18L18 6M6 6l12 12"
                                                    />
                                                </svg>
                                            </button>
                                        </div>
                                    </div>
                                </div>
                                <form
                                    class="relative mt-6 flex-1 px-4 sm:px-6"
                                    on:submit|preventDefault={newDomain}
                                >
                                    <p>
                                        <label
                                            for="domain"
                                            class="block font-medium text-gray-800"
                                            >Domain name</label
                                        >
                                        <input
                                            bind:value={new_domain_name}
                                            type="text"
                                            name="domain"
                                            id="domain"
                                            class="mt-1 focus:ring-indigo-500 focus:border-indigo-500 block w-full shadow-sm sm:text-sm border-gray-300 rounded-md"
                                            placeholder="diamondcdn.com"
                                        />
                                    </p>
                                    <p class="pt-3 text-sm text-gray-600">
                                        Welcome to the Packetware rodeo! Our
                                        platform is designed to be comprehensive
                                        and secure-by-default, so it may be
                                        different from what you're used to. Some
                                        modifications may be necessary for your
                                        site to function excellently.<br /><br
                                        />

                                        When a new domain is formed, our
                                        infrastructure presence around the world
                                        is instantly updated and available for
                                        your site. It will probably take a few
                                        moments for us to fetch approval for a
                                        secure socket certificate. Please create
                                        a CNAME, alias, or A record to
                                        packetware.net (we have some guides
                                        <a
                                            href="/docs/setup"
                                            class="text-indigo-500 hover:text-indigo-400"
                                            >here</a
                                        >).<br /><br />

                                        We're excited you're ready to take this
                                        journey on with us. Our
                                        <a
                                            href="/support"
                                            class="text-indigo-500 hover:text-indigo-400"
                                            >support team</a
                                        >
                                        stands by, ready to lend a hand if you have
                                        any questions. It is staffed by the same
                                        people who built Packetware. The
                                        <a
                                            href="/docs"
                                            class="text-indigo-500 hover:text-indigo-400"
                                            >documentation</a
                                        >
                                        is a great place to learn about our products,
                                        our philosophy, and how we're able to provide
                                        the next level of security, visibility, and
                                        performance to our clients.
                                        <br /><br />
                                    </p>
                                    <div class="flex justify-center pt-8">
                                        <button
                                            type="submit"
                                            class="mt-3 inline-flex w-full items-center justify-center rounded-md bg-red-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 sm:ml-3 sm:mt-0 sm:w-auto"
                                            >Add Domain</button
                                        >
                                    </div>
                                </form> 
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    {/if}
    -->

    <div
        in:fade
        class="mx-5 md:mx-20 my-10 p-5 md:p-20 backdrop-blur-lg rounded shadow-fuchsia-900 shadow-xl"
    >
        <p class="text-4xl text-primary text-right font-krona">
            👋 Welcome, {name}
        </p>

        <div class="pt-5">
            <p class="uppercase text-primary">Metrics - Past 72 Hours</p>
            <div class="text-center px-5 md:px-10 text-primary text-xl">
                <p>
                    Welcome to Packetware beta! We're excited to have you on
                    board. We're now on the second sprint, including Human
                    Engine (our cybersecurity suite for frontend applications,
                    now further improved), Origin Settings (connecting
                    Packetware to your backend), analytics (visibility for your
                    site), and caching (saving temporary copies of your content
                    around the world for ultra-fast delivery).
                    <br />
                    <br />
                    Our caching uses a blend of disk and memory to be extremely performant
                    and reliable. No more running out of cache.
                </p>
                <p class="pt-5">We recommend checking out the <DocsLink />.</p>
                <p>
                    If you have any comments, questions, or concerns, you can
                    reach out to <BasicLink
                        text="edward@packetware.net"
                        href="mailto:edward@packetware.net"
                    /> or
                    <BasicLink
                        text="join the Discord"
                        href="https://discord.gg/mPytm76nG2"
                    />.
                </p>
                <p></p>
            </div>
        </div>
        <!-- OVERALL ANALYTICS
<div class="pt-5">
<p class="uppercase text-slate-300">At a glance • past 72 hours</p>
<dl class="mt-5 grid grid-cols-1 gap-5 sm:grid-cols-3">
    <div
        class="overflow-hidden rounded-lg bg-background px-4 py-5 shadow sm:p-6"
    >
        <dt class="truncate text-sm font-medium text-gray-500">
            Total requests
        </dt>
        <dd
            class="mt-1 text-3xl font-semibold tracking-tight text-gray-900"
        >
            672,293
        </dd>
    </div>
    <div
        class="overflow-hidden rounded-lg bg-background px-4 py-5 shadow sm:p-6"
    >
        <dt class="truncate text-sm font-medium text-gray-500">
            Security events
        </dt>
        <dd
            class="mt-1 text-3xl font-semibold tracking-tight text-gray-900"
        >
            239,596
        </dd>
    </div>
    <div
        class="overflow-hidden rounded-lg bg-background px-4 py-5 shadow sm:p-6"
    >
        <dt class="truncate text-sm font-medium text-gray-500">
            Cache rate
        </dt>
        <dd
            class="mt-1 text-3xl font-semibold tracking-tight text-gray-900"
        >
            96.57%
        </dd>
    </div>
</dl>
</div>
-->

        <div class="pt-5">
            <div class="flex items-center w-full py-5 justify-between">
                <p class="uppercase text-slate-300 justify-start">
                    Your domains
                </p>
                <Button
                    class="justify-end"
                    on:click={() => {
                        new_domain = true;
                        newDomainDialog.showModal();
                    }}>Add Domain</Button
                >
            </div>
            <ul
                role="list"
                class="flex flex-wrap gap-x-6 gap-y-8 justify-center"
            >
                {#if domains}
                    {#each domains as domain}
                        <Card.Root class="w-[350px]">
                            <Card.Header>
                                <div class="flex justify-between">
                                    <div class="flex">
                                        <Avatar.Root>
                                            <Avatar.Image
                                                src="https://{domain.domain}/favicon.ico"
                                            />
                                            <Avatar.Fallback
                                                >{domain.domain.charAt(
                                                    0,
                                                )}{domain.domain.charAt(
                                                    domain.domain.split(".")[0]
                                                        .length - 1,
                                                )}</Avatar.Fallback
                                            >
                                        </Avatar.Root>
                                        <div class="items-center">
                                            <Card.Title
                                                ><Button
                                                    href="/i/dash/domains/{domain.domain}"
                                                    variant="link"
                                                    >{domain.domain}</Button
                                                ></Card.Title
                                            >

                                            <Card.Description class="w-full"
                                                >{domain.glance.total} requests,
                                                {domain.glance.security} security
                                                events,
                                                {domain.glance.errors} errors
                                            </Card.Description>
                                        </div>
                                    </div>
                                    <DropdownMenu.Root>
                                        <DropdownMenu.Trigger
                                            asChild
                                            let:builder
                                        >
                                            <Button
                                                variant="ghost"
                                                builders={[builder]}
                                                ><MoreHorizontal /></Button
                                            >
                                        </DropdownMenu.Trigger>
                                        <DropdownMenu.Content>
                                            <DropdownMenu.Group>
                                                <DropdownMenu.Item
                                                    href="/i/dash/domains/{domain}/settings"
                                                    >Settings</DropdownMenu.Item
                                                >
                                                <DropdownMenu.Item
                                                    href="/i/dash/domains/{domain}"
                                                    >Analytics</DropdownMenu.Item
                                                >
                                            </DropdownMenu.Group>
                                        </DropdownMenu.Content>
                                    </DropdownMenu.Root>
                                </div>
                            </Card.Header>
                        </Card.Root>
                    {/each}
                {/if}
            </ul>
        </div>
    </div>
{:else}
    <div class="h-screen" />
{/if}
