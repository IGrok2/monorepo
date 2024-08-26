<script>
    export let binder;

    export let placeholder;

    export let required = true;

    export let min = 0;
    export let max = 1000;
</script>

{#if required}
    <input bind:value={binder} minlength={min} maxlength={max} type="text" class="mt-1 focus:ring-indigo-500 focus:border-indigo-500 block w-full shadow-sm sm:text-sm border-fuchsia-500 rounded-md" placeholder="{placeholder}" required>
{:else}
    <input bind:value={binder} minlength={min} maxlength={max} type="text" class="mt-1 focus:ring-indigo-500 focus:border-indigo-500 block w-full shadow-sm sm:text-sm border-fuchsia-500 rounded-md" placeholder="{placeholder}">
{/if}
