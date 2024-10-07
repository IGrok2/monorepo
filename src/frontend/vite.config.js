import { sentrySvelteKit } from "@sentry/sveltekit";
import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [
    sentrySvelteKit({
      sourceMapsUploadOptions: {
        url: `https://sentry.packetware.net`,
        org: `packetware`,
        project: `frontend`,
        authToken: `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`,
      },
    }),
    sveltekit(),
  ],
});
