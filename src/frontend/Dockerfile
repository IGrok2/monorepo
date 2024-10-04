FROM node:18-alpine AS builder
RUN mkdir -p /app
WORKDIR /app
COPY package*.json /app
RUN npm ci
COPY . .

ARG PUBLIC_API
ARG PRIVATE_API
ARG PUBLIC_CMS
ARG SENTRY_DSN
ARG PUBLIC_SENTRY_ENV
ARG PRIVATE_DISCORD_NOTIFICATION_WEBHOOK

ENV PUBLIC_API=${PUBLIC_API}
ENV PRIVATE_API=${PRIAVTE_API}
ENV PUBLIC_CMS=${PUBLIC_CMS}
ENV SENTRY_DSN=${SENTRY_DSN}
ENV PUBLIC_SENTRY_ENV=${PUBLIC_SENTRY_ENV}
ENV PRIVATE_DISCORD_NOTIFICATION_WEBHOOK=${PRIVATE_DISCORD_NOTIFICATION_WEBHOOK}

RUN npm run build
RUN npm prune --production

FROM node:18-alpine

RUN adduser -D nodeuser
RUN mkdir -p /app
RUN chown nodeuser:nodeuser /app
USER nodeuser
WORKDIR /app
COPY --from=builder --chown=nodeuser:nodeuser /app/build build/
COPY --from=builder --chown=nodeuser:nodeuser /app/node_modules node_modules/
COPY package.json .

EXPOSE 3000
CMD [ "node", "build" ]