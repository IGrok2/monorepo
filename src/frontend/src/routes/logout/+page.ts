export const ssr = false;

import type { PageLoad } from "./$types";

export const load: PageLoad = () => {
  document.cookie = "jwt=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;";
  document.location = "/i/auth/login";
};
