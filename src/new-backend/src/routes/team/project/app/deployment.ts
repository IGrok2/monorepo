import { z } from "zod";
import type { WResponse } from "../../../../wrpc/theoretical";
import { prisma } from "../../../../app";
import { DeploymentPhase } from "@prisma/client";

/// For creating manual deployments

export const DeploymentInput = z.object({
  app_id: z.string(),

  deployment: z
    .object({
      /// Specific deployment info
      source_type: z.enum(["DOCKER_HUB", "GITHUB_REPO", "PRIVATE_REGISTRY"]),
      build_source: z.enum(["NIXPACKS", "DOCKERFILE", "BUILDPACKS"]),
      /// If the source needs to be tweaked
      github: z.string().optional(),
      docker_image: z.string().optional(),
      private_registry: z.string().optional(),
      new_private_url: z.string().optional(),
      new_private_username: z.string().optional(),
      new_private_password: z.string().optional(),
    })
    .refine((data) => {
      // make sure at least one of the deployment methods is selected
      if (
        data.github === undefined &&
        data.docker_image === undefined &&
        data.private_registry === undefined
      ) {
        return false;
      }
    }),
});

export const DeploymentOutput = z.object({
  id: z.string(),
});

export async function Deployment({
  app_id,
  deployment,
}: z.infer<typeof DeploymentInput>): Promise<
  WResponse<typeof DeploymentOutput>
> {
  // find the app
  const app = await prisma.app.findUnique({
    where: {
      id: app_id,
    },
  });

  // if the app doesn't exist, return an error
  if (app === null) {
    return {
      error: {
        status: 400,
        message: "Either that app doesn't exist or you don't have access to it",
      },
    };
  }

  // create the new pending deployment
  const newDeployment = await prisma.deployment.create({
    data: {
      phase: DeploymentPhase.PENDING,
      buildSource: deployment.build_source,
      sourceType: deployment.source_type,
      appId: app.id,
    },
  });

  return {
    success: {
      id: newDeployment.id,
    },
  };
}
