use crate::handler::pipeline::rules::models::MatchType;
use crate::GA;

pub fn key_value(actual: (&str, &str), expected: (&str, &str), m_type: &MatchType) -> bool {
    GA.handler.pr.key_value_tested.inc();

    if expected.0 == "*" && expected.1 == "*" {
        return true;
    }

    // now, at this point, something must match!

    if expected.0 == "*" {
        match m_type {
            MatchType::Exact => {
                if expected.1 == actual.1 {
                    return true;
                }
            }
            MatchType::Contains => {
                if actual.1.contains(expected.1) {
                    return true;
                }
            }
            MatchType::StartsWith => {
                if actual.1.starts_with(expected.1) {
                    return true;
                }
            }
            MatchType::UseStar => todo!(), // TODO
        }
    } else if expected.1 == "*" {
        match m_type {
            MatchType::Exact => {
                if expected.0 == actual.0 {
                    return true;
                }
            }
            MatchType::Contains => {
                if actual.0.contains(expected.0) {
                    return true;
                }
            }
            MatchType::StartsWith => {
                if actual.0.starts_with(expected.0) {
                    return true;
                }
            }
            MatchType::UseStar => todo!(), // TODO
        }
    }

    // then it's gotta be an exact match

    if actual == expected {
        return true;
    }

    false
}
