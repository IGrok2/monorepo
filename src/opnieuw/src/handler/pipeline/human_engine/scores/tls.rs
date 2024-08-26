use crate::{
    buckets::models::PublicBucket,
    models::request_context::RequestContext,
    tls::models::TlsFingerprint,
    GA,
};

/*
This will ultimately be the package for TLS fingerprinting and other TLS specifications!
 */

impl RequestContext {
    pub fn check_tls(&self) -> u32 {
        let mut score = 0;

        if let Some(t) = self.connection_context.fingerprint {
            if t == TlsFingerprint::Unknown {
                GA.handler.he.tls_fingerprint.inc();

                score += 100;
            }
        } else {
            GA.handler.he.tls_no_fingerprint.inc();

            score += 100;
        }

        score
    }
}
