import APIClient from "$lib/utils/api";

/** @type {import('./$types').PageLoad} */
export async function load({ params }) {
    try {
        let res = await APIClient.get(`/project/${params.project}/environment/${params.environment}/app/${params.app}/deployments`);

        return {
            deployments: res.data.data.deployments
        }
    }
    catch (err) {
        console.log("err: ", err)
    }
}
