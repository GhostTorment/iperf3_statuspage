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

pub mod models;

use std::env;
use std::process::{Stdio};
use std::sync::Mutex;
use std::time::{Duration, Instant};
use actix_web::{get, HttpResponse, Responder};
use async_trait::async_trait;
use once_cell::sync::Lazy;
use tokio::process::Command;
use tokio::time;
pub use models::*;

/// Global cached iperf3 result and the instant it was cached.
///
/// Wrapped in a mutex for thread-safe mutable access.
/// Initially empty until the first iperf3 run.
pub static LAST_RESULT: Lazy<Mutex<Option<(Iperf3Report, Instant)>>> = Lazy::new(|| Mutex::new(None));

/// Retrieves the last cached iperf3 result, if available.
///
/// # Examples
///
/// ```
/// # use iperf3_statuspage::{Iperf3Report, get_last_result, set_last_result_for_test, clear_last_result_for_test, Start, End, Timestamp, ConnectingTo, TestStart, SumSent, SumReceived, CpuUtilizationPercent};
/// # let dummy_result = Iperf3Report {
/// #     start: Start {
/// #         connected: vec![],
/// #         version: "".to_string(),
/// #         system_info: "".to_string(),
/// #         timestamp: Timestamp {
/// #             time: "".to_string(),
/// #             timesecs: 0
/// #         },
/// #         connecting_to: ConnectingTo {
/// #             host: "".to_string(),
/// #             port: 0
/// #         },
/// #         cookie: "".to_string(),
/// #         tcp_mss_default: 0,
/// #         target_bitrate: 0,
/// #         fq_rate: 0,
/// #         sock_bufsize: 0,
/// #         sndbuf_actual: 0,
/// #         rcvbuf_actual: 0,
/// #         test_start: TestStart {
/// #             protocol: "".to_string(),
/// #             num_streams: 0,
/// #             blksize: 0,
/// #             omit: 0,
/// #             duration: 0,
/// #             bytes: 0,
/// #             blocks: 0,
/// #             reverse: 0,
/// #             tos: 0,
/// #             target_bitrate: 0,
/// #             bidir: 0,
/// #             fqrate: 0
/// #         }
/// #     },
/// #     intervals: vec![],
/// #     end: End {
/// #         streams: vec![],
/// #         sum_sent: SumSent {
/// #             start: 0.0,
/// #             end: 0.0,
/// #             seconds: 0.0,
/// #             bytes: 0,
/// #             bits_per_second: 0.0,
/// #             retransmits: 0,
/// #             sender: false
/// #         },
/// #         sum_received: SumReceived {
/// #             start: 0.0,
/// #             end: 0.0,
/// #             seconds: 0.0,
/// #             bytes: 0,
/// #             bits_per_second: 0.0,
/// #             sender: false
/// #         },
/// #         cpu_utilization_percent: CpuUtilizationPercent {
/// #             host_total: 0.0,
/// #             host_user: 0.0,
/// #             host_system: 0.0,
/// #             remote_total: 0.0,
/// #             remote_user: 0.0,
/// #             remote_system: 0.0,
/// #         },
/// #         sender_tcp_congestion: "".to_string(),
/// #         receiver_tcp_congestion: "".to_string()
/// #     }
/// # };
/// set_last_result_for_test(dummy_result.clone());
///
/// let result = get_last_result();
/// assert!(result.is_some());
/// assert_eq!(result.unwrap().start.timestamp.timesecs, 0);
///
/// clear_last_result_for_test();
/// assert!(get_last_result().is_none());
/// ```

pub fn get_last_result() -> Option<Iperf3Report> {
    let cache = LAST_RESULT.lock().unwrap();
    cache.as_ref().map(|(result, _)| result.clone())
}

/// Sets the cached iperf3 result. Used for testing purposes.
///
/// # Examples
///
/// ```
/// # use iperf3_statuspage::{Iperf3Report, set_last_result_for_test, get_last_result, Start, ConnectingTo, Timestamp, TestStart, End, SumSent, SumReceived, CpuUtilizationPercent};
/// # let dummy_result = Iperf3Report {
/// #     start: Start {
/// #         connected: vec![],
/// #         version: "".to_string(),
/// #         system_info: "".to_string(),
/// #         timestamp: Timestamp {
/// #             time: "".to_string(),
/// #             timesecs: 0
/// #         },
/// #         connecting_to: ConnectingTo {
/// #             host: "".to_string(),
/// #             port: 0
/// #         },
/// #         cookie: "".to_string(),
/// #         tcp_mss_default: 0,
/// #         target_bitrate: 0,
/// #         fq_rate: 0,
/// #         sock_bufsize: 0,
/// #         sndbuf_actual: 0,
/// #         rcvbuf_actual: 0,
/// #         test_start: TestStart {
/// #             protocol: "".to_string(),
/// #             num_streams: 0,
/// #             blksize: 0,
/// #             omit: 0,
/// #             duration: 0,
/// #             bytes: 0,
/// #             blocks: 0,
/// #             reverse: 0,
/// #             tos: 0,
/// #             target_bitrate: 0,
/// #             bidir: 0,
/// #             fqrate: 0
/// #         }
/// #     },
/// #     intervals: vec![],
/// #     end: End {
/// #         streams: vec![],
/// #         sum_sent: SumSent {
/// #             start: 0.0,
/// #             end: 0.0,
/// #             seconds: 0.0,
/// #             bytes: 0,
/// #             bits_per_second: 0.0,
/// #             retransmits: 0,
/// #             sender: false
/// #         },
/// #         sum_received: SumReceived {
/// #             start: 0.0,
/// #             end: 0.0,
/// #             seconds: 0.0,
/// #             bytes: 0,
/// #             bits_per_second: 0.0,
/// #             sender: false
/// #         },
/// #         cpu_utilization_percent: CpuUtilizationPercent {
/// #             host_total: 0.0,
/// #             host_user: 0.0,
/// #             host_system: 0.0,
/// #             remote_total: 0.0,
/// #             remote_user: 0.0,
/// #             remote_system: 0.0,
/// #         },
/// #         sender_tcp_congestion: "".to_string(),
/// #         receiver_tcp_congestion: "".to_string()
/// #     }
/// # };
/// set_last_result_for_test(dummy_result.clone());
/// let cached = get_last_result().unwrap();
/// assert_eq!(cached.start.timestamp.timesecs, 0);
/// ```

pub fn set_last_result_for_test(result: Iperf3Report) {
    let mut cache = LAST_RESULT.lock().unwrap();
    *cache = Some((result, Instant::now()));
}

/// Clears the cached iperf3 result.
///
/// # Examples
///
/// ```
/// # use iperf3_statuspage::{set_last_result_for_test, clear_last_result_for_test, get_last_result, Iperf3Report};
/// # let dummy_result = Iperf3Report::default();
/// set_last_result_for_test(dummy_result);
/// clear_last_result_for_test();
/// assert!(get_last_result().is_none());
/// ```

pub fn clear_last_result_for_test() {
    let mut cache = LAST_RESULT.lock().unwrap();
    *cache = None;
}

/// HTTP GET endpoint `/iperf3` returns the last cached iperf3 result as JSON.
///
/// Returns HTTP 503 Service Unavailable if no result is cached yet.
#[get("/iperf3")]
pub async fn iperf3() -> impl Responder {
    let cache = LAST_RESULT.lock().unwrap();
    if let Some((cached_result, _timestamp)) = &*cache {
        HttpResponse::Ok().json(cached_result)
    } else {
        HttpResponse::ServiceUnavailable().body("Iperf3 result not available yet.")
    }
}

/// Reads the environment variable `INTERVAL_MINUTES` or returns a default of 10 minutes.
///
/// The duration represents how frequently iperf3 is run.
pub fn min_frequency_duration() -> Duration {
    let minutes = env::var("INTERVAL_MINUTES")
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(10); // default: 10 minutes
    Duration::from_secs(minutes * 60)
}

/// Trait to abstract running the iperf3 command.
///
/// Allows mocking iperf3 execution for testing.
#[async_trait]
pub trait Iperf3Runner: Send + Sync {
    /// Runs iperf3 and returns the raw JSON string output on success.
    async fn run_iperf3(&self, iperf3_ip: String, iperf3_port: String) -> Result<String, String>;
}

/// Real iperf3 runner implementation using the `iperf3` binary.
pub struct RealIperf3Runner;

#[async_trait]
impl Iperf3Runner for RealIperf3Runner {
    async fn run_iperf3(&self, iperf3_ip: String, iperf3_port: String) -> Result<String, String> {
        let output = Command::new("iperf3")
            .arg("-c")
            .arg(iperf3_ip)
            .arg("-p")
            .arg(iperf3_port)
            .arg("--json")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|e| format!("Failed to run iperf3: {}", e))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!("iperf3 failed: {}", stderr))
        }
    }
}

/// Runs the iperf3 test using the provided runner, parses the JSON output, and caches the result.
///
/// Logs errors to stderr if the command or parsing fails.
pub async fn run_iperf3_and_cache_with_runner(runner: &dyn Iperf3Runner, iperf3_ip: String, iperf3_port: String) {
    match runner.run_iperf3(iperf3_ip, iperf3_port).await {
        Ok(stdout) => match serde_json::from_str::<Iperf3Report>(&stdout) {
            Ok(data) => {
                let result = Iperf3Report {
                    start: data.start,
                    intervals: data.intervals,
                    end: data.end,
                };

                let mut cache = LAST_RESULT.lock().unwrap();
                *cache = Some((result.clone(), Instant::now()));
                println!("Iperf3 result updated at {}", result.start.timestamp.time);
            }
            Err(e) => eprintln!("Failed to parse iperf3 JSON: {}", e),
        },
        Err(e) => eprintln!("{}", e),
    }
}

/// Background async task which schedules periodic iperf3 runs.
///
/// The interval between runs is configured by the `INTERVAL_MINUTES` env variable.
pub async fn spawn_iperf3_scheduler(iperf3_ip: String, iperf3_port: String) {
    let interval = min_frequency_duration();
    let runner = RealIperf3Runner;

    // Run one immediately on startup
    run_iperf3_and_cache_with_runner(&runner, iperf3_ip.clone(), iperf3_port.clone()).await;

    let mut ticker = time::interval(interval);
    loop {
        ticker.tick().await;
        run_iperf3_and_cache_with_runner(&runner, iperf3_ip.clone(), iperf3_port.clone()).await;
    }
}

/// Async function to get the cached iperf3 result or return an error if not available.
pub async fn get_cached_iperf3_result() -> Result<Iperf3Report, String> {
    let cache = LAST_RESULT.lock().unwrap();
    if let Some((cached_result, _)) = &*cache {
        Ok(cached_result.clone())
    } else {
        Err("Iperf3 result not available yet.".to_string())
    }
}
