<script>
    import { fade } from "svelte/transition";
    import { inview } from "svelte-inview";
    import Popup from "$lib/Popup.svelte";

    import { PUBLIC_CMS } from "$env/static/public";
    import Navbar from "$lib/Navbar.svelte";
    import Footer from "$lib/Footer.svelte";
    import Application from "../lib/products/Application.svelte";
    import Serverless from "../lib/products/Serverless.svelte";
    import Faq from "../lib/products/Faq.svelte";

    let api_engine_popup = false;
    let formal_welcome = false;
    let cdn_selected = true;

    const fetchPosts = async () => {
        console.log("Posts Load Ran");
        const res = await fetch(
            `${PUBLIC_CMS}/api/posts?pagination[page]=1&pagination[pageSize]=2&sort[0]=createdAt:desc&populate=*`
        );
        // Its encapsulated in a few data objects.
        const data = await res.json();
        console.log(data);
        return data;
    };

    import * as Accordion from "$lib/components/ui/accordion";
</script>

<!-- overarching background div bg-amber-400 -->
<div class="font-mona bg-gradient-to-tr from-zinc-900 to-90% to-fuchsia-900 bg-opacity-50">
    <Navbar />

    <div class="bg-gradient-to-b from-fuchsia-900 to-gray-950">

        <div class="bg-gradient-to-tr from-zinc-900 to-90% to-fuchsia-900 bg-opacity-50">
            <div in:fade class="pb-20 bg-cover bg-opacity-80 z-10">
                <div
                    class="p-3 pt-10 md:flex items-center justify-center grid-cols-2"
                >
                    <div>
                        <svg
                                viewBox="0 0 1024 1024"
                                class="absolute left-0 top-0 z-50 -translate-x-1/2 [mask-image:radial-gradient(closest-side,white,transparent)]"
                                aria-hidden="true"
                        >
                            <circle
                                    cx="512"
                                    cy="512"
                                    r="512"
                                    fill="url(#8d958450-c69f-4251-94bc-4e091a323369)"
                                    fill-opacity="0.3"
                            />
                            <defs>
                                <radialGradient id="8d958450-c69f-4251-94bc-4e091a323369">
                                    <stop stop-color="#7775D6" />
                                    <stop offset="1" stop-color="#E935C1" />
                                </radialGradient>
                            </defs>
                        </svg>
                        <button
                            class="font-mona md:ml-5 not-italic inline-flex flex-shrink-0 items-center rounded-full border-2 border-dashed px-2 py-0.5 font-medium text-gray-200 ring-1 ring-inset ring-green-600/20"
                        >
                            Now in public preview ↗
                        </button>
                        <p
                            class="text-slate-100 lg:text-7xl text-5xl md:pr-20 md:pt-10 pt-10 md:p-5 lg:pr-80"
                        >
                            <!-- let the creative light shine through, build fearlessly, paint your own online canvas -->
                            <a
                                class="font-ultrawide bg-clip-text text-transparent bg-gradient-to-br from-slate-400 from-5% to-95% to-slate-100"
                                >Content delivery and serverless for developers.</a
                            >
                            <br />
                        </p>
                        <p
                            class="lg:text-3xl text-xl text-slate-200 md:leading-3 md:pr-20 md:p-5 pt-2 lg:pr-80"
                        >
                            The best platform to deliver cohesive experiences. <br
                            />
                            Secure, build, and optimize, fearlessly at scale.
                            <!-- <a class="tracking-tight">creating effortless <a class="underline decoration-blue-500">faster</a>, more <a class="underline decoration-amber-500">secure</a>, and more <a class="underline decoration-fuchsia-500">robust</a> user experiences. -->
                        </p>
                    </div>
                </div>
            </div>

            <div
                id="opnieuw"
                class="bg-fuchsia-900 bg-opacity-50 md:flex justify-center m-6 text-gray-800 rounded-xl shadow-amber-500 hover:shadow-2xl hover:shadow-amber-400 duration-100 shadow-xl"
            >

                <div in:fade class="p-4 md:py-20 py-10 md:w-1/2 text-center">
                    <p class="text-2xl md:text-5xl p-3 text-slate-200">
                        A requirement for the modern product.
                    </p>
                    <p class="px-3 md:px-20 text-gray-300 font-mona">
                        To thrive, you cannot be beholden to cyberattacks or slow load times. You need to be unbreakable and fast.
                        You need to be able to scale up globally as your userbase does.
                        Packetware is the absolute best platform to speed up,
                        host, and protect your entire Internet presence against the myraid of attacks and vulnerabilities that exist today.
                        <br>
                        <br>
                        Successful products require robust infrastructure. You need a platform that has your back through it all. Let Packetware be your ally.
                    </p>
                    <a
                        href="/blog"
                        class="text-fuchsia-600 hover:text-fuchsia-400 pt-6"
                        >Read more on our blog ↗</a
                    >
                </div>

                <iframe
                    class="p-4 md:w-1/2 aspect-video rounded-lg shadow-xl"
                    src="https://www.youtube.com/embed/PcFFjbArGHM?loop=1"
                />
            </div>
        </div>

        <div class="p-10 md:px-5 py-14">
            <p
                    class="grid-cols-2 md:text-7xl text-4xl text-slate-100 font-ultrawide text-center"
            >
                The best cybersecurity. And the easiest compute. Unlimited scale.
            </p>
        </div>

    </div>


    <div
            id="product"
            class="bg-[url('/cubes.gif')] shadow-2xl shadow-fuchsia-900"
            transition:fade
    >

    <Application />
    <Serverless />

    </div>

    <div
        class="px-6 lg:px-8 py-8 sm:py-16 bg-cover bg-gradient-to-b from-gray-950 to-fuchsia-500"
    >
        <p class="font-krona text-center text-6xl text-slate-200 py-20">
            The next big thing is a lot of small things.
        </p>
        <div
            class="mx-auto grid max-w-7xl grid-cols-1 gap-x-8 gap-y-12 sm:gap-y-16 lg:grid-cols-2"
        >
            {#await fetchPosts() then posts}
                {#each posts.data as post}
                    <a href="/blog/{post.attributes.slug}">
                    <article
                        class="mx-auto md:w-full max-w-2xl lg:mx-0 lg:max-w-lg"
                    >
                        <time
                            datetime={post.attributes.createdAt}
                            class="block text-sm leading-6 text-gray-200"
                            >{new Date(
                                post.attributes.createdAt
                            ).toLocaleDateString()}</time
                        >
                        <h2
                            id="featured-post"
                            class="font-krona mt-4 text-3xl font-bold tracking-tight text-purple-700 sm:text-4xl"
                        >
                            {post.attributes.title}
                        </h2>
                        <p class="mt-5 line-clamp-3 leading-6 text-slate-200">
                            {post.attributes.description}
                        </p>
                        <div
                            class="mt-4 md:flex flex-col justify-between gap-6 sm:mt-8 sm:flex-row-reverse sm:gap-8 lg:mt-4 lg:flex-col"
                        >
                            <div class="sm:flex">
                                <a
                                    href="/blog/{post.attributes.slug}"
                                    class="text-sm font-semibold leading-6 text-indigo-600"
                                    aria-describedby="featured-post"
                                    >Continue reading <span aria-hidden="true"
                                        >&rarr;</span
                                    ></a
                                >
                            </div>
                        </div>
                    </article>
                    </a>
                {/each}
            {/await}
        </div>

        <p
            class="text-2xl py-5 text-center text-amber-400 hover:text-amber-600"
        >
            <a href="/co/about">Read more about us ↗</a>
        </p>
    </div>


    <section
        class="px-6 bg-gray-600 bg-gradient-to-b from-fuchsia-500 via-fuchsia-900 to-zinc-900 py-5 lg:px-8 lg:pb-16"
    >
        <Faq />

        <div class="relative isolate overflow-hidden border-2 border-fuchsia-500">
            <div class="px-6 py-24 sm:px-6 sm:py-32 lg:px-8">
                <div class="mx-auto max-w-2xl text-center">
                    <h2
                            class="text-3xl font-bold tracking-tight text-white sm:text-4xl font-krona"
                    >
                        It's the infrastructure robustness you've always deserved.
                    </h2>
                    <p
                            class="mx-auto mt-6 max-w-xl text-lg leading-8 text-gray-300"
                    >
                        From API Engine to Analytics By Example, all of our products
                        are designed to make yours more secure, coherent, and
                        faster.<br /> Join the beta.
                    </p>
                    <div class="mt-10 sm:flex items-center justify-center gap-x-6">
                        <a
                                href="/i/auth/register"
                                class="rounded-md bg-white px-3.5 py-2.5 text-sm font-semibold text-gray-900 shadow-sm hover:bg-gray-100 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-white"
                        >Get started</a
                        >
                        <a
                                href="#"
                                class="text-sm font-semibold leading-6 text-white"
                        >Learn more <span aria-hidden="true">→</span></a
                        >
                    </div>
                </div>
            </div>
            <svg
                    viewBox="0 0 1024 1024"
                    class="absolute left-1/2 top-1/2 z-50 h-[64rem] w-[64rem] -translate-x-1/2 [mask-image:radial-gradient(closest-side,white,transparent)]"
                    aria-hidden="true"
            >
                <circle
                        cx="512"
                        cy="512"
                        r="512"
                        fill="url(#8d958450-c69f-4251-94bc-4e091a323369)"
                        fill-opacity="0.7"
                />
                <defs>
                    <radialGradient id="8d958450-c69f-4251-94bc-4e091a323369">
                        <stop stop-color="#7775D6" />
                        <stop offset="1" stop-color="#E935C1" />
                    </radialGradient>
                </defs>
            </svg>
        </div>

        <figure class="mx-auto pt-20 max-w-2xl">
            <p class="uppercase text-gray-200 font-krona">
                daniel h, CTO hydrogen.sh
            </p>
            <blockquote
                class="mt-10 text-xl font-semibold leading-8 tracking-tight text-gray-200 sm:text-2xl sm:leading-9"
            >
                <p>
                    “We had huge exponential growth after launching our product;
                    as our users increased, so did our bandwidth use. Our entire
                    infrastructure was hosted on giant cloud, and we were
                    quickly made aware of the insane egress rates. We looked for
                    a solution as soon as possible, but every CDN we found
                    offered the same outrageous pricing or lackluster features,
                    until we discovered Packetware. They brought us onto their
                    network in a couple of hours, and as a result, we gained
                    incredible stability across our 300k userbase, thanks to the
                    fancy security features. Caching took effect instantly and
                    was easy to control."
                </p>
            </blockquote>
        </figure>
    </section>

    <Footer />
</div>
