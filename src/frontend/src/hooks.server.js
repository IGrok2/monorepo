import { sequence } from "@sveltejs/kit/hooks";
import { handleErrorWithSentry, sentryHandle } from "@sentry/sveltekit";
import * as Sentry from '@sentry/sveltekit';

// import { PUBLIC_SENTRY_ENV } from '$env/static/public';


Sentry.init({
  dsn: "https://6248544d420650211959aaf9061e277d@sentry.packetware.net/3",
  environment: "production",
  tracesSampleRate: 1.0,
  debug: false,

});

// If you have custom handlers, make sure to place them after `sentryHandle()` in the `sequence` function.
export const handle = sequence(sentryHandle());

// If you have a custom error handler, pass it to `handleErrorWithSentry`
export const handleError = handleErrorWithSentry();
