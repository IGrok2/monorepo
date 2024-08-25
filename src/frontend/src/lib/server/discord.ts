import { WebhookClient } from 'discord.js';

import { PRIVATE_DISCORD_NOTIFICATION_WEBHOOK } from '$env/static/private';

export const notificationWebhookClient = new WebhookClient({ url: PRIVATE_DISCORD_NOTIFICATION_WEBHOOK });
