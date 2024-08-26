use crate::{
    models::domain_context::DomainContext,
    GA,
};
use std::time::Duration;
use tokio::time::timeout;

impl DomainContext {
    pub async fn stream_timeout<T>(
        &self,
        duration: Duration,
        future: impl std::future::Future<Output = T> + 'static,
    ) -> std::io::Result<T> {
        // let's say duration is 10sec
        match timeout(duration, future).await {
            Ok(t) => Ok(t),
            Err(_err) => {
                // make sure the error is a timeout one
                GA.timeout.stream.inc();
                Err(std::io::Error::new(
                    std::io::ErrorKind::TimedOut,
                    "Stream timeout",
                ))
            }
        }
    }
}

pub async fn admin_timeout<T>(
    duration: Duration,
    future: impl std::future::Future<Output = T>,
) -> std::io::Result<T> {
    // let's say duration is 10sec
    match timeout(duration, future).await {
        Ok(t) => Ok(t),
        Err(_err) => {
            // make sure the error is a timeout one
            GA.timeout.stream.inc();
            Err(std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                "Stream timeout",
            ))
        }
    }
}
