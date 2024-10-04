<script>
    /** @type {import('./$types').LayoutData} */
    export let data;

    $: sections = data.sections.data;
</script>

<main
    class="relative flex justify-center mx-auto max-w-8xl"
>
    <label
        for="navigation"
        class="fixed bottom-0 left-0 z-50 flex items-center justify-center w-12 h-12 mb-4 ml-4 border rounded-full shadow-lg cursor-pointer text-white border-white lg:hidden transition duration-200 ease-in-out active:scale-95"
    >
        <svg
            xmlns="http://www.w3.org/2000/svg"
            class="w-6 h-6"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            stroke-width="2"
        >
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M4 8h16M4 16h16"
            />
        </svg>
    </label>

    <input
        type="checkbox"
        name="navigation"
        id="navigation"
        class="hidden peer"
    />
    <div
        class="fixed top-[3.5rem] h-screen shadow-xl px-4 left-0 hidden bg-gray-900 peer-checked:block sm:px-2 lg:px-8 xl:px-12 lg:bg-transparent lg:relative lg:top-0 lg:h-auto lg:px-0 lg:block lg:flex-none lg:shadow-none"
    >
        <div class="absolute inset-y-0 right-0 w-full lg:w-[50vw]" />

        <nav
            class="sticky top-[4.5rem] w-64 pr-8 text-base lg:text-sm xl:w-72 xl:pr-16 text-white"
        >
            <ul
                role="list"
                class="-ml-0.5 h-[calc(100vh-4.5rem)] overflow-y-auto py-7 pl-0.5 space-y-8"
            >
                {#each sections as section}
                    <li>
                        {#if section.attributes.page.data}
                            <a href="/docs/{section.attributes.slug}">
                                <h3
                                    class="font-semibold tracking-tight text-gray-100 hover:text-white"
                                >
                                    {section.attributes.name}
                                </h3>
                            </a>
                        {:else}
                            <h3
                                class="font-semibold tracking-tight text-gray-100"
                            >
                                {section.attributes.name}
                            </h3>
                        {/if}

                        <ul role="list" class="pl-3 mt-3 space-y-2">
                            {#if section.attributes.docs.data.length > 0}
                                {#each section.attributes.docs.data as doc}
                                    <li>
                                        <a
                                            href="/docs/{section.attributes
                                                .slug}/{doc.attributes.slug}"
                                            class="text-gray-400 hover:text-gray-300"
                                        >
                                            {doc.attributes.title}
                                        </a>
                                    </li>
                                {/each}
                            {/if}
                        </ul>
                    </li>
                {/each}
            </ul>
        </nav>
    </div>

    <div
        class="flex-auto max-w-2xl min-w-0 px-4 py-10 lg:max-w-none lg:pr-0 lg:pl-8 xl:px-16"
    >
        <slot />
    </div>
</main>
