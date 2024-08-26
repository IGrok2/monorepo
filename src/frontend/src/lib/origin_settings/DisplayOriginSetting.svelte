<script>
    export let setting;
</script>

<li class="font-light list-none">
    <p class="text-sm font-bold uppercase text-slate-800">basic info</p>
    <p>SSL:
        {#if setting.ssl}
        <a class="text-green-700">enabled</a>
    {:else}
        <a class="text-red-500">disabled</a>
        {/if}
    </p>
    <p>HTTP/2:
        {#if setting.http2}
            <a class="text-green-700">enabled</a>
        {:else}
            <a class="text-red-500">disabled</a>
        {/if}
    </p>
    {setting.ip_data ? `<p>IP data forwarded</p>` : ``}
    <p>Origin timeout: {setting.timeout} seconds </p>
    <p class="text-sm font-bold uppercase text-slate-800">origins</p>
    <ol class="pl-6">
        {#each setting.origins as origin}
            <p>Host: {origin.url}</p>
            <p class="pl-4">Weight (for LB): {origin.weight}</p>
        {/each}
    </ol>
</li>
