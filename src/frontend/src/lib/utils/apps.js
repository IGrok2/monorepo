import { page } from "$app/stores";
import { getCookie } from "$lib/utils/auth";
import { PUBLIC_API } from "$env/static/public";
import axios from 'axios';

const token = getCookie("jwt");

export const appsAPIClient = axios.create({
    //baseURL: `${PUBLIC_API}/@/project/${$page.params.project}/container/${$page.params.id}`,
    baseURL: PUBLIC_API,
    headers: {
        Authorization: token,
        'Content-Type': 'application/json'
    }
});

// Update Containers
export async function updateApp(id, name, app_port, branch, subdirectory, auto_deploy) {
    const response = await fetch(
        `${PUBLIC_API}/@/container/${id}`,
        {
            method: "PATCH",
            headers: new Headers({
                "content-type": "application/json",
                Authorization: token,
            }),
            body: JSON.stringify({ name, app_port: Number(app_port), branch, subdirectory, auto_deploy }),
        },
    );

    return await response.json();
}

export async function updateName(id, name) {
    const response = await fetch(
        `${PUBLIC_API}/@/container/${id}/name`,
        {
            method: "PATCH",
            headers: new Headers({
                "content-type": "application/json",
                Authorization: token,
            }),
            body: JSON.stringify({ name }),
        },
    );

    return await response.json();
}

export async function updateBranch(id, branch) {
    const response = await fetch(
        `${PUBLIC_API}/@/container/${id}/branch`,
        {
            method: "PATCH",
            headers: new Headers({
                "content-type": "application/json",
                Authorization: token,
            }),
            body: JSON.stringify({ branch }),
        },
    );

    return await response.json();
}

export async function updatePort(id, port) {
    const response = await fetch(
        `${PUBLIC_API}/@/container/${id}/port`,
        {
            method: "PATCH",
            headers: new Headers({
                "content-type": "application/json",
                Authorization: token,
            }),
            body: JSON.stringify({ port }),
        },
    );

    return await response.json();
}

export async function updateSubdirectory(id, subdirectory) {
    const response = await fetch(
        `${PUBLIC_API}/@/container/${id}/subdirectory`,
        {
            method: "PATCH",
            headers: new Headers({
                "content-type": "application/json",
                Authorization: token,
            }),
            body: JSON.stringify({ subdirectory }),
        },
    );

    return await response.json();
}

// Delete App
export async function deleteApp(id) {
    const response = await fetch(
        `${PUBLIC_API}/@/container/${id}`,
        {
            method: "DELETE",
            headers: new Headers({
                "content-type": "application/json",
                Authorization: token,
            }),
        },
    );

    return await response.json();
}

export default appsAPIClient;
