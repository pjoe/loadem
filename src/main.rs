#![deny(warnings)]
#![warn(rust_2018_idioms)]
use futures::future::{join_all, FutureExt};
use futures::select;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, SystemTime};
use std::{env, fs, io};

use clap::{crate_version, App, Arg};
use hyper::{body::HttpBody as _, client, Client};
use simple_error::SimpleError;
use tokio::sync::mpsc;
use tokio::time::delay_for;

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() {
    let res = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(loadem());
    if let Err(e) = res {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

async fn loadem() -> Result<()> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .about("Makes some load")
        // use crate_version! to pull the version number
        .version(crate_version!())
        .arg(
            Arg::with_name("URL")
                .help("The url to test")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("CLIENTS")
                .help("The number of clients")
                .default_value("5")
                .index(2),
        )
        .arg(
            Arg::with_name("cert")
                .long("cert")
                .help("Custom certificate for HTTPS")
                .takes_value(true),
        )
        .get_matches();

    pretty_env_logger::init();

    let url = matches.value_of("URL").unwrap();
    let url = url.parse::<hyper::Uri>().unwrap();

    let clients = matches.value_of("CLIENTS").unwrap().parse::<u16>()?;

    println!("URL: {}", url);
    println!("Clients: {}", clients);
    let https = match matches.value_of("cert") {
        Some(cert_file) => {
            println!("Custom cert: {}", cert_file);
            let f = fs::File::open(cert_file)
                .map_err(|_| SimpleError::new("Custom cert file not found"))?;
            let mut rd = io::BufReader::new(f);
            // Build an HTTP connector which supports HTTPS too.
            let mut http = client::HttpConnector::new();
            http.enforce_http(false);
            // Build a TLS client, using the custom CA store for lookups.
            let mut tls = rustls::ClientConfig::new();
            tls.root_store
                .add_pem_file(&mut rd)
                .map_err(|_| SimpleError::new("Failed to load custom cert"))?;
            // Join the above part into an HTTPS connector.
            hyper_rustls::HttpsConnector::from((http, tls))
        }
        _ => hyper_rustls::HttpsConnector::new(),
    };

    let (tx, rx) = mpsc::channel::<u16>(100);

    static QUIT: AtomicBool = AtomicBool::new(false);
    let client: Client<_, hyper::Body> = Client::builder().build(https);
    let mut futures = vec![];
    for _ in 0..clients {
        futures.push(fetch_url(&url, &client, tx.clone(), &QUIT));
    }

    ctrlc::set_handler(move || {
        QUIT.store(true, Ordering::Relaxed);
    })
    .expect("Error setting Ctrl-C handler");

    select! {
        _ = join_all(futures).fuse() => {}
        _ = status(rx, &QUIT).fuse() => {}
        _ = heart_beat(tx.clone()).fuse() => {}
        _ = delay_for(Duration::from_secs(60)).fuse() => {}
    }
    Ok(())
}

async fn status(mut rx: mpsc::Receiver<u16>, quit: &AtomicBool) -> Result<()> {
    println!("Starting");
    let update_interval = Duration::from_secs(1);
    let mut now = SystemTime::now();
    let mut count_ok: u64 = 0;
    let mut count_err: u64 = 0;
    while let Some(res) = rx.recv().await {
        let elapsed = now.elapsed().unwrap();
        if elapsed.ge(&update_interval) {
            now = SystemTime::now();
            let factor = 1f32 / elapsed.as_secs_f32();
            println!(
                "Tps {:>7.2} Err {:>5.2}%",
                count_ok as f32 * factor,
                count_err as f32 * 100f32 / (count_ok + count_err) as f32,
            );
            count_ok = 0;
            count_err = 0;
        }
        match res {
            0 => {}
            200..=399 => {
                count_ok += 1;
            }
            _ => {
                count_err += 1;
            }
        }

        if quit.load(Ordering::Relaxed) {
            println!();
            break;
        }
    }
    println!("Done");
    Ok(())
}

async fn heart_beat(mut tx: mpsc::Sender<u16>) -> Result<()> {
    loop {
        tx.send(0).await?;
        delay_for(Duration::from_millis(250)).await;
    }
}

async fn fetch_url(
    url: &hyper::Uri,
    client: &Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>, hyper::Body>,
    mut tx: mpsc::Sender<u16>,
    quit: &AtomicBool,
) -> Result<()> {
    // println!("fecthing: {}", url);
    while !quit.load(Ordering::Relaxed) {
        let res = client.get(url.clone()).await;
        let status = match res {
            Ok(mut res) => {
                let mut status: u16 = res.status().into();
                // Stream the body, writing each chunk to stdout as we get it
                // (instead of buffering and printing at the end).
                while let Some(next) = res.data().await {
                    let chunk = next;
                    if chunk.is_err() {
                        status = 901;
                        break;
                    }
                    //io::stdout().write_all(&chunk).await?;
                }

                status
            }
            _ => 900,
        };
        tx.send(status).await?;

        delay_for(Duration::from_micros(1)).await;
    }

    // println!("\n\nDone!");

    Ok(())
}
