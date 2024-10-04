import { ErrorResp, Unauthorized } from "../utils/resp";
import { type Routes, type RoutingGroup, type WResponse } from "./theoretical";
import { timeout } from "../utils/timeout";
import { InternalServerError, BadPayload, JsonResponse } from "../utils/resp";

/// The handler response
type HandlerResponse = Response | null;

export class wRPCserver {
  routes: Routes;

  constructor(routes: Routes) {
    this.routes = routes;
  }

  async handler(req: Request): Promise<HandlerResponse> {
    const url = new URL(req.url);

    // find the right endpoint
    for (const group_name in this.routes.groups) {
      console.log("started for group");
      const resp = await this._process(
        req,
        this.routes.groups[group_name],
        url,
      );
      console.log("ended for group");

      if (resp !== null) return resp;
    }

    return null;
  }

  async _process(
    req: Request,
    route_group: RoutingGroup,
    url: URL,
  ): Promise<HandlerResponse> {
    // if the path starts with the prefix, run the middleware then find the correct path
    if (url.pathname.startsWith(route_group.prefix)) {
      // if there is middleware, run it
      if (route_group.middleware !== null) {
        try {
          // provide the middleware with the request and two functions to resolve and reject
          // changes to `req` shall persist
          await new Promise((resolve, reject) =>
            route_group.middleware(req, resolve, reject),
          );
        } catch (error) {
          console.error(error);
          return Unauthorized();
        }
      }

      // if there's routing groups downstream, check those too
      if (route_group.groups?.length || 0 > 0) {
        for (const group_name in route_group.groups) {
          const resp = this._process(req, route_group.groups[group_name], url);

          if (resp !== null) {
            // fantastic, we've caught a response from middleware or a handler
            return resp;
          }
        }
      } else {
        // this part won't ever execute if the first one does but this 'else' symbolically indicates traversing down the routing tree will go down one way or another
        // iterate through the paths under this group to find any matches
        for (const endpoint_name in route_group.routes) {
          const endpoint = route_group.routes[endpoint_name];

          // find if the path and request are compatible
          if (
            endpoint_name ===
              url.pathname.replaceAll(`${route_group.prefix}/`, "") &&
            endpoint.method === req.method
          ) {
            // we've found the right endpoint

            let input = {};

            // if there is any input, validate it
            // validation is intended to be fallible and recused
            if (endpoint.input !== null) {
              try {
                const json = await timeout(req.json(), 1000);

                if (!json) {
                  throw new Error(
                    "Endpoint wanted input, but request body was empty",
                  );
                }

                input = await timeout(endpoint.input.parseAsync(json), 1000);
              } catch (error) {
                return BadPayload(`${error}`);
              }
            }

            let resp: WResponse<any> = {};

            // now we know that if we are taking an input, it's valid. so now, we can run our function!
            try {
              // limit the amount of time we'll sit around and wait for a response
              resp = await timeout(endpoint.endpoint(input), 5000);
            } catch (error) {
              console.error(error);
              // TODO: use Sentry here
              return InternalServerError();
            }

            // validate the output
            try {
              // is the output successful? represent that
              if (resp.error) {
                return ErrorResp(resp.error.message, resp.error.status);
              }

              const obj = await timeout(
                endpoint.output.parseAsync(resp.success),
                5000,
              );

              return JsonResponse(obj, obj.status);
            } catch (error) {
              console.error(error);
              // TODO: use Sentry here
              return InternalServerError();
            }
          }
        }
      }
    }

    return null;
  }

  async _handleEndpoint(group: Group) {}
}
