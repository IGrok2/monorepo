export const ssr = false;

import { PUBLIC_API } from "$env/static/public";
import axios from 'axios';

export const APIClient = axios.create({
    baseURL: PUBLIC_API,
    withCredentials: true,
})

export default APIClient;
