import { error, fail, redirect } from "@sveltejs/kit"
import axios from 'axios';

/** @type {import('./$types').PageLoad} */
export async function load({ parent, url }) {
    const { session, user } = await parent();

    let path = url.searchParams.get("path")

    if (!path) {
        path = "";
    }

    const response = await axios.get('http://104.129.132.111:31920/files/contents', {
        params: {
            path: `/mnt/server${path}`
        }
    });

    return { content: response.data.content };
}

/** @type {import('./$types').Actions} */
export const actions = {
    save: async ({ cookies, request, params, url }) => {
        const data = await request.formData();
        const content = data.get('content');
        const path = data.get('path');
        console.log(path, content);

        let containerPath = `/mnt/server${encodeURIComponent(path)}`;

        try {
            const response = await axios.post(`http://104.129.132.111:31920/files/write?path=${containerPath}`, {
                content: content,
            }, {
                headers: {
                    'Content-Type': 'application/x-www-form-urlencoded'
                }
            });
            console.log(response.data);

            return { message: response.data.message };
        } catch (error) {
            console.log('Error saving file:', error.response.data);
            return { message: "Error saving file."}
        }

        //redirect(302, `/services/instance/${params.id}/files/edit?path=${path}`)
    }
};
