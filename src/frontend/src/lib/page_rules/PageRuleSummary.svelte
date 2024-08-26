<script>
    export let rule;
</script>

<!--
    order: Number,
    trigger: {
        match_type: [{
            trigger: TriggerType, // TODO: make sure this only contains one
            match_type: String, // Exact, Contains, StartsWith TODO make sure this is the case
            inversed: Boolean,
            required: Boolean
        }],
        trigger_requirement: String, // either "one" or "all", implies if this rule is required or not
    },
    action: { // one or the other
        monopoly: String, // Block is the only option
        trustbusting: [String], // multiple of these things can happen:
        // smart challenge, captcha, ratelimit (with bucket name), cache (with level + ttl), redirect, origin setting

        // special action information, can be undefined
        bucket_name: String,

        cache_level: String,
        cache_level_ttl: String,

        redirect: String,
        backend_host: String
    }
-->

<div>
    {rule.trigger.match_type.length} matches, of which {rule.trigger.trigger_requirement.toLowerCase()} must match.
    <br>
    {#each rule.trigger.match_type as match_type, match_index}
        Match type {match_index + 1} matches on {Object.keys(match_type.trigger)[0].toUpperCase()} with option {match_type.match_type.toLowerCase()}, does {match_type.inversed ? `inverse` : `not inverse on match`}{match_type.required ? `, and is required` : ``}.
    {/each}
    <br>
    <br>
    When matched,
    {#if rule.action.trustbusting && rule.action.trustbusting.length !== 0}
        completes concurrent (trustbust) options:
        {#each rule.action.trustbusting as trustbust, index}
            &nbsp;{trustbust.toLowerCase()}{#if index + 1 !== rule.action.trustbusting.length},{/if}
        {/each}
    {:else}
    {/if}
</div>
