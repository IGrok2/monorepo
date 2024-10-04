import { page } from '$app/stores';
import { getCookie } from "$lib/utils/auth";

import { PUBLIC_API } from "$env/static/public";

//Helper functions.
export const updateOrigin = async (domain_info) => {
    const token = getCookie("jwt");

    const reqBody = {
        domain: domain_info._id,
        origin_settings: domain_info.origin_settings,
        time: Date.now(),
        __foo_confirm: true,
    };

    console.log(reqBody);

    const res = await fetch(`${PUBLIC_API}/@/project/${$page.params.project}/domain/${$page.params.id}/origins/update`, {
        method: "POST",
        headers: new Headers({
            "content-type": "application/json",
            Authorization: token,
        }),
        body: JSON.stringify(reqBody),
    });

    if (!res.ok) {
        console.log("Error status", res.status);
        const text = await res.text();
        console.log("Response text", text);
        return;
    }

    return res.json();
}

// Frontend functions.

// Function to check if a single origin is valid in order to be added (show submit button).
export function validateOrigin(origin) {
    const isIdNotEmpty = origin._id.trim() !== "";
    const isTimeoutNumber = origin.timeout !== null;
    const isDestNotEmpty = origin.origins.every((origin) => {
        const destEmpty = origin.url.trim() !== "";
        return destEmpty;
    });

    const valid = isIdNotEmpty && isTimeoutNumber && isDestNotEmpty;

    return valid;
}

export const newOrigin = async (newOrigin, domain_info) => {
    // Remove _id field from each origin in domain_info.origin_settings.origins.
    for (let i = 0; i < domain_info.origin_settings.length; i++) {
        domain_info.origin_settings[i].http2 = false;
        for (let j = 0; j < domain_info.origin_settings[i].origins.length; j++) {
            delete domain_info.origin_settings[i].origins[j]._id;
        }
    }

    // Combine all to update.
    if (domain_info.origin_settings) {
        domain_info.origin_settings.push(newOrigin);
    } else {
        domain_info.origin_settings = [newOrigin];
    }

    let response;
    try {
        response = await updateOrigin(domain_info);
        console.log(response);
    } catch (err) {
        return { success: false, message: err.message }
    }

    return { success: response.success, actions: response.actions, message: response.message, origin_settings: domain_info.origin_settings };
}

export const editOrigin = async (editedOrigin, domain_info) => {
    // Replaced the old origin with the edited data.
    for (let i = 0; i < domain_info.origin_settings.length; i++) {
        if (domain_info.origin_settings[i]._id === editedOrigin._id) {
            domain_info.origin_settings[i] = editedOrigin;
        }
        domain_info.origin_settings[i].http2 = false;
    }

    // Remove _id field from each origin in domain_info.origin_settings.origins.
    for (let i = 0; i < domain_info.origin_settings.length; i++) {
        for (let j = 0; j < domain_info.origin_settings[i].origins.length; j++) {
            delete domain_info.origin_settings[i].origins[j]._id;
        }
    }

    let response;
    try {
        response = await updateOrigin(domain_info);
        console.log(response);
    } catch (err) {
        return { success: false, message: err.message }
    }

    return { success: response.success, actions: response.actions, message: response.message, origin_settings: domain_info.origin_settings };
}

export const deleteOrigin = async (deletedOrigin, domain_info) => {
    // Remove _id field from each origin in domain_info.origin_settings.origins.
    for (let i = 0; i < domain_info.origin_settings.length; i++) {
        for (let j = 0; j < domain_info.origin_settings[i].origins.length; j++) {
            delete domain_info.origin_settings[i].origins[j]._id;
        }
    }

    // Delete the origin to be deleted.
    for (let i = 0; i < domain_info.origin_settings.length; i++) {
        if (domain_info.origin_settings[i]._id === deletedOrigin._id) {
            domain_info.origin_settings.splice(i, 1);
        }
    }

    let response;
    try {
        response = await updateOrigin(domain_info);
        console.log(response);
    } catch (err) {
        return { success: false, message: err.message }
    }

    return { success: response.success, actions: response.actions, message: response.message, origin_settings: domain_info.origin_settings };
}