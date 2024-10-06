/// Create a Packetware application from a GitHub repo or a Docker image

import { number, z } from "zod";
import type { WResponse } from "../../../../wrpc/theoretical";
import { prisma } from "../../../../app";
import { AppStatus, DeploymentPhase } from "@prisma/client";

export const AppsInput = z.object({
  name: z.string(),
  deployment_info: z.array(
    z.object({
      region: z.enum(["LAX", "DFW", "ASH", "AMS"]),
      app_plan: z.enum([
        "SO_YOU_BEGIN",
        "EMBERS_CALL",
        "NEON_SKYLINE",
        "SOUL_FORGE",
        "CHROME_LEAP",
        "PULSE_OF_COSMOS",
      ]),
    }),
  ),
  ports: z.object({
    port_type: z.enum(["PRIVATE", "PUBLIC", "HTTP"]),
    external_port: z.number().max(65535),
  }),
  container_source: z
    .object({
      source_type: z.enum(["DOCKER_HUB", "GITHUB_REPO", "PRIVATE_REGISTRY"]),
      build_source: z.enum(["NIXPACKS", "DOCKERFILE", "BUILDPACKS"]),
      url: z.string().optional(),
      username: z.string().optional(),
      password: z.string().optional(),
    })
    .refine(
      (data) => {
        if (data.source_type === "PRIVATE_REGISTRY") {
          return data.url && data.username && data.password;
        }
        return true;
      },
      {
        message:
          "URL, username, and password are required for private registry",
        path: ["container_source"],
      },
    ),
});

export const AppsOutput = z.object({
  id: z.string(),
});

export async function CreateApp({
  name,
  deployment_info,
  container_source,
  ports,
}: z.infer<typeof AppsInput>): Promise<WResponse<typeof AppsOutput>> {
  const app = await prisma.app.create({
    data: {
      name,
      containerSourceType: container_source.source_type,
      buildSource: container_source.build_source,
      ports: {
        create: [
          {
            portType: ports.port_type,
            externalPort: ports.external_port,
          },
        ],
      },
      placements: {
        create: deployment_info.map((info) => ({
          region: info.region,
          status: AppStatus.NOT_DEPLOYED,
          appPlan: info.app_plan,
        })),
      },
      deployments: {
        create: [
          {
            phase: DeploymentPhase.PENDING,
            buildSource: container_source.build_source,
            sourceType: container_source.source_type,
          },
        ],
      },
    },
  });

  if (app === null) {
    throw new Error("failed to create app");
  }

  return {
    success: {
      id: app.id,
    },
  };
}
