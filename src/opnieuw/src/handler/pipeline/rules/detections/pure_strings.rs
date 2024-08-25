use crate::handler::pipeline::rules::models::MatchType;
use crate::{debug, GA};

pub fn pure_string(actual: &str, expected: &str, m_type: &MatchType) -> bool {
    GA.handler.pr.pure_string_tested.inc();

    if expected == "*" {
        return true;
    }
    match m_type {
        MatchType::Exact => {
            debug!(
                "PURE STRING MATCH: actual: {}, expected: {}",
                actual, expected
            );
            actual == expected
        }
        MatchType::Contains => actual.contains(expected),
        MatchType::StartsWith => actual.starts_with(expected),
        MatchType::UseStar => todo!(),
    }
}
