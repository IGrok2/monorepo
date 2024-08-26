<script>
import Navbar from "$lib/Navbar.svelte";
import Rizz from "$lib/Rizz.svelte";
import BasicLink from "$lib/base/BasicLink.svelte";
import DocsLink from "$lib/base/DocsLink.svelte";
import Footer from "$lib/Footer.svelte";
</script>

<section class="bg-gradient-to-tr from-zinc-900 to-90% to-fuchsia-900 relative text-slate-300 font-mona">
    <div class="overflow-y-auto md:relative h-screen md:snap-y md:snap-mandatory pb-14">
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
        <Navbar />
        <section class="md:snap-start md:h-screen md:w-screen">
            <div class="md:h-screen md:w-screen pt-14 p-5 md:p-20">
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
                        <p class="md:text-6xl text-5xl pb-3">API Engine. For validating programmable traffic.</p>
                        <p class="text-2xl pt-5 md:text-3xl">
                            Validate what goes into your API endpoints from invalid and malicious requests, bot abuse, and DDoS attacks.
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
                <div class="overflow-hidden pb-24 sm:py-32">
                    <div class="mx-auto max-w-7xl px-6 lg:px-8">
                        <div class="mx-auto grid max-w-2xl grid-cols-1 gap-x-8 gap-y-16 sm:gap-y-20 lg:mx-0 lg:max-w-none lg:grid-cols-2 lg:items-start">
                            <div class="lg:pr-4 lg:pt-4">
                                <div class="lg:max-w-lg">
                                    <h2 class="text-base font-semibold leading-7 text-indigo-400">Invisible Proof-of-Work Validation</h2>
                                    <p class="mt-2 text-4xl font-bold tracking-tight text-slate-200 sm:text-4xl">Human-Validated Endpoints</p>
                                    <p class="mt-6 text-lg leading-8 text-slate-300">
                                        Some endpoints are designed to be accessed by humans only. API Engine validates user API traffic, directly from your frontend, invisibly ensuring only humans can pass.
                                    </p>
                                    <div class="mt-8 flex items-center">
                                        <a href="#" class="mr-2 inline-flex rounded-md bg-indigo-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">Get started</a>
                                        <BasicLink text="Documentation" href="/docs" />
                                    </div>
                                    <figure class="mt-16 border-l border-gray-200 pl-8 text-gray-300">
                                        <blockquote class="text-base leading-7">
                                            <p>
                                                "API Engine human validation is a game changer. My human-only endpoints are protected from bots, and users are not inconvenienced."
                                            </p>
                                        </blockquote>
                                        <figcaption class="mt-6 flex gap-x-4 text-sm leading-6">
                                            <img src="/api-he.jpeg" alt="" class="h-6 w-6 flex-none rounded-full">
                                            <div><span class="font-semibold text-gray-200">Lily Hellriegel</span> – Cloud Engineer</div>
                                        </figcaption>
                                    </figure>
                                </div>
                            </div>

                            <img src="/api-he.jpeg" alt="Product screenshot" class="md:visible invisible w-[48rem] h-[32rem] max-w-none rounded-xl shadow-xl ring-1 ring-gray-400/10 sm:w-[57rem] md:-ml-4 lg:ml-0" width="2432" height="1442">
                        </div>
                    </div>
                </div>
            </div>
        </section>

        <section class="md:snap-start md:shrink-0 md:h-screen md:w-screen">
            <div class="py-24 sm:py-32">
                <div class="mx-auto max-w-7xl px-6 lg:px-8">
                    <div class="mx-auto max-w-2xl lg:text-center">
                        <h2 class="text-base font-semibold leading-7 text-indigo-400">Stop Malicious Requests</h2>
                        <p class="mt-2 text-3xl font-bold tracking-tight sm:text-4xl">Web Application Firewall</p>
                        <p class="mt-6 text-lg leading-8 text-slate-300">
                            Protect API endpoints from malicious requests, including SQL injection, XSS, credential stuffing, and other common web application attacks. Easy to use and deploy.
                        </p>
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

                                    Microcaching
                                </dt>
                                <dd class="mt-4 flex flex-auto flex-col text-base leading-7 text-slate-300">
                                    <p class="flex-auto">
                                        Cache public API responses for a few seconds, reducing the load on your API server and protecting against DDoS attacks.
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
                                    See how your API is utilized with highly detailed analytics showing statistically significant traffic.
                                </dd>
                            </div>
                            <div class="relative pl-9">
                                <dt class="font-semibold text-slate-200">
                                    <svg class="absolute left-0 top-1 h-5 w-5 text-indigo-500" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                        <path fill-rule="evenodd" d="M16.704 4.153a.75.75 0 01.143 1.052l-8 10.5a.75.75 0 01-1.127.075l-4.5-4.5a.75.75 0 011.06-1.06l3.894 3.893 7.48-9.817a.75.75 0 011.05-.143z" clip-rule="evenodd" />
                                    </svg>
                                    Reduce attack surface.
                                </dt>
                                <dd class="mt-2">
                                    Be as specific as you'd like with what your API should allow, based on first principles.
                                </dd>
                            </div>
                            <div class="relative pl-9">
                                <dt class="font-semibold text-slate-200">
                                    <svg class="absolute left-0 top-1 h-5 w-5 text-indigo-500" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                        <path fill-rule="evenodd" d="M16.704 4.153a.75.75 0 01.143 1.052l-8 10.5a.75.75 0 01-1.127.075l-4.5-4.5a.75.75 0 011.06-1.06l3.894 3.893 7.48-9.817a.75.75 0 011.05-.143z" clip-rule="evenodd" />
                                    </svg>
                                    IP Information
                                </dt>
                                <dd class="mt-2">
                                    Receive IP address information, such as country and ASN, for every request via HTTP headers to your origin.
                                </dd>
                            </div>
                            <div class="relative pl-9">
                                <dt class="font-semibold text-slate-200">
                                    <svg class="absolute left-0 top-1 h-5 w-5 text-indigo-500" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                        <path fill-rule="evenodd" d="M16.704 4.153a.75.75 0 01.143 1.052l-8 10.5a.75.75 0 01-1.127.075l-4.5-4.5a.75.75 0 011.06-1.06l3.894 3.893 7.48-9.817a.75.75 0 011.05-.143z" clip-rule="evenodd" />
                                    </svg>
                                    Integrates with Page Rules
                                </dt>
                                <dd class="mt-2">
                                    Use Page Rules for incredibly flexible and granular control over how your API behaves.
                                </dd>
                            </div>
                            <div class="relative pl-9">
                                <dt class="font-semibold text-slate-200">
                                    <svg class="absolute left-0 top-1 h-5 w-5 text-indigo-500" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                        <path fill-rule="evenodd" d="M16.704 4.153a.75.75 0 01.143 1.052l-8 10.5a.75.75 0 01-1.127.075l-4.5-4.5a.75.75 0 011.06-1.06l3.894 3.893 7.48-9.817a.75.75 0 011.05-.143z" clip-rule="evenodd" />
                                    </svg>
                                    Always Improving
                                </dt>
                                <dd class="mt-2">
                                    We're always adding new features and improving existing ones to keep your API secure, including quick patches to the WAF.
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
                            <p class="mt-2 text-3xl font-bold tracking-tight text-slate-200 sm:text-4xl">API security with 100% coverage.</p>
                            <p class="mt-6 text-base leading-7 text-slate-300">
                                Scale freely and securely when you know your API is protected by API Engine. Focus on your core product.
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </section>

        <section class="snap-start shrink-0 h-screen w-screen flex items-center">
            <Rizz />

            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="absolute inset-y-full inset-x-1/2 -translate-x-1/2 w-12 h-12">
                <path stroke-linecap="round" stroke-linejoin="round" d="m4.5 5.25 7.5 7.5 7.5-7.5m-15 6 7.5 7.5 7.5-7.5" />
            </svg>
        </section>
    <!-- programmatible -->

</div>

    <Footer/>
</section>
<Footer/>
