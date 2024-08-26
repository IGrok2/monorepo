use crate::tls::cert_roots::webpki_roots_cert_store;
use std::sync::Arc;

pub fn tls_config() -> Arc<tokio_rustls::rustls::ClientConfig> {
    let mut config = tokio_rustls::rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(webpki_roots_cert_store())
        .with_no_client_auth();

    config
        .dangerous()
        .set_certificate_verifier(Arc::new(danger::NoCertificateVerification));

    Arc::new(config)
}

// section for our certificate validator
mod danger {
    use super::*;
    use rustls::client::ServerCertVerifier;
    use std::time::SystemTime;

    pub struct NoCertificateVerification;

    // we don't care! (not yet)
    impl ServerCertVerifier for NoCertificateVerification {
        fn verify_server_cert(
            &self,
            _end_entity: &tokio_rustls::rustls::Certificate,
            _intermediates: &[tokio_rustls::rustls::Certificate],
            _server_name: &tokio_rustls::rustls::ServerName,
            _scts: &mut dyn Iterator<Item = &[u8]>,
            _oscp_response: &[u8],
            _now: SystemTime,
        ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
            Ok(rustls::client::ServerCertVerified::assertion())
        }
    }
}
