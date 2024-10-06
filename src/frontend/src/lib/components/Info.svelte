<script>
    import * as Tooltip from "$lib/components/ui/tooltip";

    import { Info } from "lucide-svelte";

    export let content = "";
</script>

<Tooltip.Root>
    <Tooltip.Trigger><Info class="ml-1 h-4 w-4"/></Tooltip.Trigger>
    <Tooltip.Content>
        <p>{content}</p>
    </Tooltip.Content>
</Tooltip.Root>
