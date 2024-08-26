use crate::tls::resolver_model::CertResolver;
use crate::{CERTS, GA, WILDCARD_CERT};
use std::sync::Arc;
use tokio_rustls::rustls::server::{ClientHello, ResolvesServerCert};
use tokio_rustls::rustls::sign::CertifiedKey;

impl ResolvesServerCert for CertResolver {
    fn resolve(&self, client_hello: ClientHello<'_>) -> Option<Arc<CertifiedKey>> {
        let outcome = match client_hello.server_name() {
            Some(t) => {
                if t.ends_with(".onpacketware.net") {
                    GA.tls.cert_onpacketware.inc();

                    // get the lock
                    let lock = WILDCARD_CERT.read().unwrap();

                    // return
                    return lock.as_ref().map(|t| t.clone());
                }

                CERTS.get(t).map(|t| t.clone())
            }
            None => None,
        };

        GA.tls.cert_requested.inc();

        if outcome.is_some() {
            GA.tls.cert_found.inc();
        } else {
            GA.tls.cert_not_found.inc();
        }

        outcome
    }
}
