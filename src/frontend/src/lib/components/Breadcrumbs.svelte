<script>
    import { Slash } from "lucide-svelte";

    export let domain = "";
    export let route = "";
</script>

<nav class="flex justify-center pb-5" aria-label="Breadcrumb">
    <ol role="list" class="mt-4 flex items-center space-x-4">
        <li>
            <div class="flex items-center">
                <a
                    href="/i/dash/domains"
                    class="ml-4 text-xs md:text-sm font-medium text-muted-foreground hover:text-primary"
                    >domains</a
                >
            </div>
        </li>
        <li>
            <div class="flex items-center">
                <Slash />
                <a
                    href="/i/dash/domains/{domain}"
                    class="ml-4 text-xs md:text-sm font-medium text-muted-foreground hover:text-primary"
                    aria-current="page">{domain}</a
                >
            </div>
        </li>
        <li>
            <div class="flex items-center">
                <Slash />
                <span
                    class="ml-4 text-xs md:text-sm font-medium text-muted-foreground hover:text-primary"
                    aria-current="page">{route}</span
                >
            </div>
        </li>
    </ol>
</nav>
