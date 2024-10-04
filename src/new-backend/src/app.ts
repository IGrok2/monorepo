import { PrismaClient } from "@prisma/client";
import { HandleRequest } from "./routes/handler";

export const prisma = new PrismaClient();

async function main() {
  console.log("Starting the Packetware backend ...");
  // add sentry

  // create bun server
  Bun.serve({
    port: 3030,
    static: {
      "/health": new Response("OK!"),
    },

    async fetch(req: Request) {
      return await HandleRequest(req);
    },
  });
}

main();
