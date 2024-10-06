export const ssr = false;

import type { PageLoad } from './$types';
import APIClient from "$lib/utils/api";

async function getProjects() {
	let res = await APIClient.get(`/user/projects`)

	return res.data.data.projects;
}


export const load: PageLoad = async ({ parent }) => {
	const projects = await getProjects()

	return {
		projects: {}
	};
};
