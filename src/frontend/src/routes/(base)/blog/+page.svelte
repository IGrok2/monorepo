<script>
    import { PUBLIC_CMS } from '$env/static/public';

    /** @type {import('./$types').PageData} */
    export let data;

    $: posts = data.posts.data;
</script>

<div class="py-24 sm:py-32">
    <div class="mx-auto max-w-7xl px-6 lg:px-8">
        <div class="mx-auto max-w-2xl lg:mx-0">
            <h2
                class="text-3xl font-bold tracking-tight text-white sm:text-4xl"
            >
                From the blog
            </h2>
            <p class="mt-2 text-lg leading-8 text-gray-400">
                Empowering your network with cutting-edge CDN insights and
                strategies.
            </p>
        </div>
        <div
            class="mx-auto mt-10 grid max-w-2xl grid-cols-1 gap-x-8 gap-y-16 border-t border-gray-700 pt-10 sm:mt-16 sm:pt-16 lg:mx-0 lg:max-w-none lg:grid-cols-3 bg-dark"
        >
            {#each posts as post}
                <article
                    class="flex max-w-xl flex-col items-start justify-between"
                >
                    <div class="flex items-center gap-x-4 text-xs">
                        <time datetime="2020-03-16" class="text-gray-400">
                            {new Date(post.attributes.createdAt).toLocaleDateString()}
                        </time>
                        <a
                            href="/blog/caategory/{post.attributes.category}"
                            class="relative z-10 rounded-full bg-fuchsia-700 px-3 py-1.5 font-medium text-gray-400 hover:bg-gray-600"
                        >
                            {post.attributes.category}
                        </a>
                    </div>
                    <div class="group relative">
                        <h3
                            class="mt-3 text-lg font-semibold leading-6 text-white group-hover:text-gray-400"
                        >
                            <a href="/blog/{post.attributes.slug}">
                                <span class="absolute inset-0" />
                                {post.attributes.title}
                            </a>
                        </h3>
                        <p
                            class="mt-5 line-clamp-3 text-sm leading-6 text-gray-400"
                        >
                            {post.attributes.description}
                        </p>
                    </div>
                    <div class="relative mt-8 flex items-center gap-x-4">
                        <img
                            src="{PUBLIC_CMS}/uploads/thumbnail_bigballa_b1f3c720b1.png"
                            alt=""
                            class="h-10 w-10 rounded-full bg-gray-700"
                        />
                        <div class="text-sm leading-6">
                            <p class="font-semibold text-white">
                                <a href="#">
                                    <span class="absolute inset-0" />
                                    {post.attributes.author.data.attributes.name}
                                </a>
                            </p>
                            <p class="text-gray-400">{post.attributes.author.data.attributes.about}</p>
                        </div>
                    </div>
                </article>
            {/each}
        </div>
    </div>
</div>
