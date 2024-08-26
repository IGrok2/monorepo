export const ssr = false;

//import { getCookie } from "$lib/utils/auth";

//import { PUBLIC_API } from "$env/static/public";

/*
export async function load({ params }) {
    const token = getCookie(`jwt`);

	const response = await fetch(`${PUBLIC_API}/@/github/repo/${params.repo}`, {
        method: "GET",
        headers: new Headers({
            "content-type": "application/json",
            Authorization: token,
        }),
    });

    const repo = await response.json();
    console.log(repo);

    return {
        repo,
    };
}
*/
