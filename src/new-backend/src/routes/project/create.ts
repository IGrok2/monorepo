import { z } from "zod";
import type { WResponse } from "../../wrpc/theoretical";
import { prisma } from "../../app";

/// Create a new project

export const createProjectInput = z.object({
  name: z.string().min(1).max(255),
});

export const createProjectOutput = z.object({
  project_id: z.string(),
});

export async function createProject({
  name,
}: z.infer<typeof createProjectInput>): Promise<
  WResponse<typeof createProjectOutput>
> {
  /// Create a new project
  const project = await prisma.project.create({
    data: {
      name,
      notifications: {
        create: [
          {
            message: `Welcome to ${name}!`,
          },
        ],
      },
    },
  });

  /// If the project is null, throw an error
  if (project === null) {
    throw new Error("Failed to create project");
  }

  return {
    success: {
      project_id: project.id,
    },
  };
}
