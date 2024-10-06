import { z } from "zod";
import type { WResponse } from "../../../wrpc/theoretical";
import { prisma } from "../../../app";

/// Create a new project

export const createProjectInput = z.object({
  name: z
    .string()
    .min(1, { message: "Name must not be empty." })
    .max(255, { message: "Name must not exceed 255 characters." })
    .regex(/^[a-z0-9-_]+$/, { message: "Name must contain only lowercase letters, numbers, dashes, and underscores." }),
  team_id: z.string()
});

export const createProjectOutput = z.object({
  project_id: z.string(),
});

export async function createProject({
  name, team_id,
}: z.infer<typeof createProjectInput>): Promise<
  WResponse<typeof createProjectOutput>
> {
  /// Create a new project
  const project = await prisma.project.create({
    data: {
      name,
      team_id
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
