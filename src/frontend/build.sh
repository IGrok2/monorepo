#!/bin/bash

env_file=".env"

# Check if the .env file exists
if [ -f "$env_file" ]; then
    # Source (read and execute) the .env file
    source "$env_file"
    echo "Environment variables from $env_file have been loaded."
else
    echo "Error: $env_file not found."
    exit 1
fi

echo "PUBLIC_API: $PUBLIC_API"
echo "PRIVATE_API: $PRIVATE_API"
echo "PUBLIC_CMS: $PUBLIC_CMS"
echo "SENTRY_DSN: $SENTRY_DSN"
echo "PUBLIC_SENTRY_ENV: $PUBLIC_SENTRY_ENV"
echo "PRIVATE_DISCORD_NOTIFICATION_WEBHOOK: $PRIVATE_DISCORD_NOTIFICATION_WEBHOOK"

docker build \
  --build-arg PUBLIC_API=$PUBLIC_API \
  --build-arg PRIVATE_API=$PRIVATE_API \
  --build-arg PUBLIC_CMS=$PUBLIC_CMS \
  --build-arg SENTRY_DSN=$SENTRY_DSN \
  --build-arg PUBLIC_SENTRY_ENV=$PUBLIC_SENTRY_ENV \
  --build-arg PRIVATE_DISCORD_NOTIFICATION_WEBHOOK=$PRIVATE_DISCORD_NOTIFICATION_WEBHOOK \
  -t big-baller .
