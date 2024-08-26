<script>
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

    $: TOS = data.TOS.data.attributes;

    $: addContent(TOS.document);

    //$: console.log(TOS);
</script>

<title>Terms of Service</title>

<main class="p-4 flex justify-center">
    <article class="prose prose-invert">
        {@html HTMLSource}
    </article>
</main>
