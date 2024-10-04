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

export async function HandleRequest(req: Request) {
  if (req.url.length > 10000) {
    return InvalidRequest();
  }

  const start = performance.now();

  const resp = await server.handler(req);

  console.log(`Found a route in ${performance.now() - start}`);
  console.log(resp);

  return resp !== null ? resp : NotFound();
}
