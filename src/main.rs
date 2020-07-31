#![deny(warnings)]
#![warn(rust_2018_idioms)]
use futures::future::{join_all, FutureExt};
use futures::select;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, SystemTime};
use std::{env, fs, io};

use clap::{crate_version, App, Arg};
use hyper::{body::HttpBody as _, client, Client, Method, Request};
use simple_error::SimpleError;
use tokio::io as tokio_io;
use tokio::io::AsyncWriteExt as _;
use tokio::sync::mpsc;
use tokio::time::delay_for;

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug)]
struct Response {
    status: u16,
    response_time: f32,
}

fn main() {
    let res = tokio::runtime::Runtime::new().unwrap().block_on(loadem());
    if let Err(e) = res {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

async fn loadem() -> Result<()> {
    let matches =
        App::new(env!("CARGO_PKG_NAME"))
            .about("Makes continous load for web server testing.")
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
            .arg(
                Arg::with_name("time-limit")
                    .long("time-limit")
                    .short("l")
                    .help("Time limit. Test will end after specified number of seconds")
                    .default_value("0")
                    .takes_value(true),
            )
            .arg(Arg::with_name("test").long("test").short("t").help(
                "Test mode. No throughput measurements. Full response (with headers) is shown",
            ))
            .arg(
                Arg::with_name("header")
                    .long("header")
                    .short("H")
                    .help("Add HTTP request header(s)")
                    .multiple(true)
                    .takes_value(true),
            )
            .get_matches();

    pretty_env_logger::init();

    let url = matches.value_of("URL").unwrap();
    let url = url.parse::<hyper::Uri>().unwrap();

    let test = matches.is_present("test");
    let clients = if test {
        1
    } else {
        matches.value_of("CLIENTS").unwrap().parse::<u16>()?
    };
    let time_limit = matches.value_of("time-limit").unwrap().parse::<u64>()?;
    let headers: Vec<(&str, &str)> = match matches.values_of("header") {
        Some(vals) => vals
            .map(|h| {
                let parts: Vec<&str> = h.splitn(2, ':').collect();
                match parts.len() {
                    1 => (parts[0], ""),
                    _ => (parts[0], parts[1].trim_start()),
                }
            })
            .collect(),
        None => vec![],
    };

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

    let (tx, rx) = mpsc::channel::<Response>(100);

    static QUIT: AtomicBool = AtomicBool::new(false);
    let client: Client<_, hyper::Body> = Client::builder().build(https);
    let mut futures = vec![];
    for _ in 0..clients {
        futures.push(fetch_url(&url, &client, &headers, tx.clone(), test, &QUIT));
    }

    ctrlc::set_handler(move || {
        QUIT.store(true, Ordering::Relaxed);
    })
    .expect("Error setting Ctrl-C handler");

    select! {
        _ = join_all(futures).fuse() => {}
        _ = status(rx, test, &QUIT).fuse() => {}
        _ = heart_beat(tx.clone()).fuse() => {}
        _ = timeout(time_limit).fuse() => {}
    }
    Ok(())
}

async fn timeout(limit: u64) -> Result<()> {
    match limit {
        0 => loop {
            delay_for(Duration::from_secs(1000)).await;
        },
        secs => {
            delay_for(Duration::from_secs(secs)).await;
        }
    }
    Ok(())
}

async fn status(mut rx: mpsc::Receiver<Response>, test: bool, quit: &AtomicBool) -> Result<()> {
    println!("Starting");
    let update_interval = Duration::from_secs(1);
    let mut now = SystemTime::now();
    let mut count_ok: u64 = 0;
    let mut count_err: u64 = 0;
    let mut resp_time: f32 = 0f32;
    while let Some(res) = rx.recv().await {
        let elapsed = now.elapsed().unwrap();
        if elapsed.ge(&update_interval) {
            now = SystemTime::now();
            let factor = 1f32 / elapsed.as_secs_f32();
            let total = count_ok + count_err;
            if !test {
                println!(
                    "Tps {:>7.2}, Err {:>5.2}%, Resp Time {:>6.3}",
                    count_ok as f32 * factor,
                    count_err as f32 * 100f32 / total as f32,
                    resp_time / total as f32,
                );
            }
            count_ok = 0;
            count_err = 0;
            resp_time = 0f32;
        }
        match res.status {
            0 => {}
            200..=399 => {
                count_ok += 1;
                resp_time += res.response_time;
            }
            _ => {
                count_err += 1;
                resp_time += res.response_time;
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

async fn heart_beat(mut tx: mpsc::Sender<Response>) -> Result<()> {
    loop {
        tx.send(Response {
            status: 0,
            response_time: 0f32,
        })
        .await?;
        delay_for(Duration::from_millis(250)).await;
    }
}

async fn fetch_url(
    url: &hyper::Uri,
    client: &Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>, hyper::Body>,
    headers: &Vec<(&str, &str)>,
    mut tx: mpsc::Sender<Response>,
    test: bool,
    quit: &AtomicBool,
) -> Result<()> {
    while !quit.load(Ordering::Relaxed) {
        let start = SystemTime::now();
        let mut req_builder = Request::builder().method(Method::GET).uri(url);
        for (header, value) in headers.iter() {
            req_builder = req_builder.header(*header, *value);
        }
        let req = req_builder.body(hyper::Body::empty())?;
        let res = client.request(req).await;
        let status = match res {
            Ok(mut res) => {
                let mut status: u16 = res.status().into();
                if test {
                    println!("Status: {}", status);
                    println!("Headers: {:#?}\n", res.headers());
                    println!("Body:");
                }
                // Stream the body, writing each chunk to stdout as we get it
                // (instead of buffering and printing at the end).
                while let Some(next) = res.data().await {
                    let chunk = next;
                    if chunk.is_err() {
                        status = 901;
                        break;
                    }
                    if test {
                        tokio_io::stdout().write_all(&chunk?).await?;
                    }
                }

                status
            }
            _ => 900,
        };
        let response_time = start.elapsed().unwrap().as_secs_f32();
        tx.send(Response {
            status,
            response_time,
        })
        .await?;

        delay_for(Duration::from_micros(1)).await;
        if test {
            println!();
            break;
        }
    }

    Ok(())
}
