import { z } from "zod";
import { createJwt } from "../../utils/jwt";
import type { WResponse } from "../../wrpc/theoretical";
import { prisma } from "../../app";
import { sendUserWelcomeEmail, sendVerifyUserEmail } from "../../email/sender";
import { BASE_URL } from "../../utils/env";

export const RegisterInput = z.object({
  name: z.string().max(65),
  email: z.string().email().min(5).max(255),
  password: z.string().min(8).max(255),
});

export const RegisterOutput = z.object({
  message: z.string(),
  token: z.string(),
});

/// Register a new Packetware user in the database
export async function Register({
  name,
  email,
  password,
}: z.infer<typeof RegisterInput>): Promise<WResponse<typeof RegisterOutput>> {
  // make sure the email address isn't in use
  const count = await prisma.user.count({
    where: {
      email,
    },
  });

  if (count > 0) {
    return {
      error: {
        status: 409,
        message: "This email is fortunate enough to have an account",
      },
    };
  }

  // register the user
  const user = await prisma.user.create({
    data: {
      name,
      email,
      passwordHash: await Bun.password.hash(password),
      notifications: {
        create: [
          {
            message: "Welcome to Packetware!",
            seen: false,
            createdAt: new Date(),
          },
        ],
      },
    },
  });

  // TODO: push this to nodemailer so the user can receive the welcome email and the "please verify your email" email!

  const jwt = createJwt(user);

  // send the user a welcome email
  await sendUserWelcomeEmail(email, { name });
  await sendVerifyUserEmail(email, {
    name,
    verifyUrl: BASE_URL + "/verify?token=" + user.emailToken,
  });

  return { success: { message: "Welcome to Packetware!", token: jwt } };
}
