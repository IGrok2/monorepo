<script>
    import "../app.css";
    import { fade } from "svelte/transition";
    import { Toaster } from "$lib/components/ui/sonner";
</script>

<svelte:head>
    <title>Packetware</title>
    <meta
        name="description"
        content="Imagine if you never had to worry about cybersecurity again. Packetware is the best platform to cohesively secure, optimize, and speed up, fearlessly at scale."
    />
</svelte:head>

<Toaster position="bottom-right" />

<slot />
