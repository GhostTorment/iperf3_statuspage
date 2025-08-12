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

use criterion::{criterion_group, criterion_main, Criterion};
use serde_json;
use iperf3_statuspage::{clear_last_result_for_test, get_last_result, set_last_result_for_test, ConnectingTo, CpuUtilizationPercent, End, Iperf3Report, Start, SumReceived, SumSent, TestStart, Timestamp};

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

fn bench_set_get_clear(c: &mut Criterion) {
    let res = dummy_result();
    c.bench_function("set_get_clear", |b| {
        b.iter(|| {
            set_last_result_for_test(res.clone());
            let _ = get_last_result();
            clear_last_result_for_test();
        })
    });
}

fn bench_serialize(c: &mut Criterion) {
    let res = dummy_result();
    c.bench_function("serialize_iperf3_result", |b| {
        b.iter(|| serde_json::to_string(&res).unwrap())
    });
}

fn bench_speed_endpoint_cached(c: &mut Criterion) {
    set_last_result_for_test(dummy_result());
    c.bench_function("iperf3_endpoint_cached", |b| {
        b.iter(|| {
            let _ = futures::executor::block_on(iperf3_statuspage::get_cached_iperf3_result()).unwrap();
        })
    });
}

criterion_group!(
    benches,
    bench_set_get_clear,
    bench_serialize,
    bench_speed_endpoint_cached
);
criterion_main!(benches);
