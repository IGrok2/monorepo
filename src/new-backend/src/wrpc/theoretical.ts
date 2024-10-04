/// The types associated with the Packetware server

import { z, ZodType } from "zod";
import { Login, LoginInput, LoginOutput } from "../routes/auth/login";

/// A batch of routes which contain routing groups
export interface Routes {
  /// A group is a collection of groups or routes
  groups: RoutingGroup[];
}

export interface RoutingGroup {
  /// The middleware function for the group will be run pre-endpoint and either resolve or reject the request
  middleware: Function | null;
  /// Routes will start with this prefix
  prefix: string;
  /// Optionally, the opportunity for more abstractions (middleware etc) over the routes
  groups?: RoutingGroup[];
  /// Optionally, all the way downstream at route level
  routes?: Record<string, ServerEndpointType<any, any>>;
}

/// Type alias for the endpoint function signature
type EndpointFunction<
  I extends ZodType<any, any>,
  O extends ZodType<any, any>,
> = (input: z.infer<I>) => Promise<WResponse<O>>;

/// The implementation of a specific endpoint
export interface ServerEndpointType<
  I extends ZodType<any, any>,
  O extends ZodType<any, any>,
> {
  /// The HTTP method to reach this endpoint over depending on whether data is being sent or not
  method: "GET" | "POST";
  /// The function endpoint to call that will take the input and return the output
  endpoint: EndpointFunction<I, O>;
  /// The (optional if GET) Zod typechecked input object to be validated
  input: z.ZodObject<any, any> | null;
  /// The egress typechecked output object to be validated
  output: z.ZodObject<any, any>;
}

/// The type of a response
/// Typescript equivalent of anyhow
export interface WResponse<S extends ZodType<any, any>> {
  /// A successful response (status code 200)
  success?: z.infer<S>;
  /// An errornous response
  error?: {
    status: number;
    message: string;
  };
}
