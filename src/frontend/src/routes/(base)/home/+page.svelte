<script>
    import Navbar from "$lib/Navbar.svelte";

    let cdn_selected = true;
</script>

<html class="font-ultrawide bg-[url('/cubes.gif')]">
<div class="overflow-y-auto relative h-screen snap-y snap-mandatory pb-14 bg-gray-950 bg-opacity-80">
    <Navbar />
    <section class="snap-start h-screen w-screen">
        <div class="p-20">
            <!-- the header -->
            <div class="py-10">
                <p
                        class="text-slate-400 container mx-auto lg:text-7xl text-5xl md:pr-20 md:pt-10 pt-10 md:p-5"
                >
                    <!-- let the creative light shine through, build fearlessly, paint your own online canvas -->
                    <a
                            class="font-ultrawide bg-clip-text text-transparent bg-gradient-to-br from-slate-400 from-5% to-95% to-slate-100"
                    >Packetware is the cybersecurity cloud for devs.</a>
                    Secure, optimize, deliver, and compute, fearlessly.
                    <br />
                </p>
            </div>
        </div>
    </section>

    <section class="snap-start h-screen w-screen">
        <!-- the menu options -->
        <div class="flex justify-center text-2xl">
            {#if cdn_selected}
                <div class="text-slate-200 text-center m-6 p-6 rounded-lg border-2 border-dashed bg-fuchsia-500 bg-opacity-80">
                    <p class="p-10">
                        Application Security & Performance
                    </p>
                </div>
            {:else}
                <div class="text-slate-200 text-center m-6 p-6 rounded-lg border-2 border-dashed">
                    <p class="p-10">
                        Application Security & Performance
                    </p>
                </div>
            {/if}

            <div class="text-slate-200 text-center m-6 p-6 rounded-lg border-2 border-dashed">
                <p class="p-10">
                    Serverless Compute Platform
                </p>
            </div>
        </div>
    </section>
    </div>
</html>
