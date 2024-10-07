import APIClient from "$lib/utils/api";

/** @type {import('./$types').LayoutLoad} */
export async function load({ params }) {
    try {
        let res = await APIClient.get(`/project/${params.project}/environment/${params.environment}/app/${params.app}/`);

        return {
            app: res.data.data.app
        }
    }
    catch (err) {
        console.log("err: ", err)
    }
}
