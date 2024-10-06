/// Handle incoming requests to make sure they fulfill the requirements of the API
/// The pipeline is as follows:
/// 1. Try and find the right path and method
/// 2. Safely validate the input with a timeout
/// 3. Run the endpoint
/// 4. Validate the output

import {
  BadPayload,
  InternalServerError,
  InvalidRequest,
  JsonResponse,
  NotFound,
  Unauthorized,
} from "../utils/resp";
import { timeout } from "../utils/timeout";
import { ServerRoutes } from "../utils/types";
import { wRPCserver } from "../wrpc/handler";

const server = new wRPCserver(ServerRoutes);

const allowedOrigins: string[] = ["http://dev.sive:3000", "https://packetware.net"];

export async function HandleRequest(req: Request) {
  const origin = req.headers.get("Origin");

  if (req.method === "OPTIONS") {
    // Handle CORS preflight request
    if (origin && allowedOrigins.includes(origin)) {
      return new Response(null, {
        status: 204, // No Content
        headers: {
          "Access-Control-Allow-Origin": origin, // Dynamically set the origin
          "Access-Control-Allow-Methods": "GET, POST, PUT, PATCH, DELETE, OPTIONS",
          "Access-Control-Allow-Headers": "Origin, Content-Type, Authorization",
          "Access-Control-Allow-Credentials": "true",
        },
      });
    } else {
      return new Response("Not allowed by CORS", {
        status: 403, // Forbidden
      });
    }
  }

  // Add CORS headers to the actual response
  const headers = {
    "Access-Control-Allow-Origin": origin || "*",
    "Access-Control-Allow-Methods": "GET, POST, PUT, PATCH, DELETE, OPTIONS",
    "Access-Control-Allow-Headers": "Origin, Content-Type, Authorization",
    "Access-Control-Allow-Credentials": "true",
  };

  if (req.url.length > 10000) {
    return InvalidRequest();
  }

  const start = performance.now();

  const resp = await server.handler(req);

  console.log(`Found a route in ${performance.now() - start}`);
  console.log(resp);

  //return resp !== null ? resp : NotFound();
  return resp
    ? new Response(resp.body, { status: resp.status, headers: { ...resp.headers, ...headers } })
    : NotFound();
}
