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

    $: doc = data.doc.data[0].attributes;

    $: addContent(doc.content);

    $: console.log(doc);
</script>

<!-- SEO -->
<svelte:head>
    <title>{doc.title}</title>
    <meta name="description" content={doc.description} />
    <meta name="og:title" content={doc.title} />
    <meta name="og:description" content={doc.description} />
    <meta name="og:type" content="article" />
    <meta name="og:url" content="https://packetware.net/docs/{doc.section.data.attributes.slug}/{doc.slug}" />
    <meta name="og:site_name" content="Packetware" />
    <meta name="twitter:card" content="summary_large_image" />
    <meta name="twitter:site" content="@packetware" />
    <meta name="twitter:title" content={doc.title} />
    <meta name="twitter:description" content={doc.description} />
    <meta name="twitter:creator" content="@packetware" />
    <meta name="twitter:label2" content="Published on" />
    <meta name="twitter:data2" content={doc.createdAt} />
    <!--
	{#if doc.image.data != null}
	<meta
		name="og:image"
		content="https://cms.anchored.host{doc.image.data.url}"
	/>
	<meta
		name="twitter:image"
		content="https://cms.anchored.host{doc.image.data.url}"
	/>
	{/if}
    -->
</svelte:head>

<article class="">
    <header class="">
        <p class="text-base font-medium text-fuchsia-500">{doc.section.data.attributes.name}</p>

        <h1 class="text-3xl font-bold tracking-tight text-white">
            {doc.title}
        </h1>
    </header>
    <!-- Description -->
    <p class="mt-2 text-xl text-gray-400">
        {doc.description}
    </p>
    <!-- Video Embed
    {#if doc.video}
        <div style="position:relative;padding-top:56.25%;">
            <iframe
                src={doc.video}
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
