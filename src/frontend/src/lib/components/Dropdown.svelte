<script>
    import { ChevronDown } from "lucide-svelte";

    export let tabs = ["Tab 1", "Tab 2", "Tab 3"];
    export let activeTab = "Tab 1";

    let changeTab = false;
</script>

<div>
    <button
        type="button"
        class="border-fuchsia-500 inline-flex w-full justify-center gap-x-1.5 rounded-md bg-background px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-indigo-500 hover:bg-gray-50"
        id="menu-button"
        aria-expanded="true"
        aria-haspopup="true"
        on:click={() => (changeTab = !changeTab)}
    >
        {activeTab}
        <ChevronDown />
    </button>
</div>
{#if changeTab}
    <div
        class="z-10 mt-2 origin-top-right rounded-md bg-background shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none"
        role="menu"
        aria-orientation="vertical"
        aria-labelledby="menu-button"
        tabindex="-1"
    >
        <div class="py-1" role="none">
            <!-- Active: "bg-gray-100 text-gray-900", Not Active: "text-gray-700" -->
            {#each tabs as tab}
                <button
                    class="flex w-full text-gray-700 block px-4 py-2 text-sm hover:bg-gray-200"
                    role="menuitem"
                    tabindex="-1"
                    id="menu-item-0"
                    on:click={() => {
                        activeTab = tab;
                        changeTab = false;
                    }}
                >
                    {tab}
                </button>
            {/each}
        </div>
    </div>
{/if}
