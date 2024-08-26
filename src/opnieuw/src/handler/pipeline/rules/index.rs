use crate::{debug, GA};
use std::ops::DerefMut;

use crate::handler::pipeline::rules::actions::do_action;
use crate::handler::pipeline::rules::detections::key_value::key_value;
use crate::handler::pipeline::rules::detections::pure_strings::pure_string;
use crate::handler::pipeline::rules::models::{TriggerRequirement, TriggerType};
use crate::models::pipeline_response::PipelineResponse;
use crate::models::request_context::{PipelineData, RequestContext};

impl RequestContext {
    pub fn rules(&self, _data: &[PipelineData]) -> PipelineResponse {
        if !self.domain.rules.is_enabled || self.domain.rules.rules.is_empty() {
            return PipelineResponse::Ok(None);
        }

        // what do we need to do here?
        /*
           1. iterate through rules and determine if any of them match
               - matching is somewhat complex, there's multiple ways to approach it
                   - we have to match a key that contains at least one value, attached to how it should be matched
           2. if they match, perform the action
        */

        let mut the_matches = Vec::new();

        for i in self.domain.rules.rules.iter() {
            // if the api engine rule isn't enabled, no need to continue processing this rule
            if !i.enabled {
                continue;
            }

            if !i.can_run() {
                continue;
            }

            GA.handler.pr.rules_tested.inc();

            let mut matches = false;
            // how many matches have been completed (applicable when TriggerRequirement is set to Multiple)
            let mut all_matches = 0;

            for n in i.trigger.match_type.iter() {
                matches = false;

                GA.handler.pr.matches_tested.inc();

                // see if this trigger "matches"
                match &n.trigger {
                    TriggerType::Ip(t) => {
                        if pure_string(&self.ip.ip, t, &n.m_type) {
                            matches = true
                        }
                    }
                    TriggerType::Path(t) => {
                        if pure_string(self.req.uri.path(), t, &n.m_type) {
                            matches = true
                        }
                    }
                    TriggerType::Country(t) => {
                        if pure_string(&self.get_ipdata().0, t, &n.m_type) {
                            matches = true
                        }
                    }
                    TriggerType::Continent(t) => {
                        if pure_string(&self.get_ipdata().1, t, &n.m_type) {
                            matches = true
                        }
                    }
                    TriggerType::Asn(t) => {
                        if pure_string(&self.get_ipdata().2, t, &n.m_type) {
                            matches = true
                        }
                    }
                    TriggerType::Host(t) => {
                        if pure_string(&self.full_host, t, &n.m_type) {
                            matches = true
                        }
                    }
                    TriggerType::Method(t) => {
                        // check if the methods are the same
                        if t == self.req.method {
                            matches = true;
                        }
                    }
                    TriggerType::UserAgent(t) => {
                        if pure_string(&self.user_agent, t, &n.m_type) {
                            matches = true
                        }
                    }
                    TriggerType::Cookie(t) => {
                        // check if the cookie name exists
                        if let Some(cookies) = self.get_cookies() {
                            for cookie in cookies.iter() {
                                // the amount of cookies is being checked by request inspection
                                if cookie[0] == t {
                                    matches = true;
                                    break;
                                }
                            }
                        }
                    }
                    TriggerType::Proto(t) => {
                        let proto = match self.connection_context.https {
                            true => "https",
                            false => "http",
                        };

                        if pure_string(proto, t, &n.m_type) {
                            matches = true
                        }
                    }
                    TriggerType::Headers { key, value } => {
                        for i in self.req.headers.iter() {
                            if key_value(
                                (i.0.as_str(), i.1.to_str().unwrap()),
                                (key, value),
                                &n.m_type,
                            ) {
                                matches = true;
                                break;
                            }
                        }
                    }
                    TriggerType::Query { .. } => {} // TODO (dependent on query parser)
                    TriggerType::Any => matches = true,
                }

                // flip it around if requested (for whitelisting)
                if n.inversed {
                    matches = !matches;
                }

                // doesn't match and it was required - we're done
                if !matches && n.required {
                    // doesn't match and it was required to match
                    break;
                }

                // what is the requirement?
                match i.trigger.trigger_requirement {
                    // only one trigger is required to set off the rule
                    TriggerRequirement::One => {
                        // it matched, so we can add this rule and cut it out
                        if matches {
                            the_matches.push(i);
                            matches = false;
                            // stop the loop
                            break;
                        }
                    }
                    // all of the triggers are required
                    TriggerRequirement::All => {
                        debug!("trigger requirement was all, matches: {}\n\n", matches);
                        // if it didn't match, shoot!
                        if !matches {
                            break;
                        }
                    }
                    // specific amount of triggers need to be pulled
                    TriggerRequirement::Multiple(n) => {
                        if matches {
                            // matched, add to the count
                            all_matches += 1;

                            if all_matches == n {
                                // we're good enough
                                break;
                                // matches is already true
                            }
                        }
                    }
                }
            }

            // gone through the match types, check if it's inversed from here
            if i.trigger.inversed {
                matches = !matches;
            }

            // matched, so add it to the actions
            if matches {
                use std::ops::DerefMut;
                if let Some(ref mut t) = self.by_example.borrow_mut().deref_mut() {
                    t.rules.hit = true;
                    t.rules.rule_ids.push(i.id.clone());
                    t.rules.action.push(format!("{:?}", i.action))
                }

                the_matches.push(i);
            }
        }

        let mut pipeline_data: Option<Vec<PipelineData>> = None;

        for i in the_matches.iter() {
            // increment the analytics
            i.analytic.inc();

            if let Some(pipeline_response) = do_action(self, &i.action) {
                match pipeline_response {
                    PipelineResponse::Ok(Some(data)) => {
                        if let Some(mut t) = pipeline_data.clone() {
                            t.extend(data)
                        } else {
                            pipeline_data = Some(data)
                        }
                    }
                    _ => return pipeline_response,
                }
            }
        }

        PipelineResponse::Ok(pipeline_data) // no rules here
    }
}
