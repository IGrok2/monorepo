export const API_VERSION = getEnv("API_VERSION");
export const JSON_SIGNING_TOKEN = getEnv("JSON_SIGNING_TOKEN");
export const RESEND_EMAIL_KEY = getEnv("RESEND_EMAIL_KEY");
export const BASE_URL = getEnv("BASE_URL");

export function getEnv(name: string): string {
  const value = process.env[name];

  if (value === undefined || value === "") {
    throw new Error(`Missing environment variable: ${name}`);
  }

  return value;
}
