import APIClient from "$lib/utils/api";

async function getProject(slug) {
  try {
    let res = await APIClient.get(`/project/${slug}/`);

    return res.data.data.project;
  } catch (err) {
    console.log("err: ", err);
  }
}

async function getRegions(slug) {
  try {
    let res = await APIClient.get(`/project/${slug}/regions`);

    return res.data.data.regions;
  } catch (err) {
    console.log("err: ", err);
  }
}

/** @type {import('./$types').LayoutLoad} */
export async function load({ params }) {
  return {
    projects: await getProject(params.project),
    regions: await getRegions(params.project),
  };
}
