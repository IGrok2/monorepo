<script>
import Navbar from "$lib/Navbar.svelte";
import Footer from "$lib/Footer.svelte";
import NonlandFooter from "$lib/NonlandFooter.svelte";
</script>

<Navbar />
<div class="h-screen">
    <slot />
</div>

<NonlandFooter />
