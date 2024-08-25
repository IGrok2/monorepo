const newPageRule = async (rule, domain) => {
    if (new_rule_matches.length === 0) {
        await error_notification("Couldn't stage new page rule", "The first rule wasn't filled out")
        return;
    }

    for (let i = 0; i < new_rule_matches.length; i++) {
        if (new_rule_matches[i].triggerSelected === null || new_rule_matches[i].match_type === null) {
            await error_notification("Couldn't stage new page rule", `Match ${i + 1} wasn't filled out`)
            return;
        }

        if (new_rule_matches[i].triggerSelected === "Query string") {
            if (Object.keys(new_rule_matches[i].trigger.query).length === 0) {
                await error_notification("Couldn't stage new page rule", `Match ${i + 1} wasn't filled out`)
                return;
            }
        } else if (new_rule_matches[i].triggerSelected === "Header") {
            if (Object.keys(new_rule_matches[i].trigger.headers).length === 0) {
                await error_notification("Couldn't stage new page rule", `Match ${i + 1} wasn't filled out`)
                return;
            }
        } else if (new_rule_matches[i].triggerSelected === "IP address") {
            // check if IP is filled out, then check if it's a valid IP
            if (!new_rule_matches[i].trigger.ip) {
                await error_notification("Couldn't stage new page rule", `Match ${i + 1} wasn't filled out`)
                return;
            }

            const reg = /^(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$/;

            if (!reg.test(new_rule_matches[i].trigger.ip)) {
                await error_notification("Couldn't stage new page rule", `Match ${i + 1} had an invalid IP address`)
                return;
            }
        } else if (new_rule_matches[i].triggerSelected === "HTTP Path") {
            // make sure it contains a slash
            if (!new_rule_matches[i].trigger.path || !new_rule_matches[i].trigger.path.includes("/")) {
                await error_notification("Couldn't stage new page rule", `Match ${i + 1} didn't have a correct path`)
                return;
            }
        } else if (new_rule_matches[i].triggerSelected === "ASN") {
            // make sure it's a number
            if (!new_rule_matches[i].trigger.asn || isNaN(new_rule_matches[i].trigger.asn)) {
                await error_notification("Couldn't stage new page rule", `Match ${i + 1} didn't have a correct ASN. Should be a number!`)
                return;
            }
        } else if (new_rule_matches[i].triggerSelected === "Country") {
            // make sure it's two characters
            if (!new_rule_matches[i].trigger.country || new_rule_matches[i].trigger.country.length !== 2) {
                await error_notification("Couldn't stage new page rule", `Match ${i + 1} didn't have a correct country`)
                return;
            }
        } else if (new_rule_matches[i].triggerSelected === "Continent") {
            // make sure it's two characters
            if (!new_rule_matches[i].trigger.continent || new_rule_matches[i].trigger.continent.length !== 2) {
                await error_notification("Couldn't stage new page rule", `Match ${i + 1} didn't have a correct continent`)
                return;
            }
        }
    }

    // now, check triggers. first, make sure there's at least one
    if (!new_rule_actiontype) {
        await error_notification("Couldn't stage new page rule", `No action type was selected`)
        return;
    } else if (new_rule_actiontype === "Monopoly" && !new_rule_monopoly) {
        await error_notification("Couldn't stage new page rule", `Monopoly action was indicated, but no monopoly was selected`)
        return;
    } else if (new_rule_actiontype === "Trustbust" && (!new_rule_trustbust || new_rule_trustbust.length === 0)) {
        await error_notification("Couldn't stage new page rule", `Trustbust action was indicated, but no trustbust was selected`)
        return;
    }

    if (true) {
        new_rule_popup = false;

        rules = [...rules, {
            new_staged: true,
            existed_modified: false,
            being_deleted: false,
            order: new_rule_order,
            match: new_rule_matches,
            actions: new_rule_actions,
            action_type: new_rule_actiontype,
            monopoly: new_rule_monopoly,
            trustbust: new_rule_trustbust,
            cache_settings: new_rule_cache_settings,
            bucket: new_rule_bucket,
            redirect: new_rule_redirect,
            backend_host: new_rule_backend_host
        }]

        await sortRules()

        await letSaveChanges()
    } else {
        await error_notification("Couldn't stage new API Engine rule", "Didn't include any paths")
    }
}

const editPageRule = async (rule, domain) => {

}


const deletePageRule = async (rule, domain) => {
    await confirmChange(`Delete ${rule.new_staged ? `staged` : `existing`} page rule?`, `${rule.new_staged ? "Because this rule was staged, nothing will change in your production environment" : "This rule will be staged for deletion"}`,
        () => { // TODO: deleting the setting doesn't delete the rules yet
            if (rule.new_staged) {
                rules.splice(index, 1);
                rules = rules;
            } else {
                rule.being_deleted = true;
            }
        })
}
