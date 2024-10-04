/*
  Warnings:

  - Added the required column `region` to the `AppPlacement` table without a default value. This is not possible if the table is not empty.
  - Changed the type of `name` on the `PoP` table. No cast exists, the column would be dropped and recreated, which cannot be done if there is data, since the column is required.

*/
-- CreateEnum
CREATE TYPE "Region" AS ENUM ('DFW', 'ASH', 'LAX', 'AMS');

-- AlterTable
ALTER TABLE "AppPlacement" ADD COLUMN     "region" "Region" NOT NULL;

-- AlterTable
ALTER TABLE "PoP" DROP COLUMN "name",
ADD COLUMN     "name" "Region" NOT NULL;
