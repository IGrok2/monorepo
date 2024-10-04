-- CreateEnum
CREATE TYPE "Role" AS ENUM ('READONLY', 'STANDARD', 'ADMIN');

-- CreateEnum
CREATE TYPE "ContainerSourceType" AS ENUM ('DOCKER_HUB', 'GITHUB_REPO', 'PRIVATE_REGISTRY');

-- CreateEnum
CREATE TYPE "BuildSource" AS ENUM ('NIXPACKS', 'DOCKERFILE', 'BUILDPACKS');

-- CreateEnum
CREATE TYPE "PortType" AS ENUM ('PRIVATE', 'PUBLIC', 'HTTP');

-- CreateTable
CREATE TABLE "User" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "email" TEXT NOT NULL,
    "passwordHash" TEXT NOT NULL,
    "resetCode" TEXT,
    "expiresAt" TIMESTAMP(3),
    "usedAt" TIMESTAMP(3),
    "emailVerified" BOOLEAN NOT NULL DEFAULT false,
    "emailToken" TEXT,
    "emailVerificationMailSentAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "emailVerifiedAt" TIMESTAMP(3),
    "stripeCustomerId" TEXT,

    CONSTRAINT "User_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Notification" (
    "id" TEXT NOT NULL,
    "message" TEXT NOT NULL,
    "link" TEXT,
    "seen" BOOLEAN NOT NULL DEFAULT false,
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "userId" TEXT,
    "projectId" TEXT,

    CONSTRAINT "Notification_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Project" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,

    CONSTRAINT "Project_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Member" (
    "id" TEXT NOT NULL,
    "role" "Role" NOT NULL,
    "invitedAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "invitationAccepted" BOOLEAN NOT NULL DEFAULT false,
    "invitationAcceptedAt" TIMESTAMP(3),
    "userId" TEXT NOT NULL,

    CONSTRAINT "Member_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "GithubNamespace" (
    "id" TEXT NOT NULL,
    "installId" TEXT NOT NULL,
    "projectId" TEXT NOT NULL,

    CONSTRAINT "GithubNamespace_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Secret" (
    "id" TEXT NOT NULL,
    "key" TEXT NOT NULL,
    "value" TEXT NOT NULL,
    "projectId" TEXT NOT NULL,

    CONSTRAINT "Secret_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "App" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "containerSourceType" "ContainerSourceType" NOT NULL,
    "url" TEXT,
    "username" TEXT,
    "password" TEXT,
    "buildSource" "BuildSource" NOT NULL,

    CONSTRAINT "App_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Port" (
    "id" TEXT NOT NULL,
    "portType" "PortType" NOT NULL,
    "externalPort" INTEGER,
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "appId" TEXT,

    CONSTRAINT "Port_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "AppPlacement" (
    "id" TEXT NOT NULL,
    "minimumReplicas" INTEGER NOT NULL,
    "maximumReplicas" INTEGER NOT NULL,
    "cpu" INTEGER NOT NULL,
    "memory" INTEGER NOT NULL,
    "network" INTEGER NOT NULL,
    "mountPoint" TEXT NOT NULL,
    "internalIp" TEXT NOT NULL,
    "serverId" TEXT NOT NULL,

    CONSTRAINT "AppPlacement_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "PoP" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,

    CONSTRAINT "PoP_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Server" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "poPId" TEXT NOT NULL,

    CONSTRAINT "Server_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "PacketwareIp" (
    "Ip" TEXT NOT NULL,
    "serverId" TEXT NOT NULL,

    CONSTRAINT "PacketwareIp_pkey" PRIMARY KEY ("Ip")
);

-- CreateIndex
CREATE UNIQUE INDEX "User_email_key" ON "User"("email");

-- CreateIndex
CREATE UNIQUE INDEX "User_resetCode_key" ON "User"("resetCode");

-- CreateIndex
CREATE UNIQUE INDEX "User_emailToken_key" ON "User"("emailToken");

-- AddForeignKey
ALTER TABLE "Notification" ADD CONSTRAINT "Notification_userId_fkey" FOREIGN KEY ("userId") REFERENCES "User"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Notification" ADD CONSTRAINT "Notification_projectId_fkey" FOREIGN KEY ("projectId") REFERENCES "Project"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Member" ADD CONSTRAINT "Member_userId_fkey" FOREIGN KEY ("userId") REFERENCES "User"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "GithubNamespace" ADD CONSTRAINT "GithubNamespace_projectId_fkey" FOREIGN KEY ("projectId") REFERENCES "Project"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Secret" ADD CONSTRAINT "Secret_projectId_fkey" FOREIGN KEY ("projectId") REFERENCES "Project"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Port" ADD CONSTRAINT "Port_appId_fkey" FOREIGN KEY ("appId") REFERENCES "App"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "AppPlacement" ADD CONSTRAINT "AppPlacement_serverId_fkey" FOREIGN KEY ("serverId") REFERENCES "Server"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Server" ADD CONSTRAINT "Server_poPId_fkey" FOREIGN KEY ("poPId") REFERENCES "PoP"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "PacketwareIp" ADD CONSTRAINT "PacketwareIp_serverId_fkey" FOREIGN KEY ("serverId") REFERENCES "Server"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
