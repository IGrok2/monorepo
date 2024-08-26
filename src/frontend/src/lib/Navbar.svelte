<script>
    import { fade, slide, draw, fly } from "svelte/transition";
    import { onMount } from "svelte";
    import { getCookie } from "$lib/utils/auth";

    import { PUBLIC_API } from "$env/static/public";

    function priceAlert() {
        alert(
            "Pricing is void during the beta, however, approval is required to become a tester",
        );
    }

    function docAlert() {
        alert(
            "Documentation is in the works, please join our Discord (discord.gg/cdn) or email hi@packetware.net",
        );
    }

    let navbar_shown = false;
    let name;

    let scrollY = 0;

    onMount(async () => {
        // does the jwt cookie exist?
        if (
            document.cookie
                .split(";")
                .some((item) => item.trim().startsWith("jwt="))
        ) {
            // before redirecting, make sure the cookie is valid by fetching a GET to /@/me
            const token = getCookie("jwt");

            await fetch(`${PUBLIC_API}/@/me`, {
                method: "GET",
                headers: new Headers({
                    "content-type": "application/json",
                    Authorization: token,
                }),
            })
                .then(async (resp) => {
                    let response = await resp.json();

                    if (resp.status !== 200 || !response.success) {
                        // remove the cookie
                        //document.cookie =
                            //"jwt=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;";
                    } else {
                        name = response.user.name;
                    }
                })
                .catch(() => {
                    //document.cookie =
                        //"jwt=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;";
                });
        }
    });
</script>

<svelte:window bind:scrollY />

{#if navbar_shown}
    <div>
        <div
            transition:slide
            class="mb-10 bg-gradient-to-br from-blue-500 via-gray-950 to-fuchsia-500 rounded-b-xl"
        >
            <!-- <div class="bg-[url('/black_blue.svg')] font-bold h-full"> -->
            <div class="bg-[url('/whitegrid.svg')] bg-cover">
                <nav
                    class="sticky top-0 mx-auto flex items-center justify-between p-6 lg:px-8 backdrop-blur-md opacity-90 w-screen z-50"
                    aria-label="Global"
                >
                    <div
                        on:click={() => (document.location = "/")}
                        class="cursor-pointer flex lg:flex-1 w-screen h-14"
                    >
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 620.27 252.21"
                            ><defs
                                ><style>
                                    .cls-1 {
                                        fill: #f4fafc;
                                    }
                                    .cls-2 {
                                        fill: #e500ff;
                                    }
                                </style></defs
                            ><g id="Layer_2" data-name="Layer 2"
                                ><g id="Layer_1-2" data-name="Layer 1"
                                    ><path
                                        class="cls-1"
                                        d="M116.32,117.84H41.52a6.7,6.7,0,0,1-6.35-4.58L22.75,76a6.69,6.69,0,0,1,6.35-8.81h74.79a6.7,6.7,0,0,1,6.36,4.58L122.67,109A6.7,6.7,0,0,1,116.32,117.84Z"
                                    /><path
                                        class="cls-1"
                                        d="M138.72,185H63.92a6.7,6.7,0,0,1-6.35-4.58L45.15,143.19a6.7,6.7,0,0,1,6.35-8.81h74.8a6.69,6.69,0,0,1,6.35,4.58l12.42,37.25A6.7,6.7,0,0,1,138.72,185Z"
                                    /><path
                                        class="cls-1"
                                        d="M93.92,50.65H19.12a6.7,6.7,0,0,1-6.35-4.58L.35,8.81A6.69,6.69,0,0,1,6.7,0H81.49a6.68,6.68,0,0,1,6.35,4.58l12.43,37.25A6.7,6.7,0,0,1,93.92,50.65Z"
                                    /><path
                                        class="cls-1"
                                        d="M199.53,50.65H126.28a6.7,6.7,0,0,1-6.35-4.58L107.51,8.81A6.69,6.69,0,0,1,113.86,0h73.25a6.69,6.69,0,0,1,6.35,4.58l12.42,37.25A6.7,6.7,0,0,1,199.53,50.65Z"
                                    /><path
                                        class="cls-2"
                                        d="M333.59,50.65h-74.8a6.7,6.7,0,0,1-6.35-4.58L240,8.81A6.7,6.7,0,0,1,246.37,0h74.8a6.69,6.69,0,0,1,6.35,4.58l12.42,37.25A6.7,6.7,0,0,1,333.59,50.65Z"
                                    /><path
                                        class="cls-2"
                                        d="M546.37,50.65h-74.8a6.7,6.7,0,0,1-6.35-4.58L452.8,8.81A6.69,6.69,0,0,1,459.15,0h74.79a6.7,6.7,0,0,1,6.36,4.58l12.42,37.25A6.7,6.7,0,0,1,546.37,50.65Z"
                                    /><path
                                        class="cls-2"
                                        d="M591.17,185h-74.8a6.7,6.7,0,0,1-6.35-4.58L497.6,143.19a6.69,6.69,0,0,1,6.35-8.81h74.8A6.69,6.69,0,0,1,585.1,139l12.42,37.25A6.7,6.7,0,0,1,591.17,185Z"
                                    /><path
                                        class="cls-2"
                                        d="M568.77,117.84H494a6.7,6.7,0,0,1-6.35-4.58L475.2,76a6.69,6.69,0,0,1,6.35-8.81h74.8a6.7,6.7,0,0,1,6.35,4.58L575.12,109A6.7,6.7,0,0,1,568.77,117.84Z"
                                    /><path
                                        class="cls-1"
                                        d="M221.93,117.84H148.68a6.7,6.7,0,0,1-6.35-4.58L129.91,76a6.69,6.69,0,0,1,6.35-8.81h73.25a6.69,6.69,0,0,1,6.35,4.58L228.28,109A6.69,6.69,0,0,1,221.93,117.84Z"
                                    /><path
                                        class="cls-2"
                                        d="M356,117.84h-74.8a6.7,6.7,0,0,1-6.35-4.58L262.42,76a6.69,6.69,0,0,1,6.35-8.81h74.8a6.69,6.69,0,0,1,6.35,4.58L362.34,109A6.69,6.69,0,0,1,356,117.84Z"
                                    /><path
                                        class="cls-1"
                                        d="M161.12,252.21H86.32A6.69,6.69,0,0,1,80,247.64L67.55,210.38a6.69,6.69,0,0,1,6.35-8.81h74.8a6.69,6.69,0,0,1,6.35,4.58l12.42,37.25A6.69,6.69,0,0,1,161.12,252.21Z"
                                    /><path
                                        class="cls-2"
                                        d="M378.39,185h-74.8a6.7,6.7,0,0,1-6.35-4.58l-12.42-37.26a6.7,6.7,0,0,1,6.35-8.81H366a6.69,6.69,0,0,1,6.35,4.58l12.42,37.25A6.7,6.7,0,0,1,378.39,185Z"
                                    /><path
                                        class="cls-2"
                                        d="M400.79,252.21H326a6.69,6.69,0,0,1-6.35-4.57l-12.42-37.26a6.7,6.7,0,0,1,6.35-8.81h74.8a6.69,6.69,0,0,1,6.35,4.58l12.42,37.25A6.69,6.69,0,0,1,400.79,252.21Z"
                                    /><path
                                        class="cls-2"
                                        d="M484,185H410.75a6.7,6.7,0,0,1-6.35-4.58L392,143.19a6.7,6.7,0,0,1,6.35-8.81h73.25a6.69,6.69,0,0,1,6.35,4.58l12.42,37.25A6.7,6.7,0,0,1,484,185Z"
                                    /><path
                                        class="cls-2"
                                        d="M506.4,252.21H433.15a6.69,6.69,0,0,1-6.35-4.57l-12.42-37.26a6.7,6.7,0,0,1,6.35-8.81H494a6.69,6.69,0,0,1,6.35,4.58l12.42,37.25A6.69,6.69,0,0,1,506.4,252.21Z"
                                    /><path
                                        class="cls-2"
                                        d="M613.57,252.21h-74.8a6.69,6.69,0,0,1-6.35-4.57L520,210.38a6.69,6.69,0,0,1,6.35-8.81h74.8a6.69,6.69,0,0,1,6.35,4.58l12.42,37.25A6.69,6.69,0,0,1,613.57,252.21Z"
                                    /></g
                                ></g
                            ></svg
                        >
                    </div>
                    <!--
        <div class="hidden lg:flex lg:gap-x-12">
            <a href="/#product" class="text-sm font-semibold leading-6 text-white">Product</a>
            <a href="/co/about" class="text-sm font-semibold leading-6 text-white">About</a>
            <a on:click={priceAlert} href="" class="text-sm font-semibold leading-6 text-white">Pricing</a>
            <a on:click={docAlert} href="" class="text-sm font-semibold leading-6 text-white">Docs</a>
        </div>
        -->
                    <div
                        class="lg:flex lg:flex-1 lg:justify-end flex justify-between items-center"
                    >
                        <a
                            on:click={() => {
                                navbar_shown = false;
                            }}
                            class="text-sm cursor-pointer font-semibold leading-6 text-white pr-10"
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
                                    in:draw
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    d="M6 18L18 6M6 6l12 12"
                                />
                            </svg>
                        </a>
                        <a
                            href={name ? `/i/dash/domains` : `/i/auth/login`}
                            class="text-sm text-white"
                        >
                            {name ? `Hey, ${name}` : `Log in`}
                            <span aria-hidden="true">&rarr;</span>
                        </a>
                    </div>
                </nav>
                <div class="pt-10">
                    <div class="rounded-2xl mt-10">
                        <div
                            class="mt-px font-display text-5xl font-medium tracking-tight text-white"
                        >
                            <div class="pt-0">
                                <div class="grid grid-cols-1 sm:grid-cols-2">
                                    <a
                                        href="/products"
                                        class="border-b-4 border-l-4 border-t-2 border-r-2 group relative isolate px-6 py-4 even:mt-px sm:mx-0 sm:px-0 sm:py-8 sm:odd:pr-16 sm:even:mt-0 sm:pl-16"
                                        >Products</a
                                    >
                                    <a
                                        href="/blog"
                                        class="border-b-4 border-l-4 border-t-2 border-r-2 group relative isolate px-6 py-4 even:mt-px sm:mx-0 sm:px-0 sm:py-8 sm:odd:pr-16 sm:even:mt-0 sm:pl-16"
                                        >Blog</a
                                    >
                                    <a
                                        href="/docs"
                                        class="border-b-4 border-r-4 border-t-2 border-l-2 group relative isolate px-6 py-4 even:mt-px sm:mx-0 sm:px-0 sm:py-8 sm:odd:pr-16 sm:even:mt-0 sm:pl-16"
                                        >Docs</a
                                    >
                                    <a
                                        href="/co/about"
                                        class="border-b-4 border-l-4 border-t-2 border-r-2 group relative isolate px-6 py-4 even:mt-px sm:mx-0 sm:px-0 sm:py-8 sm:odd:pr-16 sm:even:mt-0 sm:pl-16"
                                        >About</a
                                    >
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
                <!--
            <div class="mt-20 bottom-0 pb-14 px-20 lg:flex lg:flex-1 justify-between">
                <div class="font-mona text-transparent bg-clip-text bg-gradient-to-r from-red-500 to-fuchsia-800 text-5xl sm:max-w-sm tracking-tight w-1/2">
                    <p>
                        Whole Lotta Heart, Corp.
                    </p>
                    <p class="text-sm">Proudly built with ❤️ in New Canaan, Connecticut</p>
                    <p class="text-sm text-blue-800"><br>An Edward Coristine product</p>
                </div>
                <div class="pl-10">
                    <h3 class="text-xl font-semibold leading-6 text-white">Company</h3>
                    <ul role="list" class="mt-6 space-y-2">
                        <li>
                            <a href="#" class="leading-6 text-gray-200 hover:text-white">About</a>
                        </li>
                        <li>
                            <a href="#" class="leading-6 text-gray-200 hover:text-white">Blog</a>
                        </li>
                        <li>
                            <a href="#" class="leading-6 text-gray-200 hover:text-white">Partners</a>
                        </li>
                    </ul>
                </div>
                <div class="pl-24">
                    <h3 class="text-xl font-semibold leading-6 text-white">Legal</h3>
                    <ul role="list" class="mt-6 space-y-2">
                        <li>
                            <a href="#" class="leading-6 text-gray-200 hover:text-white">Privacy</a>
                        </li>
                        <li>
                            <a href="#" class="leading-6 text-gray-200 hover:text-white">Terms</a>
                        </li>
                        <li>
                            <a href="#" class="leading-6 text-gray-200 hover:text-white">Acceptable use</a>
                        </li>
                    </ul>
                </div>
            </div>
            -->
            </div>
        </div>
    </div>
{:else}
    <div class="sticky top-0 z-50">
        <nav
            class="sticky top-0 mx-auto flex items-center justify-between p-6 lg:px-8 backdrop-blur-md opacity-90 w-screen z-50"
            aria-label="Global"
        >
            <div
                on:click={() => (document.location = "/")}
                class="cursor-pointer flex lg:flex-1 w-screen h-12"
            >
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 620.27 252.21"
                    ><defs
                        ><style>
                            .cls-1 {
                                fill: #f4fafc;
                            }
                            .cls-2 {
                                fill: #e500ff;
                            }
                        </style></defs
                    ><g id="Layer_2" data-name="Layer 2"
                        ><g id="Layer_1-2" data-name="Layer 1"
                            ><path
                                class="cls-1"
                                d="M116.32,117.84H41.52a6.7,6.7,0,0,1-6.35-4.58L22.75,76a6.69,6.69,0,0,1,6.35-8.81h74.79a6.7,6.7,0,0,1,6.36,4.58L122.67,109A6.7,6.7,0,0,1,116.32,117.84Z"
                            /><path
                                class="cls-1"
                                d="M138.72,185H63.92a6.7,6.7,0,0,1-6.35-4.58L45.15,143.19a6.7,6.7,0,0,1,6.35-8.81h74.8a6.69,6.69,0,0,1,6.35,4.58l12.42,37.25A6.7,6.7,0,0,1,138.72,185Z"
                            /><path
                                class="cls-1"
                                d="M93.92,50.65H19.12a6.7,6.7,0,0,1-6.35-4.58L.35,8.81A6.69,6.69,0,0,1,6.7,0H81.49a6.68,6.68,0,0,1,6.35,4.58l12.43,37.25A6.7,6.7,0,0,1,93.92,50.65Z"
                            /><path
                                class="cls-1"
                                d="M199.53,50.65H126.28a6.7,6.7,0,0,1-6.35-4.58L107.51,8.81A6.69,6.69,0,0,1,113.86,0h73.25a6.69,6.69,0,0,1,6.35,4.58l12.42,37.25A6.7,6.7,0,0,1,199.53,50.65Z"
                            /><path
                                class="cls-2"
                                d="M333.59,50.65h-74.8a6.7,6.7,0,0,1-6.35-4.58L240,8.81A6.7,6.7,0,0,1,246.37,0h74.8a6.69,6.69,0,0,1,6.35,4.58l12.42,37.25A6.7,6.7,0,0,1,333.59,50.65Z"
                            /><path
                                class="cls-2"
                                d="M546.37,50.65h-74.8a6.7,6.7,0,0,1-6.35-4.58L452.8,8.81A6.69,6.69,0,0,1,459.15,0h74.79a6.7,6.7,0,0,1,6.36,4.58l12.42,37.25A6.7,6.7,0,0,1,546.37,50.65Z"
                            /><path
                                class="cls-2"
                                d="M591.17,185h-74.8a6.7,6.7,0,0,1-6.35-4.58L497.6,143.19a6.69,6.69,0,0,1,6.35-8.81h74.8A6.69,6.69,0,0,1,585.1,139l12.42,37.25A6.7,6.7,0,0,1,591.17,185Z"
                            /><path
                                class="cls-2"
                                d="M568.77,117.84H494a6.7,6.7,0,0,1-6.35-4.58L475.2,76a6.69,6.69,0,0,1,6.35-8.81h74.8a6.7,6.7,0,0,1,6.35,4.58L575.12,109A6.7,6.7,0,0,1,568.77,117.84Z"
                            /><path
                                class="cls-1"
                                d="M221.93,117.84H148.68a6.7,6.7,0,0,1-6.35-4.58L129.91,76a6.69,6.69,0,0,1,6.35-8.81h73.25a6.69,6.69,0,0,1,6.35,4.58L228.28,109A6.69,6.69,0,0,1,221.93,117.84Z"
                            /><path
                                class="cls-2"
                                d="M356,117.84h-74.8a6.7,6.7,0,0,1-6.35-4.58L262.42,76a6.69,6.69,0,0,1,6.35-8.81h74.8a6.69,6.69,0,0,1,6.35,4.58L362.34,109A6.69,6.69,0,0,1,356,117.84Z"
                            /><path
                                class="cls-1"
                                d="M161.12,252.21H86.32A6.69,6.69,0,0,1,80,247.64L67.55,210.38a6.69,6.69,0,0,1,6.35-8.81h74.8a6.69,6.69,0,0,1,6.35,4.58l12.42,37.25A6.69,6.69,0,0,1,161.12,252.21Z"
                            /><path
                                class="cls-2"
                                d="M378.39,185h-74.8a6.7,6.7,0,0,1-6.35-4.58l-12.42-37.26a6.7,6.7,0,0,1,6.35-8.81H366a6.69,6.69,0,0,1,6.35,4.58l12.42,37.25A6.7,6.7,0,0,1,378.39,185Z"
                            /><path
                                class="cls-2"
                                d="M400.79,252.21H326a6.69,6.69,0,0,1-6.35-4.57l-12.42-37.26a6.7,6.7,0,0,1,6.35-8.81h74.8a6.69,6.69,0,0,1,6.35,4.58l12.42,37.25A6.69,6.69,0,0,1,400.79,252.21Z"
                            /><path
                                class="cls-2"
                                d="M484,185H410.75a6.7,6.7,0,0,1-6.35-4.58L392,143.19a6.7,6.7,0,0,1,6.35-8.81h73.25a6.69,6.69,0,0,1,6.35,4.58l12.42,37.25A6.7,6.7,0,0,1,484,185Z"
                            /><path
                                class="cls-2"
                                d="M506.4,252.21H433.15a6.69,6.69,0,0,1-6.35-4.57l-12.42-37.26a6.7,6.7,0,0,1,6.35-8.81H494a6.69,6.69,0,0,1,6.35,4.58l12.42,37.25A6.69,6.69,0,0,1,506.4,252.21Z"
                            /><path
                                class="cls-2"
                                d="M613.57,252.21h-74.8a6.69,6.69,0,0,1-6.35-4.57L520,210.38a6.69,6.69,0,0,1,6.35-8.81h74.8a6.69,6.69,0,0,1,6.35,4.58l12.42,37.25A6.69,6.69,0,0,1,613.57,252.21Z"
                            /></g
                        ></g
                    ></svg
                >
            </div>
            <!--
                <div class="hidden lg:flex lg:gap-x-12">
                    <a href="/#product" class="text-sm font-semibold leading-6 text-white">Product</a>
                    <a href="/co/about" class="text-sm font-semibold leading-6 text-white">About</a>
                    <a on:click={priceAlert} href="" class="text-sm font-semibold leading-6 text-white">Pricing</a>
                    <a on:click={docAlert} href="" class="text-sm font-semibold leading-6 text-white">Docs</a>
                </div>
                -->
            <div class="flex flex-1 justify-end">
                <a
                    on:click={() => {
                        navbar_shown = true;
                        scrollY = 0;
                    }}
                    class="text-sm cursor-pointer font-semibold leading-6 text-white pr-10"
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
                            in:draw
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"
                        />
                    </svg>
                </a>
                <a
                    href={name ? `/i/dash/domains` : `/i/auth/login`}
                    class="hidden md:block text-sm font-semibold leading-6 text-white pr-2"
                >
                    {name ? `Hey, ${name}` : `Log in`}
                    <span aria-hidden="true">&rarr;</span>
                </a>
            </div>
        </nav>
    </div>
{/if}
