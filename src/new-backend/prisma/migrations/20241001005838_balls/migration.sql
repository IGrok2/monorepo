/*
  Warnings:

  - Added the required column `status` to the `AppPlacement` table without a default value. This is not possible if the table is not empty.
  - Made the column `externalPort` on table `Port` required. This step will fail if there are existing NULL values in that column.

*/
-- CreateEnum
CREATE TYPE "AppStatus" AS ENUM ('DEPLOYING', 'DEPLOYED', 'FAILING');

-- DropForeignKey
ALTER TABLE "AppPlacement" DROP CONSTRAINT "AppPlacement_serverId_fkey";

-- AlterTable
ALTER TABLE "AppPlacement" ADD COLUMN     "appId" TEXT,
ADD COLUMN     "status" "AppStatus" NOT NULL,
ALTER COLUMN "internalIp" DROP NOT NULL,
ALTER COLUMN "serverId" DROP NOT NULL;

-- AlterTable
ALTER TABLE "Port" ALTER COLUMN "externalPort" SET NOT NULL;

-- AddForeignKey
ALTER TABLE "AppPlacement" ADD CONSTRAINT "AppPlacement_serverId_fkey" FOREIGN KEY ("serverId") REFERENCES "Server"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "AppPlacement" ADD CONSTRAINT "AppPlacement_appId_fkey" FOREIGN KEY ("appId") REFERENCES "App"("id") ON DELETE SET NULL ON UPDATE CASCADE;
