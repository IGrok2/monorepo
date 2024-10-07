<script>
    import { page } from '$app/stores';
</script>

{#if $page.status === 404}
    <body class="bg-gradient-to-br from-blue-500 via-gray-900 to-fuchsia-500 h-screen">
    <div>
        <div class="backdrop-blur-sm">
            <div class="flex flex-col items-center justify-center min-h-screen py-2 text-slate-200 text-center backdrop-blur-sm">
                <div class="flex flex-col items-center justify-center px-32 lg:px-64">
                    <!-- <h2 class="text-xl font-bold text-slate-400 bod">hello there, intrepid digital explorer. it seems you've stumbled upon the virtual equivalent of a "{$page.error.message}" sign ...</h2> -->
                    <h1 class="text-6xl font-bold text-slate-50 font-ultrawide">{$page.status}: {$page.error.message}</h1>
                    <!-- <h2 class="text-5xl balls text-slate-100">(code 403)</h2> -->
                    <h1 class="text-4xl text-slate-200 balls">This page couldn't be found (code {$page.status}). 🚫</h1>
                </div>
            </div>
        </div>
        <div class="sticky bottom-0 border-amber-500 border-dotted text-center text-slate-300 bod">
            <p class="py-4">take me home, <a href="https://packetware.net" class="underline decoration-fuchsia-500">packetware</a></p>
        </div>
    </div>
    </body>
{:else}
    <body class="bg-gradient-to-br from-blue-500 via-gray-900 to-fuchsia-500 h-screen">
    <div>
        <div class="backdrop-blur-sm">
            <div class="flex flex-col items-center justify-center min-h-screen py-2 text-slate-200 text-center backdrop-blur-sm">
                <div class="flex flex-col items-center justify-center px-32 lg:px-64">
                    <h1 class="text-6xl font-bold text-slate-50 font-ultrawide">{$page.status}: {$page.error.message}</h1>
                </div>
            </div>
        </div>
        <div class="sticky bottom-0 border-amber-500 border-dotted text-center text-slate-300 bod">
            <p class="py-4">take me home, <a href="https://packetware.net" class="underline decoration-fuchsia-500">packetware</a></p>
        </div>
    </div>
    </body>
{/if}
