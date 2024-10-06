import { z } from "zod";
import type { WResponse } from "../../wrpc/theoretical";
import { createJwt } from "../../utils/jwt";
import { prisma } from "../../app";

/// These are the types this function will accept and return

export const VerifyEmailInput = z.object({
  code: z.string(),
});

export const VerifyEmailOutput = z.object({
  success: z.boolean(),
});

/// Verify the email
export async function VerifyEmail({
  code,
}: z.infer<typeof VerifyEmailInput>): Promise<
  WResponse<typeof VerifyEmailOutput>
> {
  // pull the user from the database
  const user = await prisma.user.findUnique({
    where: {
      email_token: code,
    },
  });

  if (user === null) {
    return {
      error: {
        status: 404,
        message: "No such code",
      },
    };
  }

  // make sure the code hasn't expired, they last 24 hours
  if (
    new Date().getTime() -
      new Date(user.email_verification_sent_at).getTime() >
    1000 * 60 * 60 * 24
  ) {
    return {
      error: {
        status: 400,
        message: "Code has expired",
      },
    };
  }

  await prisma.user.update({
    where: {
      id: user.id,
    },
    data: {
      email_token: null,
      email_verified: true,
      email_verified_at: new Date(),
    },
  });

  return {
    success: {
      success: true,
    },
  };
}
