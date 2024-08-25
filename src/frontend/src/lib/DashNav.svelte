<script>
    import { slide } from "svelte/transition";
    import { expoInOut } from "svelte/easing";

    import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
    import { Button } from "$lib/components/ui/button";

    import { Sun, Moon, Menu } from "lucide-svelte";

    import Notifications from "$lib/components/Notifications.svelte";

    import { setMode, resetMode } from "mode-watcher";

    import logo from "$lib/assets/Asset 19.png";

    const pages = [
        { label: "Projects", route: "/i/dash" },
        //{ label: "Domains", route: "/i/dash/domains" },
        //{ label: "Apps", route: "/i/dash/apps" },
        { label: "Support & Docs", route: "/docs" },
        { label: "Profile", route: "/i/dash/profile" },
    ];

    let navToggle = false;
</script>

<!--
<nav class="sticky top-0 mx-auto flex items-center justify-between p-6 lg:px-8 backdrop-blur-sm opacity-90 w-screen z-40" aria-label="Global">
    <div on:click={() => document.location = "/"} class="cursor-pointer flex lg:flex-1 w-screen h-14">
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 620.27 252.21"><defs><style>.cls-1{fill:#f4fafc;}.cls-2{fill:#e500ff;}</style></defs><g id="Layer_2" data-name="Layer 2"><g id="Layer_1-2" data-name="Layer 1"><path class="cls-1" d="M116.32,117.84H41.52a6.7,6.7,0,0,1-6.35-4.58L22.75,76a6.69,6.69,0,0,1,6.35-8.81h74.79a6.7,6.7,0,0,1,6.36,4.58L122.67,109A6.7,6.7,0,0,1,116.32,117.84Z"/><path class="cls-1" d="M138.72,185H63.92a6.7,6.7,0,0,1-6.35-4.58L45.15,143.19a6.7,6.7,0,0,1,6.35-8.81h74.8a6.69,6.69,0,0,1,6.35,4.58l12.42,37.25A6.7,6.7,0,0,1,138.72,185Z"/><path class="cls-1" d="M93.92,50.65H19.12a6.7,6.7,0,0,1-6.35-4.58L.35,8.81A6.69,6.69,0,0,1,6.7,0H81.49a6.68,6.68,0,0,1,6.35,4.58l12.43,37.25A6.7,6.7,0,0,1,93.92,50.65Z"/><path class="cls-1" d="M199.53,50.65H126.28a6.7,6.7,0,0,1-6.35-4.58L107.51,8.81A6.69,6.69,0,0,1,113.86,0h73.25a6.69,6.69,0,0,1,6.35,4.58l12.42,37.25A6.7,6.7,0,0,1,199.53,50.65Z"/><path class="cls-2" d="M333.59,50.65h-74.8a6.7,6.7,0,0,1-6.35-4.58L240,8.81A6.7,6.7,0,0,1,246.37,0h74.8a6.69,6.69,0,0,1,6.35,4.58l12.42,37.25A6.7,6.7,0,0,1,333.59,50.65Z"/><path class="cls-2" d="M546.37,50.65h-74.8a6.7,6.7,0,0,1-6.35-4.58L452.8,8.81A6.69,6.69,0,0,1,459.15,0h74.79a6.7,6.7,0,0,1,6.36,4.58l12.42,37.25A6.7,6.7,0,0,1,546.37,50.65Z"/><path class="cls-2" d="M591.17,185h-74.8a6.7,6.7,0,0,1-6.35-4.58L497.6,143.19a6.69,6.69,0,0,1,6.35-8.81h74.8A6.69,6.69,0,0,1,585.1,139l12.42,37.25A6.7,6.7,0,0,1,591.17,185Z"/><path class="cls-2" d="M568.77,117.84H494a6.7,6.7,0,0,1-6.35-4.58L475.2,76a6.69,6.69,0,0,1,6.35-8.81h74.8a6.7,6.7,0,0,1,6.35,4.58L575.12,109A6.7,6.7,0,0,1,568.77,117.84Z"/><path class="cls-1" d="M221.93,117.84H148.68a6.7,6.7,0,0,1-6.35-4.58L129.91,76a6.69,6.69,0,0,1,6.35-8.81h73.25a6.69,6.69,0,0,1,6.35,4.58L228.28,109A6.69,6.69,0,0,1,221.93,117.84Z"/><path class="cls-2" d="M356,117.84h-74.8a6.7,6.7,0,0,1-6.35-4.58L262.42,76a6.69,6.69,0,0,1,6.35-8.81h74.8a6.69,6.69,0,0,1,6.35,4.58L362.34,109A6.69,6.69,0,0,1,356,117.84Z"/><path class="cls-1" d="M161.12,252.21H86.32A6.69,6.69,0,0,1,80,247.64L67.55,210.38a6.69,6.69,0,0,1,6.35-8.81h74.8a6.69,6.69,0,0,1,6.35,4.58l12.42,37.25A6.69,6.69,0,0,1,161.12,252.21Z"/><path class="cls-2" d="M378.39,185h-74.8a6.7,6.7,0,0,1-6.35-4.58l-12.42-37.26a6.7,6.7,0,0,1,6.35-8.81H366a6.69,6.69,0,0,1,6.35,4.58l12.42,37.25A6.7,6.7,0,0,1,378.39,185Z"/><path class="cls-2" d="M400.79,252.21H326a6.69,6.69,0,0,1-6.35-4.57l-12.42-37.26a6.7,6.7,0,0,1,6.35-8.81h74.8a6.69,6.69,0,0,1,6.35,4.58l12.42,37.25A6.69,6.69,0,0,1,400.79,252.21Z"/><path class="cls-2" d="M484,185H410.75a6.7,6.7,0,0,1-6.35-4.58L392,143.19a6.7,6.7,0,0,1,6.35-8.81h73.25a6.69,6.69,0,0,1,6.35,4.58l12.42,37.25A6.7,6.7,0,0,1,484,185Z"/><path class="cls-2" d="M506.4,252.21H433.15a6.69,6.69,0,0,1-6.35-4.57l-12.42-37.26a6.7,6.7,0,0,1,6.35-8.81H494a6.69,6.69,0,0,1,6.35,4.58l12.42,37.25A6.69,6.69,0,0,1,506.4,252.21Z"/><path class="cls-2" d="M613.57,252.21h-74.8a6.69,6.69,0,0,1-6.35-4.57L520,210.38a6.69,6.69,0,0,1,6.35-8.81h74.8a6.69,6.69,0,0,1,6.35,4.58l12.42,37.25A6.69,6.69,0,0,1,613.57,252.21Z"/></g></g></svg>
    </div>

    <div class="hidden lg:flex lg:gap-x-12">
        <a href="/i/dash/domains" class="p-3 text-sm font-semibold leading-6 text-white hover:bg-blue-500 hover:opacity-90 duration-150 rounded">Domains</a>
        <a href="/co/about" class="p-3 text-sm font-semibold leading-6 text-white hover:bg-blue-500 hover:opacity-90 duration-150 rounded">Teams</a>
        <a href="/docs" class="p-3 text-sm font-semibold leading-6 text-white hover:bg-blue-500 hover:opacity-90 duration-150 rounded">Support & Docs</a>
        <a href="/i/dash/profile" class="p-3 text-sm font-semibold leading-6 text-white hover:bg-blue-500 hover:opacity-90 duration-150 rounded">Profile</a>
    </div>
    <div class="hidden lg:flex lg:flex-1 lg:justify-end">
        <a on:click={() => {document.cookie = "jwt=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;"; document.location.href = "/i/dash/domains";}} href="/i/auth/login" class="text-sm font-semibold leading-6 text-white">Log out <span aria-hidden="true">&rarr;</span></a>
    </div>
</nav> 
-->

<nav
    class="sticky top-0 z-40 backdrop-blur-sm opacity-90 bg-background bg-opacity-10"
>
    <div class="mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex h-16 items-center justify-between">
            <div class="flex items-center">
                <div class="flex flex-shrink-0 items-center">
                    <img class="h-8 w-18" src={logo} alt="Packetware Logo" />
                </div>
                <div class="hidden md:block">
                    <div class="ml-10 flex items-baseline space-x-4">
                        <!-- Current: "bg-gray-900 text-white", Default: "text-gray-300 hover:bg-gray-700 hover:text-white" -->
                        {#each pages as page}
                            <Button href={page.route} variant="ghost"
                                >{page.label}</Button
                            >
                        {/each}
                    </div>
                </div>
            </div>
            <div class="hidden md:block">
                <div class="ml-4 flex items-center space-x-3 md:ml-6">
                    <DropdownMenu.Root>
                        <DropdownMenu.Trigger asChild let:builder>
                            <Button
                                builders={[builder]}
                                variant="outline"
                                size="icon"
                            >
                                <Sun
                                    class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0"
                                />
                                <Moon
                                    class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100"
                                />
                                <span class="sr-only">Toggle theme</span>
                            </Button>
                        </DropdownMenu.Trigger>
                        <DropdownMenu.Content align="end">
                            <DropdownMenu.Item on:click={() => setMode("light")}
                                >Light</DropdownMenu.Item
                            >
                            <DropdownMenu.Item on:click={() => setMode("dark")}
                                >Dark</DropdownMenu.Item
                            >
                            <DropdownMenu.Item on:click={() => resetMode()}
                                >System</DropdownMenu.Item
                            >
                        </DropdownMenu.Content>
                    </DropdownMenu.Root>
                    <Notifications />
                    <div class="relative">
                        <Button
                            on:click={() => {
                                document.cookie =
                                    "jwt=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;";
                                document.location.href = "/i/dash/domains";
                            }}
                            href="/i/auth/login"
                            variant="outline">Log out</Button
                        >
                    </div>
                </div>
            </div>
            <div class="-mr-2 space-x-2 flex md:hidden">
                <!-- Mobile menu button -->
                <DropdownMenu.Root>
                    <DropdownMenu.Trigger asChild let:builder>
                        <Button
                            builders={[builder]}
                            variant="outline"
                            size="icon"
                        >
                            <Sun
                                class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0"
                            />
                            <Moon
                                class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100"
                            />
                            <span class="sr-only">Toggle theme</span>
                        </Button>
                    </DropdownMenu.Trigger>
                    <DropdownMenu.Content align="end">
                        <DropdownMenu.Item on:click={() => setMode("light")}
                            >Light</DropdownMenu.Item
                        >
                        <DropdownMenu.Item on:click={() => setMode("dark")}
                            >Dark</DropdownMenu.Item
                        >
                        <DropdownMenu.Item on:click={() => resetMode()}
                            >System</DropdownMenu.Item
                        >
                    </DropdownMenu.Content>
                </DropdownMenu.Root>
                <Notifications />
                <DropdownMenu.Root>
                    <DropdownMenu.Trigger asChild let:builder>
                        <Button
                            builders={[builder]}
                            variant="outline"
                            size="icon"
                        >
                            <Menu />
                            <span class="sr-only">Change Dashboard Page</span>
                        </Button>
                    </DropdownMenu.Trigger>
                    <DropdownMenu.Content align="end">
                        {#each pages as page}
                            <DropdownMenu.Item href={page.route}
                                >{page.label}</DropdownMenu.Item
                            >
                        {/each}
                        <DropdownMenu.Item href="/i/auth/logout"
                            >Logout</DropdownMenu.Item
                        >
                    </DropdownMenu.Content>
                </DropdownMenu.Root>
            </div>
        </div>
    </div>
</nav>
