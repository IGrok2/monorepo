use std::fs;
use std::fs::read;
use tokio::fs::File;
use tokio::io::{self, AsyncRead, AsyncReadExt, AsyncWriteExt, BufWriter, ReadBuf};
use std::io::{BufReader};
use std::path::Path;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::task::{Context, Poll};
use std::thread::sleep;
use std::time::{Duration, Instant};
use futures::io::Read;
use futures::{pin_mut, Stream, StreamExt};
use pin_project_lite::pin_project;
use tokio::{select, task};


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let file = Path::new("./me.jpeg");

    // let mut writer = File::create(&file).await.unwrap();

    let mut opened = File::open(&file).await.unwrap();
    /*
        for _ in 0..1000 {
            let bytes = "\nhello\n".as_bytes();

            writer.write_all(bytes).await.unwrap();
        }

        println!("sleeping");

     */

    // sleep(Duration::from_secs(10));

    let file2 = Path::new("./me-two.jpeg");

    let opened2 = File::create(&file2).await.unwrap();

    // Wrap the file in a BufWriter for better performance
    let mut buf_writer = BufWriter::with_capacity(8192000, opened2);

    let reader = CacheWriter {
        polls: 0,
        reader: opened,
        pos: 0,
        bytes: 0,
        buf: [0; 4096],
        read_amount: 0,
    };

    reader.for_each(|obj| {
        futures::executor::block_on(
            buf_writer.write_all(&obj)
        ).unwrap();

        futures::future::ready(())
    }).await;

    if buf_writer.buffer().len() != 0 {
        buf_writer.flush().await.unwrap();
    }

    Ok(())
}

pub struct Writer {
    pub write_to: CacheWriter,
    pub write_pos: u32,
}

pub struct Reader {
    pub obj: CachedObject,
    pub read_pos: u32,
}

pub struct Finished {
    pub size: u64,
    pub hits: u32, // will be counter
}

pub enum Status {
    InProgress(CacheWriter),
    Finished(Finished)
}
/*
impl CachedObject {
    pub fn new(ctx: String, path: String, writer: CacheWriter) -> Self {
        // enter cache directory and create directory if it's currently yet to exist
        let path: &Path = match Path::new(&format!("{}", ctx)).exists() {
            true => {
                // there is a path that exists here

                Path::new(&format!("{}/{}", ctx, path))
            },
            false => {
                // path doesn't exist yet, so let's make the directory

                fs::create_dir(&format!("{}", ctx));

                Path::new(&format!("{}/{}", ctx, path))
            }
        };

        CachedObject {
            domain: ctx,
            location: path,
            status: Status::InProgress(writer),
            created: Instant::now(),
            lives_until: (),
        }
    }
}

 */
