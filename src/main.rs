extern crate clap;

use clap::{App, Arg};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::prelude::*;
use url::{Host, Url};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let backends: Vec<std::net::SocketAddr> = Vec::new();

    let matches = App::new("My Super Program")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .arg(
            Arg::with_name("host")
                .short("h")
                .long("host")
                .help("Host to serve the application")
                .default_value("127.0.0.1")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .help("Port to serve the application")
                .default_value("9091")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("backends")
                .long("backends")
                .help("Backends for application")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let net_interface = format!(
        "{}:{}",
        matches.value_of("host").unwrap_or_default(),
        matches.value_of("port").unwrap_or_default()
    );
    let mut listener = TcpListener::bind(&net_interface).await?;

    println!("Parsing backends");
    let urls: Vec<&str> = matches
        .value_of("backends")
        .unwrap_or("")
        .split(",")
        .collect();
    for url in urls {
        let v = match Url::parse(url) {
            Ok(v) => v,
            Err(err) => {
                eprintln!("Error parsing {}, {}", url, err);
                std::process::exit(1)
            }
        };
        println!("Parsed {:?}", v);
        // let s_addr = format!("{}:{}", Host::Domain(v.host()), v.port());
    }

    println!("Starting application at {}", &net_interface);
    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // Write the data back
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
