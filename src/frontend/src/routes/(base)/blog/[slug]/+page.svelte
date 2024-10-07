<script>
    import { PUBLIC_CMS } from '$env/static/public';

    /** @type {import('./$types').PageData} */
    export let data;

    import MarkdownIt from "markdown-it";

    let HTMLSource = `Loading...`;

    function addContent(content) {
        const md = new MarkdownIt();
        var result = md.render(content);
        HTMLSource = result;
        return ""; // So it doesn't print "undefined" on the page
    }

    $: post = data.post.data[0].attributes;

    $: addContent(post.content);

    $: console.log(post);
</script>

<svelte:head>
    <title>{post.title}</title>
    <meta name="description" content={post.description} />
    <meta name="og:title" content={post.title} />
    <meta name="og:description" content={post.description} />
    <meta name="og:type" content="article" />
    <meta name="og:url" content="https://packetware.net/posts/{post.slug}" />
    <meta name="og:site_name" content="Packetware" />
    <meta name="twitter:card" content="summary_large_image" />
    <meta name="twitter:site" content="@packetware" />
    <meta name="twitter:title" content={post.title} />
    <meta name="twitter:description" content={post.description} />
    <meta name="twitter:creator" content="@packetware" />
    <meta name="twitter:label2" content="Published on" />
    <meta name="twitter:data2" content={post.createdAt} />
    {#if post.image.data}
        <meta
            name="og:image"
            content="https://cms.anchored.host{post.image.data.attributes.url}"
        />
        <meta
            name="twitter:image"
            content="https://cms.anchored.host{post.image.data.attributes.url}"
        />
    {/if}
</svelte:head>

<main class="pt-8 pb-16 lg:pt-16 lg:pb-24">
    <div class="flex justify-between px-4 mx-auto max-w-screen-xl">
        <article
            class="mx-auto w-full max-w-2xl format format-sm sm:format-base lg:format-lg format-blue dark:format-invert"
        >
            <header class="mb-4 lg:mb-6 not-format">
                <address class="flex items-center mb-6 not-italic">
                    <div
                        class="inline-flex items-center mr-3 text-sm text-gray-900 dark:text-white"
                    >
                        <img
                            class="mr-4 w-16 h-16 rounded-full"
                            src="{PUBLIC_CMS}/uploads/thumbnail_bigballa_b1f3c720b1.png"
                            alt="Author Image"
                        />
                        <div>
                            <a
                                href="/author/{post.author.data.attributes
                                    .slug}"
                                rel="author"
                                class="text-xl font-bold text-gray-900 dark:text-white"
                                >{post.author.data.attributes.name}</a
                            >
                            <p
                                class="text-base font-light text-gray-500 dark:text-gray-400"
                            >
                                {post.author.data.attributes.about}
                            </p>
                            <p
                                class="text-base font-light text-gray-500 dark:text-gray-400"
                            >
                                <time
                                    pubdate
                                    datetime="2022-02-08"
                                    title="February 8th, 2022"
                                    >{new Date(
                                        post.createdAt,
                                    ).toLocaleDateString()}</time
                                >
                            </p>
                        </div>
                    </div>
                </address>

                <h1
                    class="mb-4 text-3xl font-extrabold leading-tight text-gray-900 lg:mb-6 lg:text-4xl dark:text-white"
                >
                    {post.title}
                </h1>
            </header>

            <!-- Video Embed -->
            {#if post.video}
                <div style="position:relative;padding-top:56.25%;">
                    <iframe
                        src={post.video}
                        title="YouTube video player"
                        frameborder="0"
                        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
                        allowfullscreen
                        style="position:absolute;top:0;left:0;width:100%;height:100%;"
                    />
                </div>
            {:else if post.image.data}
                <img
                    class="my-8"
                    src="{PUBLIC_CMS}{post.image.data.attributes
                        .url}"
                    alt={post.image.data.attributes.caption}
                />
            {/if}

            <article class="prose prose-invert flex-grow">
                {@html HTMLSource}
            </article>
        </article>
    </div>
</main>
