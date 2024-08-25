use rustls::RootCertStore;
use tokio_rustls::rustls;

pub fn webpki_roots_cert_store() -> rustls::RootCertStore {
    let mut root_certs = RootCertStore::empty();

    for cert in webpki_roots::TLS_SERVER_ROOTS {
        let cert = rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
            cert.subject,
            cert.spki,
            cert.name_constraints,
        );
        let certs = vec![cert].into_iter();
        root_certs.add_trust_anchors(certs);
    }

    root_certs
}
