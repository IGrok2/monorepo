/*
  Warnings:

  - You are about to drop the column `expiresAt` on the `User` table. All the data in the column will be lost.
  - You are about to drop the column `resetCode` on the `User` table. All the data in the column will be lost.
  - A unique constraint covering the columns `[passwordResetCode]` on the table `User` will be added. If there are existing duplicate values, this will fail.

*/
-- DropIndex
DROP INDEX "User_resetCode_key";

-- AlterTable
ALTER TABLE "User" DROP COLUMN "expiresAt",
DROP COLUMN "resetCode",
ADD COLUMN     "passwordResetCode" TEXT,
ADD COLUMN     "passwordResetCodeSentAt" TIMESTAMP(3);

-- CreateIndex
CREATE UNIQUE INDEX "User_passwordResetCode_key" ON "User"("passwordResetCode");
