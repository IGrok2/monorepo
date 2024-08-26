import { error } from "@sveltejs/kit";
import { PUBLIC_CMS } from '$env/static/public';

/** @type {import('./$types').PageServerLoad} */
export async function load({ params }) {
    const fetchDoc = async () => {
        console.log('Doc Load Ran');
        const res = await fetch(`${PUBLIC_CMS}/api/docs?filters[slug][$eq]=${params.slug}&populate=*`);
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
        doc: await fetchDoc()
    };
}
