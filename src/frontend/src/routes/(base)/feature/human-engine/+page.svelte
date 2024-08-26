<script>
    import Navbar from "$lib/Navbar.svelte";
    import Rizz from "$lib/Rizz.svelte";
    import BasicLink from "$lib/base/BasicLink.svelte";
    import DocsLink from "$lib/base/DocsLink.svelte";
    import Footer from "$lib/Footer.svelte";
</script>

<section class="bg-gradient-to-tr from-zinc-900 to-90% to-fuchsia-900 relative text-slate-300 font-mona">
    <div class="overflow-y-auto md:relative h-screen md:snap-y md:snap-mandatory pb-14">
        <Navbar />
        <svg
                viewBox="0 0 1024 1024"
                class="absolute left-0 top-0 z-50 -translate-x-1/2 [mask-image:radial-gradient(closest-side,white,transparent)]"
                aria-hidden="true"
        >
            <circle
                    cx="512"
                    cy="512"
                    r="512"
                    fill="url(#8d958450-c69f-4251-94bc-4e091a323369)"
                    fill-opacity="0.3"
            />
            <defs>
                <radialGradient id="8d958450-c69f-4251-94bc-4e091a323369">
                    <stop stop-color="#7775D6" />
                    <stop offset="1" stop-color="#E935C1" />
                </radialGradient>
            </defs>
        </svg>
        <section class="md:snap-start md:h-screen md:w-screen">
            <div class="md:h-screen md:w-screen p-5 md:p-20">
                <!--
                <div class="relative">
                    <svg
                            viewBox="0 0 1024 1024"
                            class="absolute -top-1/3 -translate-x-1/2 z-20 h-[64rem] w-[64rem] [mask-image:radial-gradient(closest-side,white,transparent)]"
                            aria-hidden="true"
                    >
                        <circle
                                cx="512"
                                cy="512"
                                r="512"
                                fill="url(#8d958450-c69f-4251-94bc-4e091a323369)"
                                fill-opacity="0.2"
                        />
                        <defs>
                            <radialGradient id="8d958450-c69f-4251-94bc-4e091a323369">
                                <stop stop-color="#7775D6" />
                                <stop offset="1" stop-color="#E935C1" />
                            </radialGradient>
                        </defs>
                    </svg>
                </div>
                -->

                <div class="md:flex h-full items-center">
                    <div class="md:pr-20">
                        <p class="md:text-6xl text-5xl pb-3">Human Engine. Highly effective DDoS and bot mitigation.</p>
                        <p class="text-2xl pt-5 md:text-3xl">
                            Stop bad traffic before it reaches your servers, without your users ever noticing a thing: no CAPTCHAs.
                        </p>
                        <div class="flex my-6">
                            <button class="bg-fuchsia-500 p-4 py-1.5 text-slate-50">
                                Get started
                            </button>
                            <div class="px-3 py-1.5">
                                <BasicLink text="Documentation" href="/docs" />
                            </div>
                        </div>
                    </div>
                    <div>

                        <!--
                        <ul class="list-disc text-xl">
                            <p class="uppercase text-2xl">Highlights</p>
                            <li>Human-Validated Endpoints</li>
                            <li>Web Application Firewall</li>
                            <li>Live WebSocket Security</li>
                            <li>What Should Be Allowed - Based on First Principles</li>
                            <li>Detailed Analytics</li>
                        </ul>
                        -->
                    </div>
                </div>

                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="absolute inset-y-full inset-x-1/2 -translate-x-1/2 w-12 h-12">
                    <path stroke-linecap="round" stroke-linejoin="round" d="m4.5 5.25 7.5 7.5 7.5-7.5m-15 6 7.5 7.5 7.5-7.5" />
                </svg>

            </div>
        </section>

        <section class="md:snap-start md:shrink-0 md:h-screen md:w-screen">
            <div class="md:h-screen">
                <div class="overflow-hidden py-24 sm:py-32">
                    <div class="mx-auto max-w-7xl px-6 lg:px-8">
                        <div class="mx-auto grid max-w-2xl grid-cols-1 gap-x-8 gap-y-16 sm:gap-y-20 lg:mx-0 lg:max-w-none lg:grid-cols-2">
                            <div class="lg:ml-auto lg:pl-4 lg:pt-4">
                                <div class="lg:max-w-lg">
                                    <h2 class="text-base font-semibold leading-7 text-indigo-400">Stay online, no matter what comes at you</h2>
                                    <p class="mt-2 text-3xl font-bold tracking-tight text-slate-300 sm:text-4xl">Validate your traffic</p>
                                    <p class="mt-6 text-lg leading-8 text-slate-400">
                                        Stop bots and allow only human traffic to reach your servers, without any friction. Give your users consistently incredible experiences.
                                    </p>
                                    <dl class="mt-10 max-w-xl text-base leading-7 text-slate-400 lg:max-w-none">
                                        <div class="relative">
                                            <dt class="inline font-semibold text-slate-300">
                                                <!--
                                                <svg class="absolute left-1 top-1 h-5 w-5 text-indigo-600" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                                    <path fill-rule="evenodd" d="M10 1a4.5 4.5 0 00-4.5 4.5V9H5a2 2 0 00-2 2v6a2 2 0 002 2h10a2 2 0 002-2v-6a2 2 0 00-2-2h-.5V5.5A4.5 4.5 0 0010 1zm3 8V5.5a3 3 0 10-6 0V9h6z" clip-rule="evenodd" />
                                                </svg>
                                                -->
                                                Stop DDoS easily.
                                            </dt>
                                            <dd class="inline">
                                                Using a combination of statistically significant metrics and user information, we target attacks with high efficiency.
                                            </dd>
                                        </div>
                                        <div class="relative">
                                            <dt class="inline font-semibold text-slate-300">
                                                <!--
                                                <svg class="absolute left-1 top-1 h-5 w-5 text-indigo-600" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                                    <path fill-rule="evenodd" d="M10 1a4.5 4.5 0 00-4.5 4.5V9H5a2 2 0 00-2 2v6a2 2 0 002 2h10a2 2 0 002-2v-6a2 2 0 00-2-2h-.5V5.5A4.5 4.5 0 0010 1zm3 8V5.5a3 3 0 10-6 0V9h6z" clip-rule="evenodd" />
                                                </svg>
                                                -->
                                                Goes unnoticed by users.
                                            </dt>
                                            <dd class="inline">
                                                Protection that's always invisible, because users should never have to prove they're human using their intellect. No more clicking on buses.
                                            </dd>
                                        </div>
                                        <div class="relative">
                                            <dt class="inline font-semibold text-slate-300">
                                                <!--
                                                <svg class="absolute left-1 top-1 h-5 w-5 text-indigo-600" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                                    <path fill-rule="evenodd" d="M12 6v12m-3-2.818.879.659c1.171.879 3.07.879 4.242 0 1.172-.879 1.172-2.303 0-3.182C13.536 12.219 12.768 12 12 12c-.725 0-1.45-.22-2.003-.659-1.106-.879-1.106-2.303 0-3.182s2.9-.879 4.006 0l.415.33M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" clip-rule="evenodd" />
                                                </svg>
                                                -->
                                                Don't pay for attacks.
                                            </dt>
                                            <dd class="inline">
                                                Billing that never increases with attacks, by design. Badass mitigation of threats, without the bad surprises.
                                            </dd>
                                        </div>
                                        <!-- proof of work, caching, background challenge -->
                                    </dl>
                                </div>
                            </div>
                            <div class="flex items-start justify-end lg:order-first">
                                <img src="/he-two.jpeg" alt="Product screenshot" class="md:visible invisible w-[48rem] h-[32rem] max-w-none rounded-xl shadow-xl ring-1 ring-gray-400/10 sm:w-[57rem]" width="2432" height="1442">
                            </div>
                        </div>
                    </div>
                </div>

            </div>
        </section>

        <section class="md:snap-start md:shrink-0 md:h-screen md:w-screen">
            <div class="md:h-screen">
                <div class="overflow-hidden py-24 sm:py-32">
                    <div class="mx-auto max-w-7xl px-6 lg:px-8 pt-12">
                        <div class="mx-auto max-w-2xl lg:mx-0">
                            <h2 class="text-3xl font-bold tracking-tight text-slate-300 sm:text-4xl">There's more</h2>
                            <p class="mt-6 text-lg leading-8 text-slate-400">
                                The best cybersecurity platforms go unnoticed during attacks: whether from outages or CAPTCHAs.
                                Human Engine is a true gem: it's always there, but never in the way of your loved users.
                            </p>
                        </div>
                        <div class="mx-auto mt-16 max-w-2xl sm:mt-20 lg:mt-24 lg:max-w-none">
                            <dl class="grid max-w-xl grid-cols-1 gap-x-8 gap-y-16 lg:max-w-none lg:grid-cols-3">
                                <div class="flex flex-col">
                                    <dt class="text-base font-semibold leading-7 text-slate-300">
                                        <div class="mb-6 flex h-10 w-10 items-center justify-center rounded-lg bg-indigo-600">
                                            <svg class="h-6 w-6 text-white" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M16.5 10.5V6.75a4.5 4.5 0 1 0-9 0v3.75m-.75 11.25h10.5a2.25 2.25 0 0 0 2.25-2.25v-6.75a2.25 2.25 0 0 0-2.25-2.25H6.75a2.25 2.25 0 0 0-2.25 2.25v6.75a2.25 2.25 0 0 0 2.25 2.25Z" />                                            </svg>
                                        </div>
                                        Proof-of-Work requirements
                                    </dt>
                                    <dd class="mt-1 flex flex-auto flex-col text-base leading-7 text-slate-400">
                                        <p class="flex-auto">
                                            Ensure that users accessing your frontends and APIs are human, by requiring them to perform a small amount of work.
                                        </p>
                                        <p class="mt-6">
                                            <a href="#" class="text-sm font-semibold leading-6 text-indigo-600">Learn more <span aria-hidden="true">→</span></a>
                                        </p>
                                    </dd>
                                </div>
                                <div class="flex flex-col">
                                    <dt class="text-base font-semibold leading-7 text-slate-300">
                                        <div class="mb-6 flex h-10 w-10 items-center justify-center rounded-lg bg-indigo-600">
                                            <svg class="h-6 w-6 text-white" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M5.25 14.25h13.5m-13.5 0a3 3 0 0 1-3-3m3 3a3 3 0 1 0 0 6h13.5a3 3 0 1 0 0-6m-16.5-3a3 3 0 0 1 3-3h13.5a3 3 0 0 1 3 3m-19.5 0a4.5 4.5 0 0 1 .9-2.7L5.737 5.1a3.375 3.375 0 0 1 2.7-1.35h7.126c1.062 0 2.062.5 2.7 1.35l2.587 3.45a4.5 4.5 0 0 1 .9 2.7m0 0a3 3 0 0 1-3 3m0 3h.008v.008h-.008v-.008Zm0-6h.008v.008h-.008v-.008Zm-3 6h.008v.008h-.008v-.008Zm0-6h.008v.008h-.008v-.008Z" />
                                            </svg>
                                        </div>
                                        Integrates with caching
                                    </dt>
                                    <dd class="mt-1 flex flex-auto flex-col text-base leading-7 text-slate-400">
                                        <p class="flex-auto">
                                            Works seamlessly with caching (storing temporary copies of your content),
                                            so we'll absorb any excess traffic without ever reaching your servers.
                                        </p>
                                        <p class="mt-6">
                                            <a href="#" class="text-sm font-semibold leading-6 text-indigo-600">Learn more <span aria-hidden="true">→</span></a>
                                        </p>
                                    </dd>
                                </div>
                                <div class="flex flex-col">
                                    <dt class="text-base font-semibold leading-7 text-slate-300">
                                        <div class="mb-6 flex h-10 w-10 items-center justify-center rounded-lg bg-indigo-600">
                                            <svg class="h-6 w-6 text-white" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M21 8.25c0-2.485-2.099-4.5-4.688-4.5-1.935 0-3.597 1.126-4.312 2.733-.715-1.607-2.377-2.733-4.313-2.733C5.1 3.75 3 5.765 3 8.25c0 7.22 9 12 9 12s9-4.78 9-12Z" />
                                            </svg>
                                        </div>
                                        No interruption
                                    </dt>
                                    <dd class="mt-1 flex flex-auto flex-col text-base leading-7 text-slate-400">
                                        <p class="flex-auto">
                                            Users are validated in the background, so there won't be interruptions to their experience, even if attacks mount.
                                        </p>
                                        <p class="mt-6">
                                            <a href="#" class="text-sm font-semibold leading-6 text-indigo-600">Learn more <span aria-hidden="true">→</span></a>
                                        </p>
                                    </dd>
                                </div>
                            </dl>
                        </div>
                    </div>
                </div>

            </div>
        </section>


        <section class="md:snap-start shrink-0 md:h-screen md:w-screen">
            <div class="px-5 py-24 sm:py-32">
                <div class="mx-auto max-w-7xl">
                    <div class="mx-auto grid max-w-2xl grid-cols-1 gap-x-8 gap-y-16 sm:gap-y-20 lg:mx-0 lg:max-w-none lg:grid-cols-3">
                        <dl class="col-span-2 grid grid-cols-1 gap-x-8 gap-y-10 text-base leading-7 text-slate-300 sm:grid-cols-2 lg:gap-y-16">
                            <div class="relative pl-9">
                                <dt class="font-semibold text-slate-200">
                                    <svg class="absolute left-0 top-1 h-5 w-5 text-indigo-500" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                        <path fill-rule="evenodd" d="M16.704 4.153a.75.75 0 01.143 1.052l-8 10.5a.75.75 0 01-1.127.075l-4.5-4.5a.75.75 0 011.06-1.06l3.894 3.893 7.48-9.817a.75.75 0 011.05-.143z" clip-rule="evenodd" />
                                    </svg>
                                    Analytics by Example
                                </dt>
                                <dd class="mt-2">
                                    See how real your users are with detailed analytics and logs, including the number of requests and the number of users.
                                </dd>
                            </div>
                            <div class="relative pl-9">
                                <dt class="font-semibold text-slate-200">
                                    <svg class="absolute left-0 top-1 h-5 w-5 text-indigo-500" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                        <path fill-rule="evenodd" d="M16.704 4.153a.75.75 0 01.143 1.052l-8 10.5a.75.75 0 01-1.127.075l-4.5-4.5a.75.75 0 011.06-1.06l3.894 3.893 7.48-9.817a.75.75 0 011.05-.143z" clip-rule="evenodd" />
                                    </svg>
                                    Highly Complex
                                </dt>
                                <dd class="mt-2">
                                    A blend of traditional methods, machine learning, and statistically significant analysis to ensure your website's foundation is a rock.
                                </dd>
                            </div>
                            <div class="relative pl-9">
                                <dt class="font-semibold text-slate-200">
                                    <svg class="absolute left-0 top-1 h-5 w-5 text-indigo-500" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                        <path fill-rule="evenodd" d="M16.704 4.153a.75.75 0 01.143 1.052l-8 10.5a.75.75 0 01-1.127.075l-4.5-4.5a.75.75 0 011.06-1.06l3.894 3.893 7.48-9.817a.75.75 0 011.05-.143z" clip-rule="evenodd" />
                                    </svg>
                                    Set & Forget
                                </dt>
                                <dd class="mt-2">
                                    Deliberately designed to be incredibly effective, stopping volumetric threats, out of the box. No special configurations required.
                                </dd>
                            </div>
                            <div class="relative pl-9">
                                <dt class="font-semibold text-slate-200">
                                    <svg class="absolute left-0 top-1 h-5 w-5 text-indigo-500" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                        <path fill-rule="evenodd" d="M16.704 4.153a.75.75 0 01.143 1.052l-8 10.5a.75.75 0 01-1.127.075l-4.5-4.5a.75.75 0 011.06-1.06l3.894 3.893 7.48-9.817a.75.75 0 011.05-.143z" clip-rule="evenodd" />
                                    </svg>
                                    Stops headless browsers
                                </dt>
                                <dd class="mt-2">
                                    Invisible challenge so powerful, it stops headless browsers and other automated tools in their tracks.
                                </dd>
                            </div>
                            <div class="relative pl-9">
                                <dt class="font-semibold text-slate-200">
                                    <svg class="absolute left-0 top-1 h-5 w-5 text-indigo-500" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                        <path fill-rule="evenodd" d="M16.704 4.153a.75.75 0 01.143 1.052l-8 10.5a.75.75 0 01-1.127.075l-4.5-4.5a.75.75 0 011.06-1.06l3.894 3.893 7.48-9.817a.75.75 0 011.05-.143z" clip-rule="evenodd" />
                                    </svg>
                                    Allows Good Bots
                                </dt>
                                <dd class="mt-2">
                                    Allow good bots, like search engine crawlers, to access your website. Customize other bots with Page Rules.
                                </dd>
                            </div>
                            <div class="relative pl-9">
                                <dt class="font-semibold text-slate-200">
                                    <svg class="absolute left-0 top-1 h-5 w-5 text-indigo-500" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                        <path fill-rule="evenodd" d="M16.704 4.153a.75.75 0 01.143 1.052l-8 10.5a.75.75 0 01-1.127.075l-4.5-4.5a.75.75 0 011.06-1.06l3.894 3.893 7.48-9.817a.75.75 0 011.05-.143z" clip-rule="evenodd" />
                                    </svg>
                                    Concierge that has your back.
                                </dt>
                                <dd class="mt-2">
                                    Our concierge team, made of Packetware developers, quickly responds to any questions or issues you may have.
                                </dd>
                            </div>
                        </dl>

                        <div>
                            <p class="mt-2 text-3xl font-bold tracking-tight text-slate-200 sm:text-4xl">DDoS Mitigation That Really, Really Works.</p>
                            <p class="mt-6 text-base leading-7 text-slate-300">
                                We spoke to hundreds of developers to understand why typical DDoS mitigation solutions are so frustrating.
                                We built Human Engine to be highly effective, swift, and fuckin' badass.
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </section>

        <section class="md:snap-start shrink-0 md:h-screen md:w-screen">
            <div class="px-5 py-24 sm:py-32">
                <Rizz />
            </div>
        </section>
    </div>
</section>
