import { error, fail, json } from "@sveltejs/kit";

import { PRIVATE_API } from "$env/static/private";

/** @type {import('./$types').PageServerLoad} */
export async function load({ params, cookies }) {
    const token = cookies.get(`jwt`);

    const response = await fetch(`${PRIVATE_API}/@/me`, {
        method: "GET",
        headers: new Headers({
            "content-type": "application/json",
            Authorization: token,
        }),
    });

    const profile = await response.json();

    return {
        profile,
    };
}

/** @type {import('./$types').Actions} */
export const actions = {
    changeInformation: async ({ cookies, request }) => {
        const token = cookies.get(`jwt`);

        const data = await request.formData();
        const newName = data.get("name");
        const newEmail = data.get("email");
        const password = data.get("verify_password");

        if (password.length < 1) {
            throw error(400, { success: false, message: "Password is required" });
        }

        const reqBody = { password: password };

        if (newName !== "") {
            reqBody.newName = newName;
        }
        if (newEmail !== "") {
            reqBody.newEmail = newEmail;
        }

        console.log(reqBody);

        const res = await fetch(`${PRIVATE_API}/@/me/change-information`, {
            method: "POST",
            headers: new Headers({
                "content-type": "application/json",
                Authorization: token,
            }),
            body: JSON.stringify(reqBody),
        });

        const response = await res.json();
        console.log(response);

        return { success: response.success, actions: response.actions, message: response.message };
    },
    changePassword: async ({ cookies, request }) => {
        const token = cookies.get(`jwt`);

        const data = await request.formData();
        const password = data.get("current_password");
        const newPassword = data.get("new_password");
        const confirmPassword = data.get("confirm_password");

        if (newPassword !== confirmPassword) {
            throw fail(400, { success: false, message: "Passwords do not match" });
        }

        const res = await fetch(`${PRIVATE_API}/@/me/change-information`, {
            method: "POST",
            headers: new Headers({
                "content-type": "application/json",
                Authorization: token,
            }),
            body: JSON.stringify({
                password: password,
                newPassword: newPassword,
            }),
        });

        const response = await res.json();
        console.log(response);

        return { success: response.success, actions: response.actions, message: response.message };
    },
};
