import APIClient from "$lib/utils/api";

/** @type {import('./$types').PageLoad} */
export async function load({ params }) {
    try {
        let res = await APIClient.get(`/project/${params.project}/environment/${params.environment}/app/${params.app}/deployment/${params.deployment}`);

        return {
            deployment: res.data.data.deployment
        }
    }
    catch (err) {
        console.log("err: ", err)
    }
}
