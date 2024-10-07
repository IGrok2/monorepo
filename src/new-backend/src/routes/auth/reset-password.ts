import { z } from "zod";
import type { WResponse } from "../../wrpc/theoretical";
import { createJwt } from "../../utils/jwt";
import { prisma } from "../../app";
import { randomBytes } from "crypto";
import { sendResetPasswordEmail } from "../../email/sender";
import { BASE_URL } from "../../utils/env";

export const ResetPasswordSendInput = z.object({
  email: z.string().email().min(5).max(255),
});

export const ResetPasswordSendOutput = z.object({
  success: z.boolean(),
});

/// Send the request to the user to reset their password
export async function ResetPasswordSend({
  email,
}: z.infer<typeof ResetPasswordSendInput>): Promise<
  WResponse<typeof ResetPasswordSendOutput>
> {
  // pull the user from the database
  const user = await prisma.user.findUnique({
    where: {
      email,
    },
  });

  if (user === null) {
    return {
      error: {
        status: 400,
        message: "No account origninating from that email",
      },
    };
  }

  const code = randomBytes(100).toString("hex").slice(0, 100);

  // create a password reset token on the user model
  await prisma.user.update({
    where: {
      id: user.id,
    },
    data: {
      password_reset_code: code,
      password_reset_code_sent_at: new Date(),
    },
  });

  // send the email to the user
  await sendResetPasswordEmail(user.email, {
    name: user.name,
    resetUrl: BASE_URL + "/i/auth/reset-password/" + code,
  });

  return {
    success: {
      success: true,
    },
  };
}

export const ResetPasswordCompleteInput = z.object({
  token: z.string(),
  new_password: z.string().min(8).max(255),
});

export const ResetPasswordCompleteOutput = z.object({
  success: z.boolean(),
});

export async function ResetPasswordComplete({
  token,
  new_password,
}: z.infer<typeof ResetPasswordCompleteInput>): Promise<
  WResponse<typeof ResetPasswordCompleteOutput>
> {
  console.log("hello");
  // pull the user from the database
  const user = await prisma.user.findUnique({
    where: {
      password_reset_code: token,
    },
  });

  if (user === null) {
    return {
      error: {
        status: 400,
        message: "No such token",
      },
    };
  }

  // make sure the code hasn't expired, they last 15 minutes
  if (
    new Date().getTime() -
      new Date(user.password_reset_code_sent_at ?? 0).getTime() >
    1000 * 60 * 15
  ) {
    return {
      error: {
        status: 400,
        message: "Code has expired",
      },
    };
  }
  console.log("hello");
  console.log(
    await prisma.user.update({
      where: {
        id: user.id,
      },
      data: {
        password_hash: await Bun.password.hash(new_password),
        password_reset_code: null,
        notifications: {
          create: [
            {
              message: "Your password was reset",
            },
          ],
        },
      },
    }),
  );

  return {
    success: {
      success: true,
    },
  };
}
