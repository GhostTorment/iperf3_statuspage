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

//! Integration and unit tests for `iperf3-statuspage` internals.
//!
//! These tests verify the behavior of the cached iperf3 results,
//! the HTTP endpoint `/iperf3`, and cache manipulation helpers.
//!
//! Note: Some tests require `actix-web` async runtime and thus
//! are annotated with `#[actix_web::test]` or `#[tokio::test]`.
//!
//! The `serial_test::serial` attribute ensures tests sharing global
//! state run sequentially to avoid race conditions.

use actix_web::{test, http, App};
use serial_test::serial;
use iperf3_statuspage::*;

/// Creates a dummy `Iperf3Report` with fixed example values
/// for use in tests.
fn dummy_result() -> Iperf3Report {
    Iperf3Report {
        start: Start {
            connected: vec![],
            version: "".to_string(),
            system_info: "".to_string(),
            timestamp: Timestamp {
                time: "".to_string(),
                timesecs: 0
            },
            connecting_to: ConnectingTo {
                host: "".to_string(),
                port: 0
            },
            cookie: "".to_string(),
            tcp_mss_default: 0,
            target_bitrate: 0,
            fq_rate: 0,
            sock_bufsize: 0,
            sndbuf_actual: 0,
            rcvbuf_actual: 0,
            test_start: TestStart {
                protocol: "".to_string(),
                num_streams: 0,
                blksize: 0,
                omit: 0,
                duration: 0,
                bytes: 0,
                blocks: 0,
                reverse: 0,
                tos: 0,
                target_bitrate: 0,
                bidir: 0,
                fqrate: 0
            }
        },
        intervals: vec![],
        end: End {
            streams: vec![],
            sum_sent: SumSent {
                start: 0.0,
                end: 0.0,
                seconds: 0.0,
                bytes: 0,
                bits_per_second: 0.0,
                retransmits: 0,
                sender: false
            },
            sum_received: SumReceived {
                start: 0.0,
                end: 0.0,
                seconds: 0.0,
                bytes: 0,
                bits_per_second: 0.0,
                sender: false
            },
            cpu_utilization_percent: CpuUtilizationPercent {
                host_total: 0.0,
                host_user: 0.0,
                host_system: 0.0,
                remote_total: 0.0,
                remote_user: 0.0,
                remote_system: 0.0,
            },
            sender_tcp_congestion: "".to_string(),
            receiver_tcp_congestion: "".to_string()
        }
    }
}

/// Test that the `/iperf3` endpoint returns HTTP 503 Service Unavailable
/// when there is no cached iperf3 result.
///
/// ```rust
/// # use iperf3_statuspage::{clear_last_result_for_test};
/// # // This example shows intent but is not a real doctest (needs actix runtime)
/// clear_last_result_for_test();
/// // HTTP GET /iperf3 should return 503 Service Unavailable
/// ```
#[actix_web::test]
#[serial]
async fn iperf3_returns_service_unavailable_when_no_cache() {
    clear_last_result_for_test();

    let app = test::init_service(App::new().service(iperf3)).await;
    let req = test::TestRequest::get().uri("/iperf3").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::SERVICE_UNAVAILABLE);

    let body = test::read_body(resp).await;
    assert_eq!(body, "Iperf3 result not available yet.");
}

/// Test that the `/iperf3` endpoint returns the cached iperf3 result
/// as JSON with HTTP 200 OK status.
///
/// ```rust
/// # use iperf3_statuspage::{set_last_result_for_test, clear_last_result_for_test};
/// # // This example shows intent but is not a real doctest (needs actix runtime)
/// set_last_result_for_test(dummy_result());
/// // HTTP GET /iperf3 should return 200 OK with JSON body containing cached data
/// clear_last_result_for_test();
/// ```
#[actix_web::test]
#[serial]
async fn iperf3_returns_cached_result() {
    set_last_result_for_test(dummy_result());

    let app = test::init_service(App::new().service(iperf3)).await;
    let req = test::TestRequest::get().uri("/iperf3").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let body = test::read_body(resp).await;
    let result: Iperf3Report = serde_json::from_slice(&body).unwrap();

    assert_eq!(result.start.timestamp.timesecs, 0);
    assert_eq!(result.end.cpu_utilization_percent.host_system, 0.0);

    clear_last_result_for_test();
}

/// Tests the cache helper functions `set_last_result_for_test`,
/// `get_last_result`, and `clear_last_result_for_test` for expected behavior.
///
/// ```rust
/// # use iperf3_statuspage::{set_last_result_for_test, get_last_result, clear_last_result_for_test};
/// # let res = dummy_result();
/// clear_last_result_for_test();
/// assert!(get_last_result().is_none());
///
/// set_last_result_for_test(res.clone());
/// assert_eq!(get_last_result().unwrap().start.timestamp.timesecs, res.start.timestamp.timesecs);
///
/// clear_last_result_for_test();
/// assert!(get_last_result().is_none());
/// ```
#[tokio::test]
async fn test_set_get_clear_last_result_for_test() {
    clear_last_result_for_test();
    assert!(get_last_result().is_none());

    let res = dummy_result();
    set_last_result_for_test(res.clone());
    let cached = get_last_result().unwrap();
    assert_eq!(cached.start.timestamp.timesecs, res.start.timestamp.timesecs);

    clear_last_result_for_test();
    assert!(get_last_result().is_none());
}