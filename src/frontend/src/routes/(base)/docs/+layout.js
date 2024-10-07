import { error } from "@sveltejs/kit";

import { PUBLIC_CMS } from '$env/static/public';

/** @type {import('./$types').LayoutLoad} */
export async function load({ fetch, params }) {
    const fetchSections = async () => {
        console.log('Sections Load Ran');
        const res = await fetch(`${PUBLIC_CMS}/api/sections?sort[1]=priority&populate=*`);
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
        sections: await fetchSections()
    };
}
