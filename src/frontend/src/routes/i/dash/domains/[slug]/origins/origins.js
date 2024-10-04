import { getCookie } from "$lib/utils/auth";

export const newOrigin = async ({ origin, domain_info }) => {
    const token = getCookie("jwt");

    // Remove _id field from each origin in domain_info.origin_settings.origins.
    for (let i = 0; i < domain_info.origin_settings.length; i++) {
        domain_info.origin_settings[i].http2 = false;
        for (let j = 0; j < domain_info.origin_settings[i].origins.length; j++) {
            delete domain_info.origin_settings[i].origins[j]._id;

        }
    }

    // Combine all to update.
    let origin_settings = domain_info.origin_settings;
    if (origin_settings) {
        origin_settings.push(origin);
    } else {
        origin_settings = [origin];
    }

    const reqBody = {
        domain: domain_info._id,
        origin_settings: origin_settings,
        time: Date.now(),
        __foo_confirm: true,
    };

    console.log(reqBody);

    let response;
    try {
        const res = await fetch(`${PRIVATE_API}/@/project/${$page.params.project}/domain/${$page.params.id}/origins/update`, {
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

        response = await res.json();
        console.log(response);
    } catch (err) {
        return { success: false, message: err.message }
    }

    return { success: response.success, actions: response.actions, message: response.message, origin_settings: origin_settings };
}

export const editOrigin = async ({ request, params }) => {
    const token = getCookie("jwt");

    const domain = params.slug;

    const data = await request.formData();

    const domain_info = JSON.parse(data.get("domain_info"));
    const old_id = data.get("old_id");
    const origin = JSON.parse(data.get("origin"));

    // Replaced the old origin with the edited data.
    for (let i = 0; i < domain_info.origin_settings.length; i++) {
        if (domain_info.origin_settings[i]._id === old_id) {
            domain_info.origin_settings[i] = origin;
        }
        domain_info.origin_settings[i].http2 = false;
    }

    // Remove _id field from each origin in domain_info.origin_settings.origins.
    for (let i = 0; i < domain_info.origin_settings.length; i++) {
        for (let j = 0; j < domain_info.origin_settings[i].origins.length; j++) {
            delete domain_info.origin_settings[i].origins[j]._id;
        }
    }

    const reqBody = {
        domain: domain,
        origin_settings: domain_info.origin_settings,
        time: Date.now(),
        __foo_confirm: true,
    };

    console.log(reqBody);

    let response;
    try {
        const res = await fetch(`${PRIVATE_API}/@/project/${$page.params.project}/domain/${$page.params.id}/origins/update`, {
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

        response = await res.json();
        console.log(response);
    } catch (err) {
        return { success: false, message: err.message }
    }

    return { success: response.success, actions: response.actions, message: response.message, origin_settings: domain_info.origin_settings };
}

export const deleteOrigin = async ({ request, params }) => {
    const token = getCookie("jwt");

    const domain = params.slug;

    const data = await request.formData();

    const deleted_id = data.get("deleted_id");

    const domain_info = JSON.parse(data.get("domain_info"));
    console.log(domain_info);
    // Remove _id field from each origin in domain_info.origin_settings.origins.
    for (let i = 0; i < domain_info.origin_settings.length; i++) {
        for (let j = 0; j < domain_info.origin_settings[i].origins.length; j++) {
            delete domain_info.origin_settings[i].origins[j]._id;
        }
    }

    // Delete the origin to be deleted.
    for (let i = 0; i < domain_info.origin_settings.length; i++) {
        if (domain_info.origin_settings[i]._id === deleted_id) {
            domain_info.origin_settings.splice(i, 1);
        }
    }

    const reqBody = {
        domain: domain,
        origin_settings: domain_info.origin_settings,
        time: Date.now(),
        __foo_confirm: true,
    };

    let response;
    try {
        const res = await fetch(`${PRIVATE_API}/@/project/${$page.params.project}/domain/${$page.params.id}/origins/update`, {
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

        response = await res.json();
        console.log(response);
    } catch (err) {
        return { success: false, message: err.message }
    }

    return { success: response.success, actions: response.actions, message: response.message, origin_settings: domain_info.origin_settings };
}