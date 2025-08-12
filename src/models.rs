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

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Iperf3Report {
    pub start: Start,
    pub intervals: Vec<Interval>,
    pub end: End,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Start {
    pub connected: Vec<Connected>,
    pub version: String,
    pub system_info: String,
    pub timestamp: Timestamp,
    pub connecting_to: ConnectingTo,
    pub cookie: String,
    pub tcp_mss_default: u32,
    pub target_bitrate: u64,
    pub fq_rate: u64,
    pub sock_bufsize: u64,
    pub sndbuf_actual: u64,
    pub rcvbuf_actual: u64,
    pub test_start: TestStart,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Connected {
    pub socket: u32,
    pub local_host: String,
    pub local_port: u16,
    pub remote_host: String,
    pub remote_port: u16,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Timestamp {
    pub time: String,
    pub timesecs: u64,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConnectingTo {
    pub host: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TestStart {
    pub protocol: String,
    pub num_streams: u32,
    pub blksize: u64,
    pub omit: u64,
    pub duration: u64,
    pub bytes: u64,
    pub blocks: u64,
    pub reverse: u32,
    pub tos: u32,
    pub target_bitrate: u64,
    pub bidir: u32,
    pub fqrate: u64,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Interval {
    pub streams: Vec<Stream>,
    pub sum: Sum,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Stream {
    pub socket: u32,
    pub start: f64,
    pub end: f64,
    pub seconds: f64,
    pub bytes: u64,
    pub bits_per_second: f64,
    pub retransmits: u32,
    pub snd_cwnd: u64,
    pub snd_wnd: u64,
    pub rtt: u32,
    pub rttvar: u32,
    pub pmtu: u32,
    pub omitted: bool,
    pub sender: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Sum {
    pub start: f64,
    pub end: f64,
    pub seconds: f64,
    pub bytes: u64,
    pub bits_per_second: f64,
    pub retransmits: u32,
    pub omitted: bool,
    pub sender: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct End {
    pub streams: Vec<EndStream>,
    pub sum_sent: SumSent,
    pub sum_received: SumReceived,
    pub cpu_utilization_percent: CpuUtilizationPercent,
    pub sender_tcp_congestion: String,
    pub receiver_tcp_congestion: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct EndStream {
    pub sender: Sender,
    pub receiver: Receiver,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Sender {
    pub socket: u32,
    pub start: f64,
    pub end: f64,
    pub seconds: f64,
    pub bytes: u64,
    pub bits_per_second: f64,
    pub retransmits: u32,
    pub max_snd_cwnd: u64,
    pub max_snd_wnd: u64,
    pub max_rtt: u32,
    pub min_rtt: u32,
    pub mean_rtt: u32,
    pub sender: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Receiver {
    pub socket: u32,
    pub start: f64,
    pub end: f64,
    pub seconds: f64,
    pub bytes: u64,
    pub bits_per_second: f64,
    pub sender: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SumSent {
    pub start: f64,
    pub end: f64,
    pub seconds: f64,
    pub bytes: u64,
    pub bits_per_second: f64,
    pub retransmits: u32,
    pub sender: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SumReceived {
    pub start: f64,
    pub end: f64,
    pub seconds: f64,
    pub bytes: u64,
    pub bits_per_second: f64,
    pub sender: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CpuUtilizationPercent {
    pub host_total: f64,
    pub host_user: f64,
    pub host_system: f64,
    pub remote_total: f64,
    pub remote_user: f64,
    pub remote_system: f64,
}