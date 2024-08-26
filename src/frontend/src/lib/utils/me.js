import { getCookie } from "$lib/utils/auth";
import { PUBLIC_API } from "$env/static/public";
import axios from 'axios';

const token = getCookie("jwt");

export const meAPIClient = axios.create({
    baseURL: `${PUBLIC_API}/@/me`,
    headers: {
        Authorization: token,
        'Content-Type': 'application/json'
    }
});

export default meAPIClient;