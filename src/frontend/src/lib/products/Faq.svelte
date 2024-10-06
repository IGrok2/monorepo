<script>
    import DocsLink from "$lib/base/DocsLink.svelte";
</script>

<div class="mx-auto max-w-7xl px-6 py-16 sm:py-24 lg:px-8">
    <h2 class="text-2xl font-bold leading-10 tracking-tight text-slate-100">Frequently asked questions</h2>
    <p class="mt-6 max-w-2xl text-base leading-7 text-slate-200">
        Have any other questions? Reach out to the <a class="underline" href="mailto:concierge@packetware.net">Concierge</a> or <a class="underline" href="https://discord.gg/qTWxD9Ak85">join the Discord</a>
        to get help from the community.
    </p>
    <div class="mt-20">
        <dl class="space-y-16 sm:grid sm:grid-cols-2 sm:gap-x-6 sm:gap-y-16 sm:space-y-0 lg:grid-cols-3 lg:gap-x-10">
            <div>
                <dt class="text-base font-semibold leading-7 text-slate-200">Why Packetware?</dt>
                <dd class="mt-2 text-base leading-7 text-slate-300">
                    We recognized the glaring inefficiencies and limitations within the mainstream cybersecurity and serverless products.
                    We reached out to CEOs, tried alternative products, but nobody was willing to step up. So we did.
                </dd>
            </div>

            <div>
                <dt class="text-base font-semibold leading-7 text-slate-200">Who owns Packetware?</dt>
                <dd class="mt-2 text-base leading-7 text-slate-300">
                    Packetware is run by techno-loving ambitious individuals who love to solve problems.
                    Based in Connecticut, we're a small team that focuses on excellence and first-principles thinking.
                </dd>
            </div>

            <div>
                <dt class="text-base font-semibold leading-7 text-slate-200">
                    How can I use Packetware?
                </dt>
                <dd class="mt-2 text-base leading-7 text-slate-300">
                    Packetware offers secure-by-default application performance and compute. We're a little different from the rest.
                    We have great <DocsLink /> to help you get started.
                </dd>
            </div>

            <!-- More questions... -->
        </dl>
    </div>
</div>
