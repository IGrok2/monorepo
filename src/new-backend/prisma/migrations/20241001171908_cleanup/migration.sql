/*
  Warnings:

  - Added the required column `buildSource` to the `Deployment` table without a default value. This is not possible if the table is not empty.
  - Added the required column `sourceType` to the `Deployment` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "Deployment" ADD COLUMN     "buildSource" "BuildSource" NOT NULL,
ADD COLUMN     "sourceType" "ContainerSourceType" NOT NULL;
