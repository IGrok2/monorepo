import APIClient from "$lib/utils/api";

/** @type {import('./$types').PageLoad} */
export async function load({ params }) {
    try {
        let res = await APIClient.get(`/project/${params.project}/environment/${params.environment}/volumes`);

        return {
            volumes: res.data.data.volumes
        }
    }
    catch (err) {
        console.log("err: ", err)
    }
}