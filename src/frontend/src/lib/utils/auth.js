export function getCookie(name) {
  const cookies = document.cookie.split("; ");
  const cookie = cookies.find((cookie) => cookie.startsWith(name + "="));

  if (cookie) {
    return cookie.split("=")[1];
  }

  return undefined;
}

export function getJWT() {
  const cookies = document.cookie.split("; ");
  const cookie = cookies.find((cookie) => cookie.startsWith("jwt" + "="));

  if (cookie) {
    return cookie.split("=")[1];
  }

  return undefined;
}
