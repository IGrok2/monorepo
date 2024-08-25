export const ssr = false;

import { getCookie } from "$lib/utils/auth";

import { PUBLIC_API } from "$env/static/public";

const installs = async () => {
    const token = getCookie(`jwt`);

    const response = await fetch(`${PUBLIC_API}/@/github/repos`, {
        method: "GET",
        headers: new Headers({
            "content-type": "application/json",
            Authorization: token,
        }),
    });

    return await response.json();
}

/** @type {import('./$types').PageLoad} */
export async function load({ params }) {
	/*const response = await fetch(`${PUBLIC_API}/@/github/repos`, {
        method: "GET",
        headers: new Headers({
            "content-type": "application/json",
            Authorization: token,
        }),
    });

    const github = await response.json();*/

    //return {
    //    github: await installs(),
    //};
}