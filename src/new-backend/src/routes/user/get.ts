import { z } from "zod";
import type { WResponse } from "../../wrpc/theoretical";
import { prisma } from "../../app";

export const GetUserInput = null;

export const GetUserOutput = z.object({
  user: z.object({
    id: z.string(),
    name: z.string(),
    email: z.string(),
  })
});

/// Get the current authenticated users email and id
export async function GetUser(req: Request): Promise<
  WResponse<typeof GetUserOutput>
> {
  /// Create a new project
  const user = await prisma.user.findUnique({
    where: {
      id: "cm1wzgu7y0000i32rf0g03m5w"
    }
  });

  if (!user) {
    //throw new Error("Failed to find user");
    return {
      error: {
        status: 400,
        message: "Could not find user",
      },
    };
  }

  return {
    success: {
      user
    },
  };



}
