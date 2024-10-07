<script>
    export let switch_variable;
</script>

{#if !switch_variable}
    <div class="mt-5 sm:ml-6 sm:mt-0 sm:flex sm:flex-shrink-0 sm:items-center">
        <!-- Enabled: "bg-indigo-600", Not Enabled: "bg-gray-200" -->
        <button on:click={async () => switch_variable = true} type="button" class="bg-gray-400 relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-600 focus:ring-offset-2" role="switch" aria-checked="false" aria-labelledby="renew-subscription-label" aria-describedby="renew-subscription-description">
            <!-- Enabled: "translate-x-5", Not Enabled: "translate-x-0" -->
            <span aria-hidden="true" class="translate-x-0 inline-block h-5 w-5 transform rounded-full bg-background shadow ring-0 transition duration-200 ease-in-out"></span>
        </button>
    </div>
{:else}
    <div class="mt-5 sm:ml-6 sm:mt-0 sm:flex sm:flex-shrink-0 sm:items-center">
        <!-- Enabled: "bg-indigo-600", Not Enabled: "bg-gray-200" -->
        <button on:click={async () => switch_variable = false} type="button" class="bg-indigo-600 relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-600 focus:ring-offset-2" role="switch" aria-checked="false" aria-labelledby="renew-subscription-label" aria-describedby="renew-subscription-description">
            <!-- Enabled: "translate-x-5", Not Enabled: "translate-x-0" -->
            <span aria-hidden="true" class="translate-x-5 inline-block h-5 w-5 transform rounded-full bg-background shadow ring-0 transition duration-200 ease-in-out"></span>
        </button>
    </div>
{/if}
