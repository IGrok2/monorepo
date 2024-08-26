import { error } from "@sveltejs/kit";
import { PUBLIC_CMS } from '$env/static/public';

/** @type {import('./$types').PageServerLoad} */
export async function load() {
    const fetchPrivacy = async () => {
        console.log('TOS Load Ran');
        const res = await fetch(`${PUBLIC_CMS}/api/privacy-policy`);
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
        privacy: await fetchPrivacy()
    };
}
