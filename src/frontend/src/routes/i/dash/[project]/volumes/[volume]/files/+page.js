export const ssr = false;

import { error, fail, redirect } from "@sveltejs/kit";
import axios from "axios";
import { getJWT } from "$lib/utils/auth";
import { PUBLIC_API } from "$env/static/public";

/** @type {import('./$types').PageLoad} */
export async function load({ parent, url, params, cookies }) {
  console.log(PUBLIC_API);

  let path = url.searchParams.get("path");

  if (!path) {
    path = "";
  }

  const response = await axios.get(
    `${PUBLIC_API}/@/project/${params.project}/volume/666cd1bce12eec2dd280740b/files/list-directory`,
    {
      params: {
        path: `/srv${path}`,
      },
      headers: {
        Authorization: getJWT(),
      },
    },
  );

  return { files: response.data.files };
}
