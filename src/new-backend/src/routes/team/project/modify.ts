import { z } from "zod";
import type { WResponse } from "../../../wrpc/theoretical";

/// Modifying the project, like creating new apps or adding members

export const modifyProjectInput = z.object({
  project_id: z.string(),
  action: z.union([z.literal("create_app"), z.literal("add_member")]),
});

export const modifyProjectOutput = z.object({
  success: z.boolean(),
});

export async function modifyProject({
  project_id,
  action,
}: z.infer<typeof modifyProjectInput>): Promise<
  WResponse<typeof modifyProjectOutput>
> {}
