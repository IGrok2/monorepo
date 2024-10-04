import { z } from "zod";
import jwt from "jsonwebtoken";
import { prisma } from "../app";
import { JSON_SIGNING_TOKEN } from "./env";

const jwtRequest = z.object({
  id: z.string(),
  passwordHash: z.string(),
});

/// Create an encrypted JSON web token from a request
export function createJwt(raw_request: z.infer<typeof jwtRequest>): string {
  // purposefully throw an error if the schema passed is invalid
  const request = jwtRequest.parse(raw_request);

  return jwt.sign(request, JSON_SIGNING_TOKEN, {
    expiresIn: "7d",
  });
}

/// Validate an encrypted JSON web token
export async function validateJwt(jwt_str: string): Promise<boolean> {
  try {
    // verify the JWT object, will throw an error if unsuccessful
    const decoded = jwt.verify(jwt_str, JSON_SIGNING_TOKEN);

    const { data, success } = jwtRequest.safeParse(decoded);

    // if the JWT couldn't be decrypted or expired, it's no longer valid
    if (!success) {
      return false;
    }

    // find the user
    const user = await prisma.user.findUnique({
      where: {
        id: data.id,
      },
    });

    // if the user doesn't exist, the JWT is invalid
    if (user === null) {
      return false;
    }

    // validate the password has not changed
    if (user.passwordHash !== data.passwordHash) {
      // keys are invalid across password changes
      return false;
    }

    return true;
  } catch (_e) {
    return false;
  }
}
