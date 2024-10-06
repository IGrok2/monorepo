/// The middleware for the project
/// This will be run before anything else in the project
/// It can choose to:
/// 1. Add data
/// 2. Reject the request
export async function projectMiddleware(
  req: Request,
  resolve: any,
  reject: any,
) {}
