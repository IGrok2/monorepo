import { page } from "$app/stores";
import { getCookie } from "$lib/utils/auth";
import { PUBLIC_API } from "$env/static/public";
import axios from 'axios';

const token = getCookie("jwt");

export const domainsAPIClient = axios.create({
    //baseURL: `${PUBLIC_API}/@/project/${$page.params.project}/domain/${$page.params.slug}`,
    baseURL: PUBLIC_API,
    headers: {
        Authorization: token,
        'Content-Type': 'application/json'
    }
});

export default domainsAPIClient;