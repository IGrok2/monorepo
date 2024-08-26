use crate::{
    buckets::private::PrivateKeys,
    handler::pipeline::bot_management::models::{
        Bots,
        IsBotResponse,
    },
    models::request_context::RequestContext,
    templates::error::internal_error,
    GA,
};

impl RequestContext {
    pub fn is_bot(&self) -> IsBotResponse {
        for i in self.domain.bot_settings.bots_allowed.iter() {
            // TODO: this shouldn't be done here! there should be another function that does this
            match crate::BOTS.read().get(i) {
                Some(t) => {
                    for ip in t.0.iter() {
                        if ip == &self.ip.ip && self.user_agent == t.1 {
                            return if is_rl_allowed(i.clone(), self) {
                                GA.handler.b.bot.inc();

                                IsBotResponse::Bot
                            } else {
                                GA.handler.b.ratelimited.inc();

                                IsBotResponse::Ratelimited
                            };
                        }
                    }
                }
                None => {
                    internal_error("user had a bot that didn't exist");
                }
            }
        }

        GA.handler.b.not_bot.inc();

        IsBotResponse::NotBot
    }
}

pub fn is_rl_allowed(bot: Bots, ctx: &RequestContext) -> bool {
    if ctx.domain.private_bucket.is_allowed(PrivateKeys::AllBots) {
        return ctx
            .domain
            .private_bucket
            .is_allowed(PrivateKeys::BotKey(bot));
    }
    false
}
