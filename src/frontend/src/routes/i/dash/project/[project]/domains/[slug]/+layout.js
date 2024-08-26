export const ssr = false;

import { PUBLIC_API } from "$env/static/public";
import { getCookie } from "$lib/utils/auth";

import { redirect } from "@sveltejs/kit";
//import domainsAPIClient from "$lib/utils/domains.client";

/** @type {import('./$types').LayoutLoad} */
export async function load({ params }) {
    console.log("layout loaded");
    const token = getCookie('jwt');

    let project;

    let domain;
    let verification_key;
    let verification_required;
    let verification_date;
    let success;
    let no_origins;

    try {
        const response = await fetch(`${PUBLIC_API}/@/project/${params.project}/brief`, {
            method: "GET",
            headers: new Headers({
                "content-type": "application/json",
                Authorization: token,
            }),
        });

        let res = await response.json();
        if (res.success) {
            project = res.project;
        }
    } catch (error) {
        console.log("Error fetching project brief: ", error);
    }

    try {
        const response = await fetch(`${PUBLIC_API}/@/project/${params.project}/domain/${params.slug}`, {
            method: "GET",
            headers: new Headers({
                "content-type": "application/json",
                Authorization: token,
            }),
        });

        domain = await response.json();
        console.log("domain: ", domain);

        if (response.status === 403 && domain.message === "Verification required") {
            verification_key = domain.verificationToken;
            verification_date = domain.lastVerifiedCheck;
            verification_required = true;
        } else if (response.status !== 200) {
            success = false;
        } else {
            success = true;
            // check if they have an origin or not
            // we only want this on a 200
            if (domain.domain.origin_settings.length === 0) {
                no_origins = true;
            }
        }

        return {
            resp: {
                success,
                project,
                domain,
                no_origins
            },
            verification: {
                required: verification_required,
                key: verification_key,
                date: verification_date,
            },
            time_based: Date.now(),
        };
    } catch (err) {
        console.log("error: ", err);
        throw redirect(307, "/i/dash/");
    }
}
