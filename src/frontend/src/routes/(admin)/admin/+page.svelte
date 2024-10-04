<script>
    /** @type {import('./$types').PageData} */
    export let data;
    
    import MarkdownIt from "markdown-it";
    
    //import TableOfContents from "./(components)/TableOfContents.svelte";

    let HTMLSource = `Loading...`;

	function addContent(content) {
		const md = new MarkdownIt();
		var result = md.render(content);
		HTMLSource = result;
		return ''; // So it doesn't print "undefined" on the page
	}

    $: page = data.page.data[0].attributes;

    $: addContent(page.content);

    $: console.log(page);
</script>

<!-- SEO -->
<svelte:head>
    <title>{page.title}</title>
    <meta name="description" content={page.description} />
    <meta name="og:title" content={page.title} />
    <meta name="og:description" content={page.description} />
    <meta name="og:type" content="article" />
    <meta name="og:url" content="https://packetware.net/docs/{page.slug}" />
    <meta name="og:site_name" content="Packetware" />
    <meta name="twitter:card" content="summary_large_image" />
    <meta name="twitter:site" content="@packetware" />
    <meta name="twitter:title" content={page.title} />
    <meta name="twitter:description" content={page.description} />
    <meta name="twitter:creator" content="@packetware" />
    <meta name="twitter:label2" content="Published on" />
    <meta name="twitter:data2" content={page.createdAt} />
    <!-- 
	{#if page.image.data != null}
	<meta
		name="og:image"
		content="https://cms.anchored.host{page.image.data.url}"
	/>
	<meta
		name="twitter:image"
		content="https://cms.anchored.host{page.image.data.url}"
	/>
	{/if}
    -->
</svelte:head>

<article class="">
    <header class="">
        <p class="text-base font-medium text-fuchsia-500">{page.section.data.attributes.name}</p>

        <h1 class="text-3xl font-bold tracking-tight text-white">
            {page.title}
        </h1>
    </header>
    <!-- Description -->
    <p class="mt-2 text-xl text-gray-400">
        {page.description}
    </p>
    <!-- Video Embed 
    {#if page.video}
        <div style="position:relative;padding-top:56.25%;">
            <iframe
                src={page.video}
                title="YouTube video player"
                frameborder="0"
                allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
                allowfullscreen
                style="position:absolute;top:0;left:0;width:100%;height:100%;"
            />
        </div>
    {/if} -->

    <!-- Body of Document -->
    <article class="mt-4">
        <article class="prose prose-invert flex-grow">
            {@html HTMLSource}
        </article>
    </article>
</article>
