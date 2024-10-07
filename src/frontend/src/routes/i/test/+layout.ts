export const ssr = false;

import type { LayoutLoad } from "./$types";
import axios from "axios";
import { APIClient } from "$lib/utils/api";
import { PUBLIC_API } from "$env/static/public";

async function getUser(jwt) {
  let res = await APIClient.get(`/user/`, {
    withCredentials: true,
  });

  return res.data.data.user;
}

export const load: LayoutLoad = async ({ parent }) => {
  const { jwt } = await parent();
  console.log("server");

  const user = await getUser(jwt);

  return {
    user,
  };
};
