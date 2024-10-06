<script>
    import { fade, slide } from "svelte/transition";
    import { inview } from "svelte-inview";
    import Rizz from "$lib/Rizz.svelte";

    import { PUBLIC_CMS } from "$env/static/public";
    import Navbar from "$lib/Navbar.svelte";

    let title = false;
    let who_are_we = false;
    let blog = false;

    const fetchPosts = async () => {
        console.log("Posts Load Ran");
        const res = await fetch(
            `${PUBLIC_CMS}/api/posts?pagination[page]=1&pagination[pageSize]=2&sort[0]=createdAt:desc&populate=*`,
        );
        // Check if the request was successful
        if (!res.ok) {
            throw error(`HTTP error! status: ${res.status}`);
        }
        // Its encapsulated in a few data objects.
        const data = await res.json();
        console.log(data);
        return data;
    };
</script>

<Navbar />

<div
    class="pb-40"
    use:inview={{ unobserveOnEnter: true, rootMargin: "-20%" }}
    on:change={({ detail }) => {
        title = detail.inView;
    }}
>
    <p in:slide class="text-8xl text-slate-200 p-6 pr-20 pt-6 tracking-tight">
        About us: we believe that the next big thing is a lot of small things.
    </p>
    <p in:fade class="pl-36 text-2xl text-zinc-200 text-right pr-6">
        ... that's why we've crafted a content delivery platform crafted for
        security and performance requirements of tomorrow. Build fearlessly, now
        then again.
    </p>
</div>

<p class="md:text-5xl text-3xl text-gray-200 text-center py-24 pb-14 px-14">
    It's not just about caching, optimization. <br />Cybersecurity. <br /><br
    />It's platform coherency. <br />Letting your ambition find its canvas.
    <br /><br /><a class="font-ultrawide text-4xl  md:text-7xl"
        >It's about daring to <a class="whitespace underline decoration-blue-500"
            >democratize</a
        >
        a
        <a
            class="text-transparent bg-clip-text bg-gradient-to-r from-blue-500 to-fuchsia-500"
            >better</a
        > Internet edge. Fearlessly.</a
    >
</p>

<div
    class="p-8 sm:bg-cover md:bg-no-repeat text-center px-5 md:px-10 w-full justify-center"
    use:inview={{ unobserveOnEnter: false, rootMargin: "-5%" }}
    on:change={({ detail }) => {
        who_are_we = detail.inView;
    }}
>
    <p in:fade class="text-left text-sm uppercase text-gray-200 font-bold">
        who are we?
    </p>
    <div
        in:slide
        class="pt-4 text-slate-100 text-xl first-line:uppercase first-line:tracking-widest
      first-letter:text-3xl first-letter:font-bold first-letter:font-serif first-letter:text-white px-5 md:px-10"
    >
        We are web developers that were frustrated with traditional content
        delivery platforms.<br />
        We tried to reach out to these platforms to specific suggestions to improve
        their products - they wouldn't listen. We were ready for change. So we began
        crafting the outline for a new service, based on fundamentally sound principles.<br
        /><br />
        Packetware was formed from this ambition to create a higher level of robustness
        from cyberattacks and traffic spikes. <br />Sticking by our three
        principles, molding a product that was better in meaningful ways. We believe every user of a website is
        impacted either negatively or positively. Our desire to relive humankind
        of frustrating experiences in large ways, such as never-ending loading
        loops, services being down, or filling out CAPTCHAs, but also in subtle
        ways, like incoherent (slow, laggy, or error prone) experiences that
        leave users unsatisfied.
        <br /><br />
        <a class="italic">Our principles:</a>
        <ul class="list-decimal px-12 md:px-36 text-left py-10 font-bold">
            <li>
                Go out of the way to offer incredible experiences for our clients and their beloved users
            </li>
            <li>
                Cybersecurity shouldn't be something you have to configure
            </li>
            <li>
                We should push for positive internet change (privacy, security) before profits or success
            </li>
        </ul>
    </div>
</div>

<div
    class="px-6 lg:px-8 py-8 sm:py-16"
>
    <p class="font-krona text-center text-4xl md:text-6xl text-slate-200 py-20">
        Learn more about Packetware
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
                                post.attributes.createdAt,
                            ).toLocaleDateString()}</time
                        >
                        <h2
                            id="featured-post"
                            class="font-krona mt-4 text-3xl font-bold tracking-tight text-gray-100 sm:text-4xl"
                        >
                            {post.attributes.title}
                        </h2>
                        <p class="mt-5 line-clamp-3 leading-6 text-slate-400">
                            {post.attributes.description}
                        </p>
                        <div
                            class="mt-4 md:flex flex-col justify-between gap-6 sm:mt-8 sm:flex-row-reverse sm:gap-8 lg:mt-4 lg:flex-col"
                        >
                            <div class="sm:flex">
                                <a
                                    href="/blog/{post.attributes.slug}"
                                    class="text-sm font-semibold leading-6 text-fuchsia-600"
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
        class="text-2xl py-5 text-center text-white hover:text-fuchsia-400"
    >
        <a href="/co/about">Read more about us ↗</a>
    </p>
</div>

<div
    class="p-8 sm:bg-cover md:bg-no-repeat text-center px-5 md:px-10 md:shadow-xl md:backdrop-blur shadow-black w-full justify-center"
>
    <p class="text-left text-sm uppercase text-gray-200 font-bold">
        Note directly from the founder
    </p>
    <div class="pt-4 text-slate-100 text-2xl px-5 md:px-10">
        <div class="first-line:uppercase">
            <a class="md:indent-8"
                >I am the founder of Packetware and I believe every product and
                every company is an expression of a person. And at a fundamental
                level, an expression of humanity itself. It is humanity’s
                thoughts, feelings, and intellect, abstracted into something
                shared for others on the Web.</a
            >
            <br /><br />
            <a class="md:indent-8"
                >Considering the Internet is the largest and most powerful
                interconnective expression of humanity, it needs to be safe. It
                needs to represent humanity, which begins with every online
                product and service, no matter how small or large.</a
            >
        </div>
        <br /><br />
        <p
            class="text-right pl-4 text-xl md:text-2xl pr-6 md:pr-16 uppercase text-gray-200 font-bold"
        >
            With a Whole Lotta Heart,
        </p>
        <p class="text-right pt-2 text-2xl">Edward Coristine</p>
    </div>
</div>

<Rizz />
