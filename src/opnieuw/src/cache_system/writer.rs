use crate::cache_system::models::{CacheBucket, CachedObject};
use crate::models::domain_context::DomainContext;
use clickhouse::sql::Bind;
use hyper::{HeaderMap, StatusCode};
use pin_project_lite::pin_project;
use std::fs;
use std::fs::read;
use std::io::{BufReader, Error, ErrorKind};
use std::mem::ManuallyDrop;
use std::ops::Add;
use std::path::Path;
use std::pin::Pin;
use std::process::Command;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::thread::sleep;
use std::time::{Duration, Instant};
use tokio::fs::File;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWriteExt, BufWriter, ReadBuf};
use tokio::{select, task};

pub struct CacheWriter {
    pub domain: Arc<DomainContext>,
    pub ttl: Duration,
    pub id: String,
    pub location: String,
    pub writer: ManuallyDrop<BufWriter<File>>,
    pub bytes: u64,
    pub buf: [u8; 4096],
    pub finished: bool,
    pub headers: HeaderMap,
    pub expected_length: u64,
    pub level: CacheLevel,
    pub started: Instant,
    pub status: StatusCode,
}

impl Drop for CacheWriter {
    fn drop(&mut self) {
        GA.cs.writer_drop.inc();

        if self.expected_length == self.bytes {
            // expected cache length, so let's add it to the cache

            // get the things that have to be done async
            let mut writer = unsafe { ManuallyDrop::<BufWriter<File>>::take(&mut self.writer) };

            // flush any bytes that may be lingering
            tokio::spawn(async move {
                writer.flush().await.unwrap();
            });

            GA.cs.writer_attempt_insert.inc();

            self.domain.caching_settings.bucket.map.insert(
                self.id.clone(),
                Arc::new(CachedObject {
                    domain: self.domain.clone(),
                    location: self.location.clone(),
                    id: self.id.clone(),
                    headers: self.headers.clone(),
                    level: self.level,
                    created: self.started,
                    lives_until: self.started.add(self.ttl),
                    size: self.bytes as usize,
                    http_status: self.status,
                }),
            );

            GA.cs.writer_finish_insert.inc();
        } else {
            GA.cs.writer_non_expected_length.inc();

            std::fs::remove_file(self.location.clone()).unwrap();
        }

        GA.cs.writer_drop_finish.inc();

        let domain = self.domain.clone();

        let location = self.location.clone();

        // flush any bytes that may be lingering
        tokio::spawn(async move {
            domain
                .caching_settings
                .bucket
                .allocations
                .write()
                .await
                .remove(&location);
        });
    }
}

impl CacheWriter {
    pub async fn new(
        domain: Arc<DomainContext>,
        ttl: Duration,
        uri: String,
        headers: HeaderMap,
        status: StatusCode,
        expected_length: u64,
    ) -> Option<Self> {
        let location = format!(
            "/var/bigballs/opnieuw/oligarch/{}/{}",
            domain.domain.clone(),
            uri.replace("https://", "")
                .replace("http://", "")
                .replace("/", "#")
        );

        let mut lock = domain.caching_settings.bucket.allocations.write().await;

        if domain.caching_settings.bucket.map.get(&uri).is_some() {
            return None;
        }

        if lock.get(&location).is_some() {
            return None;
        } else {
            lock.insert(location.clone(), "".to_string());
        }

        drop(lock);

        match File::create(&location).await {
            Ok(t) => Some(CacheWriter {
                domain,
                ttl,
                id: uri,
                location,
                writer: ManuallyDrop::new(BufWriter::new(t)),
                bytes: 0,
                buf: [0; 4096],
                finished: false,
                expected_length,
                level: CacheLevel::Null,
                started: Instant::now(),
                headers,
                status,
            }),
            Err(_) => {
                // finally, remove the allocation
                domain
                    .caching_settings
                    .bucket
                    .allocations
                    .write()
                    .await
                    .remove(&location);

                GA.cs.writer_create_directory.inc();

                // try to create the directory
                if let Err(e) = Command::new("mkdir")
                    .args([&format!(
                        "/var/bigballs/opnieuw/oligarch/{}",
                        domain.domain.clone()
                    )])
                    .spawn()
                {
                    GA.cs.writer_create_directory_fail.inc();

                    internal_error(&format!("Error writing to oligarch directory: {:?}", e));
                };

                None
            }
        }
    }

    pub async fn write(&mut self, bytes: &[u8]) {
        GA.cs.writer_write.inc();

        self.bytes += bytes.len() as u64;

        match self.debug_writer(bytes).await {
            Ok(_) => {
                GA.cs.writer_write_ok.inc();
            }
            Err(e) => {
                GA.cs.writer_write_err.inc();

                internal_error(&format!("Error writing to path: {}: {}", self.location, e));

                // cancel the write
                self.bytes = 0;
            }
        };
    }

    async fn debug_writer(&mut self, bytes: &[u8]) -> Result<(), String> {
        GA.cs.executor_began.inc();

        match timeout(Duration::from_secs(10), self.debug_debug_writer(bytes)).await {
            Ok(t) => {
                GA.cs.executor_finished.inc();

                if let Err(e) = t {
                    internal_error(&format!("Error writing to path: {}: {}", self.location, e));

                    self.bytes = 0;
                }

                Ok(())
            }
            Err(_e) => {
                let str = format!(
                    "Write timed out, specs: written: {}, total: {}\
            location: {}, buffer size: {}",
                    self.bytes,
                    self.expected_length,
                    self.location,
                    self.writer.buffer().len()
                );

                Err(str)
            }
        }
    }

    async fn debug_debug_writer(&mut self, bytes: &[u8]) -> Result<(), Error> {
        self.writer.write_all(bytes).await?;

        Ok(())
    }

    pub fn finish_write(&mut self) {
        GA.cs.writer_finish_write.inc();

        // finish the buffer and convert it to a cached object
        ::futures::executor::block_on(self.writer.flush()).unwrap();
        self.finished = true;

        // now, add it to the map
        self.domain.caching_settings.bucket.map.insert(
            self.id.clone(),
            Arc::new(CachedObject {
                domain: self.domain.clone(),
                location: self.location.clone(),
                id: self.id.clone(),
                headers: self.headers.clone(),
                level: CacheLevel::Null,
                http_status: self.status,
                created: Instant::now(),
                lives_until: Instant::now().add(self.ttl),
                size: self.bytes as usize,
            }),
        );

        GA.cs.writer_finish_write_finished.inc();
    }
}

use crate::handler::pipeline::caching::models::CacheLevel;
use crate::GA;
use futures_util::ready;
use futures_util::task::SpawnExt;
use hyper::body::Body;
use tokio::time::timeout;
// TODO reinstate
// use crate::rproxy::pipeline::compression::compress_zip;
use crate::templates::error::internal_error;
