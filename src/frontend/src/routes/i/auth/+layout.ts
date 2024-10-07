import type { LayoutLoad } from "./$types";
import { redirect } from "@sveltejs/kit";

export const load: LayoutLoad = async ({ parent }) => {
  const { jwt } = await parent();
  console.log(await parent());

  if (jwt) {
    throw redirect(307, "/i/dash");
  }
};
