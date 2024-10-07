import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = ({ cookies }) => {
  return { jwt: cookies.get("jwt"), team: cookies.get("team") };
};
