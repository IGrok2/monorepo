import { error } from "@sveltejs/kit";
import { PUBLIC_CMS } from "$env/static/public";

/** @type {import('./$types').PageServerLoad} */
export async function load({ params }) {
  const fetchPost = async () => {
    console.log(
      "Post Load Ran",
      `${PUBLIC_CMS}/api/posts?filters[slug][$eq]=${params.slug}&populate=*`,
    );
    const res = await fetch(
      `${PUBLIC_CMS}/api/posts?filters[slug][$eq]=${params.slug}&populate=*`,
    );
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
    post: await fetchPost(),
  };
}
