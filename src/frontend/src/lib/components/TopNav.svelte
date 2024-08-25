<script>
  import { page } from "$app/stores";

  export let items = [
    {
      name: "Overview",
      link: `/i/dash/project/${$page.params.project}/apps/${$page.params.id}`,
    },
    {
      name: "Deployments",
      link: `/i/dash/project/${$page.params.project}/apps/${$page.params.id}/deployments`,
    },
    {
      name: "Logs",
      link: `/i/dash/project/${$page.params.project}/apps/${$page.params.id}/logs`,
    },
    {
      name: "Analytics",
      link: `/i/dash/project/${$page.params.project}/apps/${$page.params.id}/analytics`,
    },
    {
      name: "Settings",
      link: `/i/dash/project/${$page.params.project}/apps/${$page.params.id}/settings`,
    },
  ];

  console.log($page);
</script>

<div class="relative border-b border-gray-200 pb-5 sm:pb-0">
  <div class="mt-1">
    <!-- Dropdown menu on small screens -->
    <div class="sm:hidden">
      <label for="current-item" class="sr-only">Select a item</label>
      <select
        id="current-item"
        name="current-item"
        class="block w-full rounded-md border-0 py-1.5 pl-3 pr-10 ring-1 ring-inset ring-gray-300 focus:ring-2 focus:ring-inset focus:ring-fuchsia-600"
      >
        {#each items as item}
          {#if item.link === $page.url.pathname}
            <option selected>{item.name}</option>
          {:else}
            <option>{item.name}</option>
          {/if}
        {/each}
      </select>
    </div>
    <!-- items at small breakpoint and up -->
    <div class="hidden sm:block">
      <nav class="-mb-px ml-4 flex space-x-8">
        <!-- Current: "border-fuchsia-500 text-primary", Default: "border-transparent text-primary-muted hover:border-secondary hover:text-primary" -->
        {#each items as item}
          {#if item.link === $page.url.pathname}
            <a
              href={item.link}
              class="border-fuchsia-500 text-primary whitespace-nowrap border-b-2 px-1 pb-2 text-sm font-medium"
              aria-current="page">{item.name}</a
            >
          {:else}
            <a
              href={item.link}
              class="border-transparent text-primary-muted hover:border-secondary hover:text-primary whitespace-nowrap border-b-2 px-1 pb-2 text-sm font-medium"
              >{item.name}</a
            >
          {/if}
        {/each}
      </nav>
    </div>
  </div>
</div>
