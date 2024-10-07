export const ssr = false;

import APIClient from "$lib/utils/api";

/** @type {import('./$types').LayoutLoad} */
export async function load({ params }) {
    try {
        let res = await APIClient.get(`/project/${params.project}/`);

        return {
            project: res.data.data.project
        }
    }
    catch (err) {
        console.log("err: ", err)
    }
};
