//! # iperf3-statuspage
//!
//! A utility application to serve iperf3 results over an HTTP endpoint.

// Copyright (c) 2025 Jak Bracegirdle
//
// This file is part of the iperf3_statuspage crate.
//
// Licensed under the Apache License, Version 2.0 <http://www.apache.org/licenses/LICENSE-2.0>
// or the MIT license <http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

mod models;

use actix_web::{App, HttpServer};
use dotenvy;
use std::env;
use iperf3_statuspage::{spawn_iperf3_scheduler, iperf3};

/// Main entrypoint starts the Actix-web server and the periodic iperf3 runner.
///
/// Binds to `BIND_ADDRESS` and `BIND_PORT` environment variables or defaults.
///
/// # Panics
///
/// Panics if `BIND_PORT` cannot be parsed as a valid `u16`.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let bind_address = env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1".to_string());
    let bind_port_str = env::var("BIND_PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_port: u16 = bind_port_str.parse().expect("BIND_PORT must be a valid u16");

    let iperf3_ip = env::var("IPERF3_SERVER_IP").expect("IPERF3_SERVER_IP must be set");
    let iperf3_port = env::var("IPERF3_SERVER_PORT").expect("IPERF3_SERVER_PORT must be set");


    // Spawn the periodic speedtest updater
    tokio::spawn(spawn_iperf3_scheduler(iperf3_ip, iperf3_port));

    println!("Starting server at http://{}:{}/iperf3", bind_address, bind_port);

    HttpServer::new(|| App::new().service(iperf3))
        .bind((bind_address.as_str(), bind_port))?
        .run()
        .await
}