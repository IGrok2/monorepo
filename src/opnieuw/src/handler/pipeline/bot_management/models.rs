use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Bots {
    Googlebot = 1,
    Bingbot = 2,
    UptimeRobot = 3,
}

impl Display for Bots {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Bots::Googlebot => write!(f, "Googlebot"),
            Bots::Bingbot => write!(f, "Bingbot"),
            Bots::UptimeRobot => write!(f, "UptimeRobot"),
        }
    }
}

pub enum IsBotResponse {
    Bot,
    NotBot,
    Ratelimited,
}
