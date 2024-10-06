import { z } from "zod";
import type { WResponse } from "../../wrpc/theoretical";
import { createJwt } from "../../utils/jwt";
import { prisma } from "../../app";

/// These are the types this function will accept and return

export const LoginInput = z.object({
  email: z.string().email().min(5).max(255),
  password: z.string().min(8).max(255),
});

export const LoginOutput = z.object({
  token: z.string(),
});

/// Authenticate a user to the Packetware API
export async function Login({
  email,
  password,
}: z.infer<typeof LoginInput>): Promise<WResponse<typeof LoginOutput>> {
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

  // validate the password
  if (!(await Bun.password.verify(password, user.password_hash))) {
    return {
      error: {
        status: 400,
        message: "Wrong password",
      },
    };
  }

  const token = createJwt(user);

  return {
    success: {
      token,
    },
  };
}
