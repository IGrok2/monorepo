<script>
    import {fade} from "svelte/transition";
</script>


<div class="bg-gray-950 bg-opacity-90 md:pb-10 py-10">
    <div class="overflow-hidden px-5">
        <div class="mx-auto max-w-7xl">
            <div class="mx-auto max-w-2xl grid-cols-1 gap-x-8 gap-y-16 sm:gap-y-20 lg:mx-0 lg:max-w-none lg:grid-cols-2">
        <div class="md:pb-10">
            <div class="py-10 md:px-5">
                <div class="p-5">
                    <div class="text-slate-200">
                        <p class="text-4xl font-ultrawide">
                            When it comes to reverse proxy, clients love our attention to making their user experiences exceptional.
                        </p>
                    </div>
                </div>
                <div class="md:grid md:grid-cols-3 md:pt-5">
                    <div
                            class="md:col-span-2 md:pb-10"
                    >
                        <div
                                class="hover:shadow-2xl hover:shadow-blue-500 duration-500 m-5 p-5 pl-5 md:pl-10 rounded-lg shadow-xl bg-gradient-to-b from-fuchsia-950 to-fuchsia-500 bg-cover h-full -z-20"
                        >
                            <div
                                    class="bg-[url('/api_engine.svg')] bg-no-repeat bg-left-top h-full hover:bg-right-bottom duration-500"
                            >
                                <div class="h-full">
                                    <div
                                            class="flex md:w-full justify-between"
                                    >
                                        <p
                                                class="text-3xl md:text-4xl text-slate-200 justify-start font-krona"
                                        >
                                            Incredible API security.
                                        </p>
                                        <p class="justify-end text-white">
                                            ↗
                                        </p>
                                    </div>
                                    <p
                                            class="text-slate-50 text-xl md:text-2xl"
                                    >
                                        Protect your API from head to toe
                                        with a comprehensive and compelling
                                        platform. Includes WAF, schema
                                        validation, proof-of-work
                                        requirements, intelligent WebSocket
                                        security, microcaching, and advanced
                                        ratelimiting.
                                    </p>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="md:pb-10">
                        <div
                                class="m-5 p-5 pl-5 md:pl-10 hover:shadow-fuchsia-500 duration-500 rounded-lg shadow-xl bg-gradient-to-b from-blue-950 to-blue-500 bg-cover h-full -z-20"
                        >
                            <div
                                    class="bg-[url('/globe.svg')] bg-no-repeat bg-right-bottom h-full hover:bg-left-top duration-500"
                            >
                                <div class="z-10">
                                    <div
                                            class="flex md:w-full justify-between"
                                    >
                                        <p
                                                class="text-3xl md:text-4xl text-slate-200 justify-start font-krona"
                                        >
                                            Global network
                                        </p>
                                        <p class="justify-end text-white">
                                            ↗
                                        </p>
                                    </div>
                                    <p
                                            class="text-slate-50 text-xl md:text-2xl font-mona"
                                    >
                                        We're everywhere you want to be, and
                                        then some. Huge network capacity to
                                        dance whilst handling the highest
                                        peaks.
                                    </p>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="md:pb-10">
                        <div
                                class="m-5 p-5 pl-5 md:pl-14 rounded-lg shadow-xl bg-cover h-full bg-gradient-to-b from-blue-900 to-blue-500 hover:shadow-fuchsia-500 duration-500"
                        >
                            <div
                                    class="bg-[url('/buckets.svg')] bg-no-repeat bg-left-top h-full hover:bg-right-bottom duration-500"
                            >
                                <div class="flex md:w-full justify-between">
                                    <p
                                            class="text-3xl md:text-4xl text-slate-200 justify-start font-krona"
                                    >
                                        Ratelimit from JSON key to IP
                                    </p>
                                    <p class="justify-end text-white">↗</p>
                                </div>
                                <p
                                        class="text-slate-50 text-xl md:text-2xl font-mona"
                                >
                                    Flexible and advanced bucket-based
                                    ratelimiting that can be shared across
                                    API endpoints and page rules.
                                </p>
                            </div>
                        </div>
                    </div>
                    <div class="md:pb-10 md:col-span-2">
                        <div
                                class="m-5 p-5 pl-5 md:pl-14 rounded-lg shadow-xl from-fuchsia-900 to-fuchsia-500 bg-cover h-full -z-20 bg-gradient-to-b hover:shadow-blue-500 duration-500"
                        >
                            <div
                                    class="bg-[url('/human_engine.svg')] bg-no-repeat bg-right-bottom h-full hover:bg-left-top duration-500"
                            >
                                <div>
                                    <div
                                            class="flex md:w-full justify-between"
                                    >
                                        <p
                                                class="text-3xl md:text-4xl text-slate-200 justify-start font-krona"
                                        >
                                            We think DDoS should never
                                            impact user experiences. Even in
                                            the small ways.
                                        </p>
                                        <p class="justify-end text-white">
                                            ↗
                                        </p>
                                    </div>
                                    <p
                                            class="text-slate-50 text-xl md:text-2xl font-mona"
                                    >
                                        The most effective DDoS mitigation
                                        platform, based on an exclusive
                                        blend of traditional and innovative
                                        methods and tools. No time wasted
                                        and invisible, so your users won't
                                        notice an attack: no CAPTCHAs, ever.
                                    </p>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="md:pb-10 md:col-span-2">
                        <div
                                class="m-5 p-5 pl-5 md:pl-14 rounded-lg shadow-xl bg-fuchsia-500 bg-cover h-full bg-gradient-to-b from-fuchsia-700 to-fuchsia-500 hover:shadow-blue-500 duration-500"
                        >
                            <div
                                    class="bg-[url('/analytics.svg')] bg-no-repeat bg-left-top h-full hover:bg-right-bottom duration-500"
                            >
                                <div class="flex md:w-full justify-between">
                                    <p
                                            class="text-3xl md:text-4xl text-slate-200 justify-start font-krona"
                                    >
                                        The visibility you've always wanted
                                        with Analytics by Example
                                    </p>
                                    <p class="justify-end text-white">↗</p>
                                </div>
                                <p
                                        class="text-slate-50 text-xl md:text-2xl font-mona"
                                >
                                    See what's actually going on: measure
                                    requests from head to toe and inspect
                                    every attribute, from threat score to
                                    end-to-end latency.
                                </p>
                            </div>
                        </div>
                    </div>
                    <div class="md:pb-10">
                        <div
                                class="m-5 p-5 pl-5 md:pl-14 rounded-lg bg-blue-500 bg-cover h-full -z-20 shadow-xl bg-gradient-to-b from-blue-700 to-blue-500 hover:shadow-fuchsia-500 duration-500"
                        >
                            <div
                                    class="bg-[url('/page_rules.svg')] bg-no-repeat bg-right-bottom h-full hover:bg-left-top duration-500"
                            >
                                <div class="z-10">
                                    <div
                                            class="flex md:w-full justify-between"
                                    >
                                        <p
                                                class="text-3xl md:text-4xl text-slate-200 justify-start font-krona"
                                        >
                                            Customize everything.
                                        </p>
                                        <p class="justify-end text-white">
                                            ↗
                                        </p>
                                    </div>
                                    <p
                                            class="text-slate-50 text-xl md:text-2xl font-mona"
                                    >
                                        Page rules so powerful, you can make
                                        your own anycast. Our edge is yours.
                                    </p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            <div class="flex flex-row px-10 pt-10 justify-center">
                <div class="">
                    <p
                            class="text-slate-100 text-4xl md:text-5xl font-krona"
                    >
                        Includes all the important content delivery features
                        & beyond
                    </p>
                    <p class="text-slate-200 text-2xl">
                        From flexible caching and image optimization to
                        smart load balancing, we've got your back.
                    </p>
                </div>
                <div class="w-1/2 flex pt-5 md:pt-0">
                    <a
                            href="/product/cdn"
                            class="m-auto text-fuchsia-600 hover:underline hover:text-fuchsia-900 text-2xl md:text-5xl"
                    >
                        explore CDN ↗
                    </a>
                </div>
            </div>
        </div>
    </div>
        </div>
    </div>
</div>