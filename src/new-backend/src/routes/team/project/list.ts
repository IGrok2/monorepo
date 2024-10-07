import { z } from "zod";
import type { WResponse } from "../../../wrpc/theoretical";
import { prisma } from "../../../app";

export const ListProjectsInput = z.object({
  team_id: z.string(),
});

export const ListProjectsOutput = z.object({
  projects: z.array(
    z.object({
      id: z.string(),
      name: z.string(),
    }),
  ),
});

/// List the projects associated with the team and user
export async function ListProjects(
  req: Request,
): Promise<WResponse<typeof ListProjectsOutput>> {
  /// Create a new project
  const projects = await prisma.project.findMany({
    where: {
      team_id: "cm1wzgu7y0000i32rf0g03m5w",
    },
  });

  if (!projects) {
    return {
      error: {
        status: 400,
        message: "Could not find project",
      },
    };
  }

  return {
    success: {
      projects,
    },
  };
}
