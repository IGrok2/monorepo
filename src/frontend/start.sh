#!/bin/bash

npm i
npm run build
npm ci --omit dev
pm2 restart packetware
