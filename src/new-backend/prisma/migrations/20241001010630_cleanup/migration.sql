/*
  Warnings:

  - You are about to drop the column `cpu` on the `AppPlacement` table. All the data in the column will be lost.
  - You are about to drop the column `maximumReplicas` on the `AppPlacement` table. All the data in the column will be lost.
  - You are about to drop the column `memory` on the `AppPlacement` table. All the data in the column will be lost.
  - You are about to drop the column `minimumReplicas` on the `AppPlacement` table. All the data in the column will be lost.
  - You are about to drop the column `network` on the `AppPlacement` table. All the data in the column will be lost.
  - Added the required column `appPlan` to the `AppPlacement` table without a default value. This is not possible if the table is not empty.
  - Made the column `appId` on table `AppPlacement` required. This step will fail if there are existing NULL values in that column.

*/
-- DropForeignKey
ALTER TABLE "AppPlacement" DROP CONSTRAINT "AppPlacement_appId_fkey";

-- AlterTable
ALTER TABLE "AppPlacement" DROP COLUMN "cpu",
DROP COLUMN "maximumReplicas",
DROP COLUMN "memory",
DROP COLUMN "minimumReplicas",
DROP COLUMN "network",
ADD COLUMN     "appPlan" "AppPlan" NOT NULL,
ALTER COLUMN "mountPoint" DROP NOT NULL,
ALTER COLUMN "appId" SET NOT NULL;

-- AddForeignKey
ALTER TABLE "AppPlacement" ADD CONSTRAINT "AppPlacement_appId_fkey" FOREIGN KEY ("appId") REFERENCES "App"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
