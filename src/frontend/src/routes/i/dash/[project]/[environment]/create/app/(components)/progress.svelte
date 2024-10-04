<script>
    import { Progress } from "$lib/components/ui/progress";

    export let steps;
    export let stepNumber;
</script>

<div class="md:flex md:items-center md:space-x-4 space-y-2">
    <h3>{steps[stepNumber]}</h3>
    <Progress value={stepNumber} max={steps.length - 1} />
</div>
