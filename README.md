# iperf3-statuspage

A utility application to serve iperf3 results over an HTTP endpoint.

---

## Overview

`iperf3-statuspage` runs periodic iperf3 using the `iperf3` binary, caches the latest results, and exposes them via an HTTP JSON API endpoint (`/iperf3`).  
The service can be configured via environment variables and is built using Rust with Actix-web and Tokio for async task scheduling.

---

## Features

- Periodically runs `iperf3` every N minutes (default: 60).
- Caches the last successful iperf3 result in memory.
- Exposes `/iperf3` HTTP GET endpoint returning the latest cached iperf3 result as JSON.
- Returns HTTP 503 if no cached iperf3 result is available yet.
- Configurable bind address, port, and iperf3 server ip, port and interval via environment variables.

---

## Environment Variables

| Variable             | Description                                | Default     |
|----------------------|--------------------------------------------|-------------|
| `BIND_ADDRESS`       | Address to bind the HTTP server to         | `127.0.0.1` |
| `BIND_PORT`          | Port for the HTTP server                   | `8080`      |
| `INTERVAL_MINUTES`   | Minutes between running iperf3 tests       | `60`        |
| `IPERF3_SERVER_IP`   | IP Address of the Iperf3 Server            | `0.0.0.0`   |
| `IPERF3_SERVER_PORT` | Port of the Iperf3 Server                  | `5201`      |

---

## Usage

1. Ensure `iperf3` is installed and available in your system `PATH`.
2. Ensure you have an iperf3 server running that is accessible by the system running this package.

3. Set environment variables as needed, for example:

    ```shell
    export BIND_ADDRESS=0.0.0.0
    export BIND_PORT=8080
    export INTERVAL_MINUTES=5
    export IPERF_SERVER_IP=0.0.0.0
    export IPERF_SERVER_PORT=5201
    ```

4. Run the application:

    ```shell
    cargo run --release
    ```

5. Access iperf3 results at:

    ```
    http://<BIND_ADDRESS>:<BIND_PORT>/iperf3
    ```

   The endpoint returns JSON with the latest iperf3 data or HTTP 503 if no results are available yet.

---

## Example Response

```json
{
  "start": {
    "connected": [
      {
        "socket": 5,
        "local_host": "127.0.0.1",
        "local_port": 60158,
        "remote_host": "127.0.0.1",
        "remote_port": 5201
      }
    ],
    "version": "iperf 3.16",
    "system_info": "Linux Hostname 6.8.0-71-generic #71-Ubuntu SMP PREEMPT_DYNAMIC Tue Jul 22 16:52:38 UTC 2025 x86_64",
    "timestamp": {
      "time": "Tue, 12 Aug 2025 10:39:42 GMT",
      "timesecs": 1754995182
    },
    "connecting_to": {
      "host": "127.0.0.1",
      "port": 5201
    },
    "cookie": "ep7a45jo7f2rh67vkdmw7phnm4xh6zmytkmm",
    "tcp_mss_default": 32768,
    "target_bitrate": 0,
    "fq_rate": 0,
    "sock_bufsize": 0,
    "sndbuf_actual": 16384,
    "rcvbuf_actual": 131072,
    "test_start": {
      "protocol": "TCP",
      "num_streams": 1,
      "blksize": 131072,
      "omit": 0,
      "duration": 10,
      "bytes": 0,
      "blocks": 0,
      "reverse": 0,
      "tos": 0,
      "target_bitrate": 0,
      "bidir": 0,
      "fqrate": 0
    }
  },
  "intervals": [
    {
      "streams": [
        {
          "socket": 5,
          "start": 0,
          "end": 1.001128,
          "seconds": 1.00112795829773,
          "bytes": 8758493184,
          "bits_per_second": 69989000797.8003,
          "retransmits": 0,
          "snd_cwnd": 1506109,
          "snd_wnd": 6191872,
          "rtt": 35,
          "rttvar": 24,
          "pmtu": 65535,
          "omitted": false,
          "sender": true
        }
      ],
      "sum": {
        "start": 0,
        "end": 1.001128,
        "seconds": 1.00112795829773,
        "bytes": 8758493184,
        "bits_per_second": 69989000797.8003,
        "retransmits": 0,
        "omitted": false,
        "sender": true
      }
    },
    {
      "streams": [
        {
          "socket": 5,
          "start": 1.001128,
          "end": 2.000423,
          "seconds": 0.999294996261597,
          "bytes": 2688024576,
          "bits_per_second": 21519367842.7772,
          "retransmits": 0,
          "snd_cwnd": 1637075,
          "snd_wnd": 6191872,
          "rtt": 25,
          "rttvar": 1,
          "pmtu": 65535,
          "omitted": false,
          "sender": true
        }
      ],
      "sum": {
        "start": 1.001128,
        "end": 2.000423,
        "seconds": 0.999294996261597,
        "bytes": 2688024576,
        "bits_per_second": 21519367842.7772,
        "retransmits": 0,
        "omitted": false,
        "sender": true
      }
    },
  ],
  "end": {
    "streams": [
      {
        "sender": {
          "socket": 5,
          "start": 0,
          "end": 10.000691,
          "seconds": 10.000691,
          "bytes": 86734274560,
          "bits_per_second": 69382625308.5912,
          "retransmits": 0,
          "max_snd_cwnd": 3798014,
          "max_snd_wnd": 6192128,
          "max_rtt": 35,
          "min_rtt": 24,
          "mean_rtt": 26,
          "sender": true
        },
        "receiver": {
          "socket": 5,
          "start": 0,
          "end": 10.000862,
          "seconds": 10.000691,
          "bytes": 86734274560,
          "bits_per_second": 69381438967.961,
          "sender": true
        }
      }
    ],
    "sum_sent": {
      "start": 0,
      "end": 10.000691,
      "seconds": 10.000691,
      "bytes": 86734274560,
      "bits_per_second": 69382625308.5912,
      "retransmits": 0,
      "sender": true
    },
    "sum_received": {
      "start": 0,
      "end": 10.000862,
      "seconds": 10.000862,
      "bytes": 86734274560,
      "bits_per_second": 69381438967.961,
      "sender": true
    },
    "cpu_utilization_percent": {
      "host_total": 102.507701117166,
      "host_user": 0.8030118769529,
      "host_system": 101.704689240213,
      "remote_total": 75.6893571812466,
      "remote_user": 3.12445661004662,
      "remote_system": 72.5648905720749
    },
    "sender_tcp_congestion": "cubic",
    "receiver_tcp_congestion": "cubic"
  }
}

