import { error } from "@sveltejs/kit";

import { PUBLIC_CMS } from '$env/static/public';

/** @type {import('./$types').PageLoad} */
export async function load({ fetch }) {
    const fetchPosts = async () => {
        console.log('Posts Load Ran');
        const res = await fetch(`${PUBLIC_CMS}/api/posts?sort[0]=createdAt:desc&populate=*`);
        // Check if the request was successful
        if (!res.ok) {
            throw error(`HTTP error! status: ${res.status}`);
        }
        // Its encapsulated in a few data objects.
        const data = await res.json();
        //console.log(data);
        return data;
    };

    return {
        posts: await fetchPosts()
    };
}
