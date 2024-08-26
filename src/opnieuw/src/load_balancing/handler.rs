use crate::{
    models::{
        domain_context::{
            AppSettings,
            OriginSetting,
            OriginType,
        },
        regions::Region,
        request_context::{
            PipelineData,
            RequestContext,
        },
    },
    templates::error::internal_error,
    utils::counter::Counter,
    DOMAINS_DB,
    REGION,
};
use rand::seq::SliceRandom;
use std::{
    ops::Deref,
    sync::Arc,
};
use url::Url;

impl RequestContext {
    #[allow(clippy::type_complexity)]
    fn choose_setting(
        &self,
        data: &[PipelineData],
    ) -> Result<Option<(Arc<OriginSetting>, Option<Arc<AppSettings>>)>, String> {
        for i in data.iter() {
            if let Some(t) = i.backend.clone() {
                match t.as_ref() {
                    OriginType::Origin(origin) => return Ok(Some((origin.clone(), None))),
                    OriginType::App(app_name) => {
                        if let Some(app) = DOMAINS_DB.get(app_name) {
                            return Ok(Some((
                                app.app_settings.clone().unwrap().origin_settings.clone(),
                                Some(app.app_settings.clone().unwrap()),
                            )));
                        } else {
                            let msg = format!("App not found: {}", app_name);

                            internal_error(msg.as_str());

                            return Err(msg);
                        }
                    }
                }
            }
        }

        match self.domain.origin.settings.get(self.full_host.as_str()) {
            Some(t) => {
                return match t.value().deref() {
                    OriginType::Origin(origin) => return Ok(Some((origin.clone(), None))),
                    OriginType::App(app_name) => {
                        if let Some(app) = DOMAINS_DB.get(app_name) {
                            return Ok(Some((
                                app.app_settings.clone().unwrap().origin_settings.clone(),
                                Some(app.app_settings.clone().unwrap()),
                            )));
                        } else {
                            let msg = format!("App not found: {}", app_name);

                            internal_error(msg.as_str());

                            Err(msg)
                        }
                    }
                }
            }
            None => Ok(None),
        }
    }

    #[allow(clippy::type_complexity)]
    pub fn choose_backend(
        &self,
        data: &[PipelineData],
    ) -> Option<(Arc<OriginSetting>, (Url, Option<String>))> {
        if let Some(app) = &self.domain.app_settings {
            self.choose_app_middleware(app)
        } else if let Ok(Some(data)) = self.choose_setting(data) {
            if let Some(app) = data.1 {
                self.choose_app_middleware(&app)
            } else if let Some(origin) = data.0.origins.choose(&mut rand::thread_rng()) {
                Some((data.0.clone(), (origin.0.clone(), origin.1 .1.clone())))
            } else {
                None
            }
        } else {
            None
        }
    }

    // middleware to convert app_backend to origin_setting
    #[allow(clippy::type_complexity)]
    fn choose_app_middleware(
        &self,
        settings: &AppSettings,
    ) -> Option<(Arc<OriginSetting>, (Url, Option<String>))> {
        match Self::choose_app_backend(settings) {
            Some(t) => {
                // hit for the origin
                t.1 .1 .0.inc();

                Some((
                    settings.origin_settings.clone(),
                    (t.1 .1 .1 .0.clone(), Some(t.1 .1 .1 .1.clone())),
                ))
            }
            None => None,
        }
    }

    // takes in the vector of app origins (with region) and calculates which is closest to current region
    #[allow(clippy::type_complexity)]
    fn choose_app_backend(
        settings: &AppSettings,
    ) -> Option<(AppSettings, (Region, (Counter, (Url, String))))> {
        // choosing the method of least points

        // vector of closest app origins
        let mut closest_app = Vec::from([Vec::new(), Vec::new(), Vec::new()]);

        for i in settings.origins.iter() {
            let distance = i.0.get_distance(&REGION);

            if distance == 0 {
                return Some((settings.clone(), (i.0.clone(), i.1.clone())));
            }

            closest_app[distance as usize - 1].push((i.0.clone(), i.1.clone()));
        }

        // if there are no origins in the region, return None
        for i in closest_app.iter() {
            match i.len() {
                0 => continue,
                1 => {
                    return Some((settings.clone(), (i[0].0.clone(), i[0].1.clone())));
                }
                _ => {
                    // pick randomly of the options
                    let t = i.choose(&mut rand::thread_rng()).unwrap();

                    return Some((settings.clone(), (t.0.clone(), t.1.clone())));
                }
            }
        }

        None
    }
}
