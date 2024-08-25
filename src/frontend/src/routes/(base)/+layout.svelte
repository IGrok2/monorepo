<script>
    import Navbar from "$lib/Navbar.svelte";
    import Footer from "$lib/Footer.svelte";
    import NonlandFooter from "$lib/NonlandFooter.svelte";
</script>

<html class="bg-gray-900">
    <div class="bg-background">
        <section class="bg-gradient-to-tr from-zinc-900 to-90% to-fuchsia-900 relative text-slate-300 font-mona">
            <slot />

            <NonlandFooter />

        </section>
    </div>
</html>
