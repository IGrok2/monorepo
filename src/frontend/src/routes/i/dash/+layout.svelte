<script>
    import DashNav from "$lib/DashNav.svelte";
    import NamespaceDashNav from "$lib/NamespaceDashNav.svelte";
    import NonlandFooter from "$lib/NonlandFooter.svelte";
</script>

<!--<DashNav /> -->
<NamespaceDashNav />

<slot />

<NonlandFooter />
