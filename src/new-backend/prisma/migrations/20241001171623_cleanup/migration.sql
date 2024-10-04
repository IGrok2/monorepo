-- CreateEnum
CREATE TYPE "DeploymentPhase" AS ENUM ('PENDING', 'BUILDING', 'DEPLOYING', 'SUCCESS', 'FAILED');

-- CreateTable
CREATE TABLE "Deployment" (
    "id" TEXT NOT NULL,
    "phase" "DeploymentPhase" NOT NULL,
    "appId" TEXT NOT NULL,
    "startedAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "finishedAt" TIMESTAMP(3),

    CONSTRAINT "Deployment_pkey" PRIMARY KEY ("id")
);

-- AddForeignKey
ALTER TABLE "Deployment" ADD CONSTRAINT "Deployment_appId_fkey" FOREIGN KEY ("appId") REFERENCES "App"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
