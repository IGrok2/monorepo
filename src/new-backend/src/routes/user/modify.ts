import { z } from "zod";
import type { WResponse } from "../../wrpc/theoretical";
import { prisma } from "../../app";

/// Modifying the user, like creating new projects or changing your email

export const ModifyUserInput = z.object({
  action: z.union([
    z.object({
      type: z.literal("create_project"),
      name: z.string().min(1).max(255),
    }),
  ]),
});

export const ModifyUserOutput = z.union([
  z.object({
    project_id: z.string(),
  }),
  z.object({
    success: z.boolean(),
  }),
]);

/// Modify the user - like creating a project, or emailing the user
export async function ModifyUser({
  action,
}: z.infer<typeof ModifyUserInput>): Promise<
  WResponse<typeof ModifyUserOutput>
> {
  switch (action.type) {
    case "create_project":
      /// Create a new project
      const project = await prisma.project.create({
        data: {
          name: action.name,
          notifications: {
            create: [
              {
                message: `Welcome to ${action.name}!`,
              },
            ],
          },
        },
      });

      if (!project) {
        throw new Error("Failed to create project");
      }

      return {
        success: {
          project_id: project.id,
        },
      };
  }

  return {
    error: {
      status: 400,
      message: "Invalid action type",
    },
  };
}
