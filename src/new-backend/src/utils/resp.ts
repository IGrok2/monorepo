import { API_VERSION } from "./env";

/// Generic function to generate a JSON response with a status code
export function JsonResponse(json: object, status: number = 200): Response {
  const stringified = JSON.stringify({
    _meta: {
      api_version: API_VERSION,
      status,
    },
    data: json,
  });

  return new Response(stringified, {
    status,
    headers: {
      "content-type": "application/json",
    },
  });
}

/// Helper function to return a 404 response
export function NotFound(): Response {
  return JsonResponse(
    {
      message: "Not found",
    },
    404,
  );
}

/// Helper function to indicate the request was invalid
export function InvalidRequest(): Response {
  return JsonResponse(
    {
      message: "Invalid request",
    },
    400,
  );
}

/// Helper function to indicate the request was unauthorized
export function Unauthorized(): Response {
  return JsonResponse(
    {
      message: "Unauthorized",
    },
    401,
  );
}

/// Helper function for when the egress type validation fails
export function InvalidEgressType(): Response {
  return JsonResponse(
    {
      message:
        "We're sorry, but we weren't able to prepare the data in the format expected.",
    },
    500,
  );
}

/// Helper function to indicate the handler failed
export function InternalServerError(): Response {
  return JsonResponse(
    {
      message:
        "Internal server error: this error has been reported. Try again later.",
    },
    500,
  );
}

/// Helper function to indicate the request failed payload validation
export function BadPayload(reason: string): Response {
  return JsonResponse(
    {
      message: `Failed validation: ${reason}`,
    },
    400,
  );
}

/// Helper function to indicate there was an error with the request
export function ErrorResp(error: string, code: number): Response {
  return JsonResponse(
    {
      error: {
        message: error,
      },
    },
    code,
  );
}
