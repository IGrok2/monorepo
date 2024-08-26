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
                        <p class="md:text-6xl text-5xl pb-3">Content delivery network for developers.</p>
                        <p class="text-2xl pt-5 md:text-3xl">
                            The stress-free cybersecurity suite you've always wanted.
                            <br />
                            The performance to conquer the world.
                            <br />
                            The customization to make it your own.
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
            <div class="py-24 sm:py-24">
                <div class="overflow-hidden">
                    <div class="mx-auto max-w-7xl px-6 lg:px-8">
                        <div class="mx-auto grid max-w-2xl grid-cols-1 gap-x-8 gap-y-16 sm:gap-y-20 lg:mx-0 lg:max-w-none lg:grid-cols-2">
                            <div class="lg:ml-auto lg:pl-4 lg:pt-4">
                                <div class="lg:max-w-lg">
                                    <h2 class="text-base font-semibold leading-7 text-indigo-400">Deploy the next ChatGPT</h2>
                                    <p class="mt-2 text-3xl font-bold tracking-tight text-slate-300 sm:text-4xl">Cache Engine: Badass Performance.</p>
                                    <p class="mt-6 text-lg leading-8 text-slate-400">
                                        Deliver your content to the world, without fear of scale or slow performance.
                                    </p>
                                    <dl class="mt-5 max-w-xl space-y-8 text-base leading-7 text-slate-400 lg:max-w-none">
                                        <div class="relative pl-9">
                                            <dt class="inline font-semibold text-slate-300">
                                                <svg class="absolute left-1 top-1 h-5 w-5 text-indigo-600" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                                    <path fill-rule="evenodd" d="M5.5 17a4.5 4.5 0 01-1.44-8.765 4.5 4.5 0 018.302-3.046 3.5 3.5 0 014.504 4.272A4 4 0 0115 17H5.5zm3.75-2.75a.75.75 0 001.5 0V9.66l1.95 2.1a.75.75 0 101.1-1.02l-3.25-3.5a.75.75 0 00-1.1 0l-3.25 3.5a.75.75 0 101.1 1.02l1.95-2.1v4.59z" clip-rule="evenodd" />
                                                </svg>
                                                Caching, rethought.
                                            </dt>
                                            <dd class="inline">
                                                Cache Engine allows you to cache large files with more control and is extraordinarily fast. It can deliver your content to the world in milliseconds with unlimited scale.
                                            </dd>
                                        </div>
                                        <div class="relative pl-9">
                                            <dt class="inline font-semibold text-slate-300">
                                                <svg class="absolute left-1 top-1 h-5 w-5 text-indigo-600" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                                    <path fill-rule="evenodd" d="M10 1a4.5 4.5 0 00-4.5 4.5V9H5a2 2 0 00-2 2v6a2 2 0 002 2h10a2 2 0 002-2v-6a2 2 0 00-2-2h-.5V5.5A4.5 4.5 0 0010 1zm3 8V5.5a3 3 0 10-6 0V9h6z" clip-rule="evenodd" />
                                                </svg>
                                                One sink for multiple drains.
                                            </dt>
                                            <dd class="inline">
                                                Larger files can take some time to cache. Cache Engine can merge thousands of concurrent requests for the same file into one. Great for streaming.
                                            </dd>
                                        </div>

                                        <div class="relative pl-9">
                                            <dt class="inline font-semibold text-slate-300">
                                                <svg class="absolute left-1 top-1 h-5 w-5 text-indigo-600" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                                    <path d="M4.632 3.533A2 2 0 016.577 2h6.846a2 2 0 011.945 1.533l1.976 8.234A3.489 3.489 0 0016 11.5H4c-.476 0-.93.095-1.344.267l1.976-8.234z" />
                                                    <path fill-rule="evenodd" d="M4 13a2 2 0 100 4h12a2 2 0 100-4H4zm11.24 2a.75.75 0 01.75-.75H16a.75.75 0 01.75.75v.01a.75.75 0 01-.75.75h-.01a.75.75 0 01-.75-.75V15zm-2.25-.75a.75.75 0 00-.75.75v.01c0 .414.336.75.75.75H13a.75.75 0 00.75-.75V15a.75.75 0 00-.75-.75h-.01z" clip-rule="evenodd" />
                                                </svg>
                                                Automatic minification and compression.
                                            </dt>
                                            <dd class="inline">
                                                We automatically minify and compress your content, so it uses less bandwidth and delivers quicker, especially over slow connections.
                                            </dd>
                                        </div>
                                    </dl>
                                </div>
                            </div>
                            <div class="flex items-start justify-end lg:order-first">
                                <img src="/cache-it-baby!.png" alt="Product screenshot" class="w-[48rem] max-w-none rounded-xl shadow-xl ring-1 ring-gray-400/10 sm:w-[57rem]" width="2432" height="1442">
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>

        <section class="md:snap-start md:shrink-0 md:h-screen md:w-screen">
            <div class="py-24 sm:py-32">
                <div class="mx-auto max-w-7xl px-6 lg:px-8">
                    <div class="mx-auto max-w-2xl lg:text-center">
                        <h2 class="text-base font-semibold leading-7 text-indigo-400">API Engine</h2>
                        <p class="mt-2 text-3xl font-bold tracking-tight sm:text-4xl">Web Application Firewall</p>
                        <p class="mt-6 text-lg leading-8 text-slate-300">
                            Protect API endpoints from malicious requests, including SQL injection, XSS, credential stuffing, and other common web application attacks. Easy to use and deploy.
                        </p>
                        <a href="/feature/api-engine" class="font-semibold leading-7 text-indigo-400 underline">Learn more about API Engine</a>
                    </div>

                    <div class="mx-auto mt-16 max-w-2xl sm:mt-20 lg:mt-24 lg:max-w-none">
                        <dl class="grid max-w-xl grid-cols-1 gap-x-8 gap-y-16 lg:max-w-none lg:grid-cols-3">
                            <div class="flex flex-col">
                                <dt class="flex items-center gap-x-3 text-base font-semibold leading-7">
                                    <svg class="h-5 w-5 flex-none text-indigo-600" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                        <path fill-rule="evenodd" d="M10 1a4.5 4.5 0 00-4.5 4.5V9H5a2 2 0 00-2 2v6a2 2 0 002 2h10a2 2 0 002-2v-6a2 2 0 00-2-2h-.5V5.5A4.5 4.5 0 0010 1zm3 8V5.5a3 3 0 10-6 0V9h6z" clip-rule="evenodd" />
                                    </svg>
                                    Stops What's Important
                                </dt>
                                <dd class="mt-4 flex flex-auto flex-col text-base leading-7 text-slate-300">
                                    <p class="flex-auto">Stops traditional threats and cutting-edge threats, out of the box. Custom application filters for WordPress and Pterodactyl.</p>
                                    <p class="mt-6">
                                        <a href="#" class="text-sm font-semibold leading-6 text-indigo-600">Learn more <span aria-hidden="true">→</span></a>
                                    </p>
                                </dd>
                            </div>
                            <div class="flex flex-col">
                                <dt class="flex items-center gap-x-3 text-base font-semibold leading-7">
                                    <svg class="h-5 w-5 flex-none text-indigo-600" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="m20.25 7.5-.625 10.632a2.25 2.25 0 0 1-2.247 2.118H6.622a2.25 2.25 0 0 1-2.247-2.118L3.75 7.5m8.25 3v6.75m0 0-3-3m3 3 3-3M3.375 7.5h17.25c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125Z" />
                                    </svg>

                                    Invisible Proof-of-Work Requirements
                                </dt>
                                <dd class="mt-4 flex flex-auto flex-col text-base leading-7 text-slate-300">
                                    <p class="flex-auto">
                                        Some endpoints are designed to be accessed by humans only. API Engine validates user API traffic, directly from your frontend, invisibly ensuring only humans can pass.
                                    </p>
                                    <p class="mt-6">
                                        <a href="#" class="text-sm font-semibold leading-6 text-indigo-600">Learn more <span aria-hidden="true">→</span></a>
                                    </p>
                                </dd>
                            </div>
                            <div class="flex flex-col">
                                <dt class="flex items-center gap-x-3 text-base font-semibold leading-7">
                                    <svg class="h-5 w-5 flex-none text-indigo-600" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M21 16.811c0 .864-.933 1.406-1.683.977l-7.108-4.061a1.125 1.125 0 0 1 0-1.954l7.108-4.061A1.125 1.125 0 0 1 21 8.689v8.122ZM11.25 16.811c0 .864-.933 1.406-1.683.977l-7.108-4.061a1.125 1.125 0 0 1 0-1.954l7.108-4.061a1.125 1.125 0 0 1 1.683.977v8.122Z" />
                                    </svg>

                                    Ratelimit from JSON key to IP
                                </dt>
                                <dd class="mt-4 flex flex-auto flex-col text-base leading-7 text-slate-300">
                                    <p class="flex-auto">
                                        Utilize buckets to ratelimit requests based on JSON keys, IP addresses, and other request attributes.
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
        </section>

        <section class="md:snap-start md:shrink-0 md:h-screen md:w-screen">
            <div class="md:h-screen">
                <div class="overflow-hidden py-24 sm:py-32">
                    <div class="mx-auto max-w-7xl px-6 lg:px-8">
                        <div class="mx-auto grid max-w-2xl grid-cols-1 gap-x-8 gap-y-16 sm:gap-y-20 lg:mx-0 lg:max-w-none lg:grid-cols-2">
                            <div class="lg:pr-8 lg:pt-4">
                                <div class="lg:max-w-lg">
                                    <h2 class="text-base font-semibold leading-7 text-indigo-400">Unlimited customization</h2>
                                    <p class="mt-2 text-3xl font-bold tracking-tight text-slate-300 sm:text-4xl">Page Rules</p>
                                    <p class="mt-6 text-lg leading-8 text-slate-400">
                                        Customize every aspect of your site on Packetware with Page Rules. Make it unmistakably yours.
                                    </p>
                                    <dl class="mt-10 max-w-xl space-y-8 text-base leading-7 text-slate-400 lg:max-w-none">
                                        <div class="relative pl-9">
                                            <dt class="inline font-semibold text-slate-300">
                                                <svg class="absolute left-1 top-1 h-5 w-5 text-indigo-600" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                                    <path fill-rule="evenodd" d="M5.5 17a4.5 4.5 0 01-1.44-8.765 4.5 4.5 0 018.302-3.046 3.5 3.5 0 014.504 4.272A4 4 0 0115 17H5.5zm3.75-2.75a.75.75 0 001.5 0V9.66l1.95 2.1a.75.75 0 101.1-1.02l-3.25-3.5a.75.75 0 00-1.1 0l-3.25 3.5a.75.75 0 101.1 1.02l1.95-2.1v4.59z" clip-rule="evenodd" />
                                                </svg>
                                                Set a trigger.
                                            </dt>
                                            <dd class="inline">
                                                Trigger based on IP, country, ASN, headers, methods, paths, query string, cookies and more.
                                            </dd>
                                        </div>
                                        <div class="relative pl-9">
                                            <dt class="inline font-semibold text-slate-300">
                                                <svg class="absolute left-1 top-1 h-5 w-5 text-indigo-600" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                                    <path fill-rule="evenodd" d="M10 1a4.5 4.5 0 00-4.5 4.5V9H5a2 2 0 00-2 2v6a2 2 0 002 2h10a2 2 0 002-2v-6a2 2 0 00-2-2h-.5V5.5A4.5 4.5 0 0010 1zm3 8V5.5a3 3 0 10-6 0V9h6z" clip-rule="evenodd" />
                                                </svg>
                                                Set a match type.
                                            </dt>
                                            <dd class="inline">
                                                Match based on exact, prefix, suffix, or contains. Then combine match types for complex rules.
                                            </dd>
                                        </div>
                                        <div class="relative pl-9">
                                            <dt class="inline font-semibold text-slate-300">
                                                <svg class="absolute left-1 top-1 h-5 w-5 text-indigo-600" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                                    <path d="M4.632 3.533A2 2 0 016.577 2h6.846a2 2 0 011.945 1.533l1.976 8.234A3.489 3.489 0 0016 11.5H4c-.476 0-.93.095-1.344.267l1.976-8.234z" />
                                                    <path fill-rule="evenodd" d="M4 13a2 2 0 100 4h12a2 2 0 100-4H4zm11.24 2a.75.75 0 01.75-.75H16a.75.75 0 01.75.75v.01a.75.75 0 01-.75.75h-.01a.75.75 0 01-.75-.75V15zm-2.25-.75a.75.75 0 00-.75.75v.01c0 .414.336.75.75.75H13a.75.75 0 00.75-.75V15a.75.75 0 00-.75-.75h-.01z" clip-rule="evenodd" />
                                                </svg>
                                                Set your action.
                                            </dt>
                                            <dd class="inline">
                                                Choose from a variety of actions, including changing cache settings, modifying headers, and redirecting requests.
                                            </dd>
                                        </div>
                                    </dl>
                                </div>
                            </div>
                            <img src="/page_rules.png" alt="Product screenshot" class="w-[48rem] max-w-none rounded-xl shadow-xl ring-1 ring-gray-400/10 sm:w-[57rem] md:-ml-4 lg:-ml-0" width="2432" height="1442">
                        </div>
                    </div>
                </div>
            </div>
        </section>

        <section class="md:snap-start md:shrink-0 md:h-screen md:w-screen">
            <div class="md:h-screen">
                <div class="overflow-hidden py-24 sm:py-32">
                    <div class="mx-auto max-w-7xl px-6 lg:px-8">
                        <div class="mx-auto grid max-w-2xl grid-cols-1 gap-x-8 gap-y-16 sm:gap-y-20 lg:mx-0 lg:max-w-none lg:grid-cols-2">
                            <div class="lg:ml-auto lg:pl-4 lg:pt-4">
                                <div class="lg:max-w-lg">
                                    <a href="/feature/human-engine" class="text-base font-semibold leading-7 text-indigo-400 underline">Human Engine</a>
                                    <p class="mt-2 text-3xl font-bold tracking-tight text-slate-300 sm:text-4xl">Mitigate threats like DDoS and bad bots without breaking a sweat.</p>
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
                    <div class="mx-auto max-w-7xl px-6 lg:px-8">
                        <div class="mx-auto grid max-w-2xl grid-cols-1 gap-x-8 gap-y-16 sm:gap-y-20 lg:mx-0 lg:max-w-none lg:grid-cols-2">
                            <div class="lg:ml-auto lg:pl-4 lg:pt-4">
                                <div class="lg:max-w-lg">
                                    <h2 class="text-base font-semibold leading-7 text-indigo-400">
                                        Analytics By Example
                                    </h2>
                                    <p class="mt-2 text-3xl font-bold tracking-tight text-slate-300 sm:text-4xl">
                                        Don't Just View Numbers. See The Traffic.
                                    </p>
                                    <p class="mt-6 text-lg leading-8 text-slate-400">
                                        View rich data of requests and users coming into your sites and how they interact with your content.
                                    </p>
                                    <dl class="mt-10 max-w-xl text-base leading-7 text-slate-400 lg:max-w-none">
                                        <div class="relative">
                                            <dt class="inline font-semibold text-slate-300">
                                                <!--
                                                <svg class="absolute left-1 top-1 h-5 w-5 text-indigo-600" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                                    <path fill-rule="evenodd" d="M10 1a4.5 4.5 0 00-4.5 4.5V9H5a2 2 0 00-2 2v6a2 2 0 002 2h10a2 2 0 002-2v-6a2 2 0 00-2-2h-.5V5.5A4.5 4.5 0 0010 1zm3 8V5.5a3 3 0 10-6 0V9h6z" clip-rule="evenodd" />
                                                </svg>
                                                -->
                                                Statistically significant traffic.
                                            </dt>
                                            <dd class="inline">
                                                Don't get bogged down in showing every single request. Instead, see the most significant ones.
                                            </dd>
                                        </div>
                                        <div class="relative">
                                            <dt class="inline font-semibold text-slate-300">
                                                <!--
                                                <svg class="absolute left-1 top-1 h-5 w-5 text-indigo-600" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                                    <path fill-rule="evenodd" d="M10 1a4.5 4.5 0 00-4.5 4.5V9H5a2 2 0 00-2 2v6a2 2 0 002 2h10a2 2 0 002-2v-6a2 2 0 00-2-2h-.5V5.5A4.5 4.5 0 0010 1zm3 8V5.5a3 3 0 10-6 0V9h6z" clip-rule="evenodd" />
                                                </svg>
                                                -->
                                                Easy debuggability.
                                            </dt>
                                            <dd class="inline">
                                                Analytics By Example tags the request through the entire request lifetime with in-depth reporting of any errors.
                                                See how users interact with Packetware and your content.
                                            </dd>
                                        </div>
                                        <div class="relative">
                                            <dt class="inline font-semibold text-slate-300">
                                                <!--
                                                <svg class="absolute left-1 top-1 h-5 w-5 text-indigo-600" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                                    <path fill-rule="evenodd" d="M12 6v12m-3-2.818.879.659c1.171.879 3.07.879 4.242 0 1.172-.879 1.172-2.303 0-3.182C13.536 12.219 12.768 12 12 12c-.725 0-1.45-.22-2.003-.659-1.106-.879-1.106-2.303 0-3.182s2.9-.879 4.006 0l.415.33M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" clip-rule="evenodd" />
                                                </svg>
                                                -->
                                                See what we see in requests.
                                            </dt>
                                            <dd class="inline">
                                                Including IP, threat score, ASN, country, user agent, request lifetime, content requested, cache status, API Engine status, Page Rule actions, and more.
                                            </dd>
                                        </div>
                                        <!-- proof of work, caching, background challenge -->
                                    </dl>
                                </div>
                            </div>
                            <div class="flex items-start justify-end lg:order-first">
                                <img src="/by-example.png" alt="Product screenshot" class="md:visible invisible w-[48rem] h-[32rem] max-w-none rounded-xl shadow-xl ring-1 ring-gray-400/10 sm:w-[57rem]" width="2432" height="1442">
                            </div>
                        </div>
                    </div>
                </div>

            </div>
        </section>

        <section class="md:snap-start shrink-0 md:h-screen md:w-screen">
            <div class="px-5 py-24 sm:py-32">
                    <div class="mx-auto max-w-7xl px-6 lg:px-8">
                        <div class="mx-auto grid max-w-2xl grid-cols-1 gap-x-8 gap-y-16 sm:gap-y-20 lg:mx-0 lg:max-w-none lg:grid-cols-3">
                            <div>
                                <h2 class="text-base font-semibold leading-7 text-indigo-400">Everything you need</h2>
                                <p class="mt-2 text-3xl font-bold tracking-tight text-slate-300 sm:text-4xl">There's so much to love.</p>
                                <p class="mt-6 text-base leading-7 text-slate-400">
                                    Designed to rethink all the rules. Packetware is a new way to think about your web infrastructure. Built by Whole Lotta Heart, Corp.
                                </p>
                            </div>
                            <dl class="col-span-2 grid grid-cols-1 gap-x-8 gap-y-10 text-base leading-7 text-slate-400 sm:grid-cols-2 lg:gap-y-16">
                                <div class="relative pl-9">
                                    <dt class="font-semibold text-slate-300">
                                        <svg class="absolute left-0 top-1 h-5 w-5 text-indigo-500" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                            <path fill-rule="evenodd" d="M16.704 4.153a.75.75 0 01.143 1.052l-8 10.5a.75.75 0 01-1.127.075l-4.5-4.5a.75.75 0 011.06-1.06l3.894 3.893 7.48-9.817a.75.75 0 011.05-.143z" clip-rule="evenodd" />
                                        </svg>
                                        Bucket Ratelimiting
                                    </dt>
                                    <dd class="mt-2">
                                        Ratelimit requests based on JSON keys, IP addresses, and other request attributes, then share them across endpoints with API Engine and Page Rules.
                                    </dd>
                                </div>
                                <div class="relative pl-9">
                                    <dt class="font-semibold text-slate-300">
                                        <svg class="absolute left-0 top-1 h-5 w-5 text-indigo-500" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                            <path fill-rule="evenodd" d="M16.704 4.153a.75.75 0 01.143 1.052l-8 10.5a.75.75 0 01-1.127.075l-4.5-4.5a.75.75 0 011.06-1.06l3.894 3.893 7.48-9.817a.75.75 0 011.05-.143z" clip-rule="evenodd" />
                                        </svg>
                                        Global Network
                                    </dt>
                                    <dd class="mt-2">
                                        Built on a global network of servers, Packetware is where you'd like to be. Control our anycast network with Page Rules.
                                    </dd>
                                </div>
                                <div class="relative pl-9">
                                    <dt class="font-semibold text-slate-300">
                                        <svg class="absolute left-0 top-1 h-5 w-5 text-indigo-500" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                            <path fill-rule="evenodd" d="M16.704 4.153a.75.75 0 01.143 1.052l-8 10.5a.75.75 0 01-1.127.075l-4.5-4.5a.75.75 0 011.06-1.06l3.894 3.893 7.48-9.817a.75.75 0 011.05-.143z" clip-rule="evenodd" />
                                        </svg>
                                        Bot Management
                                    </dt>
                                    <dd class="mt-2">
                                        Pick and choose which bots you'd like to allow, then have us watch them for any abuse.
                                    </dd>
                                </div>
                                <div class="relative pl-9">
                                    <dt class="font-semibold text-slate-300">
                                        <svg class="absolute left-0 top-1 h-5 w-5 text-indigo-500" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                            <path fill-rule="evenodd" d="M16.704 4.153a.75.75 0 01.143 1.052l-8 10.5a.75.75 0 01-1.127.075l-4.5-4.5a.75.75 0 011.06-1.06l3.894 3.893 7.48-9.817a.75.75 0 011.05-.143z" clip-rule="evenodd" />
                                        </svg>
                                        Load Balancing
                                    </dt>
                                    <dd class="mt-2">
                                        Load balance traffic across your infrastructure, including your own servers and third-party services.
                                    </dd>
                                </div>
                                <div class="relative pl-9">
                                    <dt class="font-semibold text-slate-300">
                                        <svg class="absolute left-0 top-1 h-5 w-5 text-indigo-500" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                            <path fill-rule="evenodd" d="M16.704 4.153a.75.75 0 01.143 1.052l-8 10.5a.75.75 0 01-1.127.075l-4.5-4.5a.75.75 0 011.06-1.06l3.894 3.893 7.48-9.817a.75.75 0 011.05-.143z" clip-rule="evenodd" />
                                        </svg>
                                        Live Analytics
                                    </dt>
                                    <dd class="mt-2">
                                        See what's happening in real-time, including requests, threats, and more.
                                    </dd>
                                </div>
                                <div class="relative pl-9">
                                    <dt class="font-semibold text-slate-300">
                                        <svg class="absolute left-0 top-1 h-5 w-5 text-indigo-500" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                            <path fill-rule="evenodd" d="M16.704 4.153a.75.75 0 01.143 1.052l-8 10.5a.75.75 0 01-1.127.075l-4.5-4.5a.75.75 0 011.06-1.06l3.894 3.893 7.48-9.817a.75.75 0 011.05-.143z" clip-rule="evenodd" />
                                        </svg>
                                        Packetware Concierge
                                    </dt>
                                    <dd class="mt-2">
                                        A team of experts completely dedicated to your success.
                                    </dd>
                                </div>
                            </dl>
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