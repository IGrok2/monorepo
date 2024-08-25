export const ssr = false;

import { getCookie } from "$lib/utils/auth";

import { PUBLIC_API } from "$env/static/public";

/** @type {import('./$types').LayoutLoad} */
export async function load({ params }) {
    const token = getCookie("jwt");

    const response = await fetch(
        `${PUBLIC_API}/@/project/${params.project}/container/${params.id}`,
        {
            method: "GET",
            headers: new Headers({
                "content-type": "application/json",
                Authorization: token,
            }),
        },
    );

    const res = await response.json();
    console.log(res);

    return {
        project: res.project,
        container: res.container,
        deployment: res.deployment,
        loading: false,
    }
}