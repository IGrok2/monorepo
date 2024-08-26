use std::{
    error::Error,
    mem,
    ops::DerefMut,
};

use crate::{
    buckets::private::PrivateBucket,
    debug,
    grpc::{
        all::{
            big_baller_client::BigBallerClient,
            QueryResponse,
        },
        specific_helpers::RpcToRust,
    },
    models::{
        analytics::Analytic,
        analytics_by_example::AnalyticsByExampleDomain,
        domain_context::DomainContext,
    },
    utils::counter::Counter,
    DOMAINS_DB,
};
use aes::cipher::{
    generic_array::GenericArray,
    KeyInit,
};
use aes_gcm::Aes128Gcm;
use bytes::Bytes;
use dashmap::DashMap;
use std::sync::Arc;
use tonic::transport::{
    Certificate,
    ClientTlsConfig,
    Identity,
};

pub async fn get_all_domains() -> Result<(), Box<dyn Error>> {
    lazy_static! {
        static ref RPC_HOST: String =
            "https://".to_string() + &std::env::var("API_RPC_HOST").unwrap();
    }

    let channel = tonic::transport::Channel::from_static(&RPC_HOST)
        .tls_config(
            ClientTlsConfig::new()
                .ca_certificate(Certificate::from_pem(std::fs::read_to_string(
                    "certs/ca-cert.pem",
                )?))
                .identity(Identity::from_pem(
                    std::fs::read_to_string("certs/client-cert.pem")?,
                    std::fs::read_to_string("certs/client-key.pem")?,
                )),
        )?
        .connect()
        .await?;

    let mut client = BigBallerClient::new(channel);

    let request = tonic::Request::new(QueryResponse {
        success: true,
        message: "now tell me, WHY THE fuck aren't you working here?!!?!".to_string(),
    });

    // make the request
    let foo_request = client.all_domains(request).await?;

    let response = foo_request.into_inner();

    if response.success {
        let mut bg_mut = crate::BACKGROUND_CHALLENGE.write().unwrap();
        let mut smart_mut = crate::SMART_CHALLENGE.write().unwrap();
        let mut keys_mut = crate::CHALLENGE_KEYS.write().unwrap();

        let _ = mem::replace(
            bg_mut.deref_mut(),
            response.challenge.clone().unwrap().bg_challenge_script,
        );
        let _ = mem::replace(
            smart_mut.deref_mut(),
            response.challenge.clone().unwrap().smart_challenge_script,
        );
        let _ = mem::replace(
            keys_mut.deref_mut(),
            Aes128Gcm::new(GenericArray::from_slice(
                &response.challenge.clone().unwrap().key,
            )),
        );

        for domain_schema in response.everything.iter() {
            // all the unwraps are appropriate because there should absolutely be a panic here if this data doesn't exist

            // these need to be processed first
            let internal_settings =
                RpcToRust::internal_settings(domain_schema.internal_settings.clone().unwrap())
                    .unwrap();
            let buckets =
                RpcToRust::bucket_settings(domain_schema.buckets.clone().unwrap()).unwrap();
            let origin =
                RpcToRust::origin_settings(domain_schema.origin_settings.clone(), None).unwrap();

            debug!("inserting for domain {}", domain_schema.domain.clone());

            DOMAINS_DB.insert(
                domain_schema.domain.clone(),
                Arc::new(DomainContext {
                    domain: domain_schema.domain.clone(),
                    origin: origin.clone(),
                    api_engine_settings: RpcToRust::api_engine(
                        domain_schema.api_engine_settings.clone().unwrap(),
                        None,
                    )
                    .unwrap(),
                    caching_settings: RpcToRust::cache_settings(
                        domain_schema.cache_settings.clone().unwrap(),
                        None,
                    )
                    .unwrap(),
                    private_bucket: Arc::new(
                        PrivateBucket::new().setup_defaults(&internal_settings),
                    ),
                    internal_settings,
                    bot_settings: RpcToRust::bots(domain_schema.bot_settings.clone().unwrap())
                        .unwrap(),
                    human_engine_settings: RpcToRust::human_engine(
                        domain_schema.human_engine_settings.clone().unwrap(),
                        None,
                    )
                    .unwrap(),
                    buckets: buckets.clone(),
                    analytic: Arc::new(Analytic::new()),
                    rules: RpcToRust::page_rules(
                        domain_schema.page_rules.clone().unwrap(),
                        None,
                        buckets.clone(),
                        origin.clone(),
                    )
                    .unwrap(),
                    analytics_by_example: AnalyticsByExampleDomain {
                        started: Counter::new(),
                        examples: DashMap::new(),
                    },
                    app_settings: RpcToRust::app_settings(
                        domain_schema.app_origin_settings.clone(),
                        None,
                    )
                    .unwrap(),
                }),
            );
        }

        for keys in response.keys.iter() {
            // easy
            RpcToRust::cert(keys.clone()).unwrap();
        }
    } else {
        // cool question mark at the end
        return Err("invalid data")?;
    }

    Ok(())
}
