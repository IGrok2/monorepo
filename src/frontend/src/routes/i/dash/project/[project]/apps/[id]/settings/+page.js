export const ssr = false;

import { getCookie } from "$lib/utils/auth";

import { PUBLIC_API } from "$env/static/public";

/** @type {import('./$types').PageLoad} */
export async function load({ params, parent }) {

    const parentLoad = await parent();
    const container = parentLoad.container;

    const token = getCookie(`jwt`);

	const response = await fetch(`${PUBLIC_API}/@/github/repo/${container.github.git_repo_id}`, {
        method: "GET",
        headers: new Headers({
            "content-type": "application/json",
            Authorization: token,
        }),
    });

    const repo = await response.json();

    return {
        repo,
    };
}