export const ssr = false;

import type { LayoutLoad } from "./$types";
import { redirect } from "@sveltejs/kit";
import APIClient from "$lib/utils/api";

export const load: LayoutLoad = async ({ parent }) => {
  const { jwt } = await parent();
  if (!jwt) {
    // If there is no token we want them to login
    throw redirect(307, "/i/auth/login");
  }
  //let res2 = await APIClient.get(`/project/test-project`);

  try {
    let res = await APIClient.get(`/i/user`);

    return {
      user: res.data.data.user,
    };
  } catch (err) {
    console.log("err: ", err);
    if (err.response.status === 401) {
      console.log("unauthorized");
      document.cookie = "jwt=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;";
      throw redirect(307, "/i/auth/login");
    }
  }
};
