<div align="center">

# rustmond

**A lightweight Linux monitoring daemon written in Rust.**

Collects server health metrics and exposes them via an HTTP API.

[![Rust](https://img.shields.io/badge/Rust-1.85+-orange?logo=rust)](#prerequisites)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Status](https://img.shields.io/badge/Status-Active_Development-brightgreen)](#roadmap)

</div>

---

## Overview

**rustmond** is a monitoring daemon designed to run on a Linux VPS. It periodically samples system metrics — CPU usage, memory consumption, and more — and serves them through a JSON API built on [Axum](https://github.com/tokio-rs/axum). The daemon is architected to support additional data sources, starting with Apache access log ingestion.

The goal is a single, statically-compiled binary that can be deployed as a `systemd` service and queried by dashboards, alerting tools, or any HTTP client.

---

## Features

### System Metrics Collector
Samples host-level metrics on a configurable interval using [`sysinfo`](https://crates.io/crates/sysinfo):
- CPU utilization (%)
- Memory used / total (bytes)

### Apache Log Collector *(in progress)*
A background task designed to tail and parse Apache access logs, extracting:
- Request method & endpoint
- HTTP status codes
- Response size
- Client IP address

### HTTP API
A non-blocking HTTP server exposes collected metrics as JSON.

| Endpoint | Description |
|---|---|
| `GET /health` | Liveness check — returns `"ok"` |
| `GET /metrics/system` | Current CPU and memory metrics |

**Example response** — `GET /metrics/system`:

```json
{
  "cpu": 21.3,
  "memory_used": 1245184000,
  "memory_total": 8589934592
}
```

---

## Architecture

```
┌────────────────────────────────────────────────────┐
│                    rustmond                        │
│                                                    │
│   ┌──────────────────┐    ┌──────────────────┐     │
│   │ System Collector │    │ Apache Collector │     │
│   │   (sysinfo)      │    │   (log parser)   │     │
│   └────────┬─────────┘    └────────┬─────────┘     │
│            │                       │               │
│            ▼                       ▼               │
│        ┌───────────────────────────────┐           │
│        │     Shared Metrics Store      │           │
│        │     Arc<RwLock<MetricsStore>> │           │
│        └──────────────┬────────────────┘           │
│                       │                            │
│                       ▼                            │
│              ┌─────────────────┐                   │
│              │  Axum HTTP API  │                   │
│              │  0.0.0.0:8080   │                   │
│              └─────────────────┘                   │
└────────────────────────────────────────────────────┘
```

**Collectors** run as independent `tokio` tasks that write metrics into a shared, lock-protected store. The **API server** reads from the same store to serve requests — a clean reader/writer separation with no blocking.

---

## Project Structure

```
src/
├── main.rs              # Entrypoint — spawns collectors, starts API server
├── api/
│   ├── mod.rs
│   └── server.rs        # Axum routes and handlers
├── collectors/
│   ├── mod.rs
│   ├── system.rs        # System metrics sampling loop
│   └── apache.rs        # Apache log collector (stub)
├── metrics/
│   ├── mod.rs
│   └── store.rs         # Shared in-memory metrics store
├── lifecycle/
│   ├── mod.rs
│   └── shutdown.rs      # Graceful shutdown handling
└── config/
    └── mod.rs           # Runtime configuration
```

---

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) 1.85+ (edition 2024)

### Build & Run

```bash
# Clone the repository
git clone https://github.com/djmartin2019/rustmond.git
cd rustmond

# Run in development mode
cargo run

# Or build a release binary
cargo build --release
./target/release/rustmond
```

The API server starts on **`http://localhost:8080`**.

### Verify

```bash
curl http://localhost:8080/health
# "ok"

curl http://localhost:8080/metrics/system
# {"cpu":12.5,"memory_used":2147483648,"memory_total":8589934592}
```

---

## Deployment

### Building for Linux

```bash
cargo build --release
```

The output binary at `target/release/rustmond` is a single static executable — copy it to your server and run.

### Running as a systemd Service

Create a unit file at `/etc/systemd/system/rustmond.service`:

```ini
[Unit]
Description=rustmond — system monitoring daemon
After=network.target

[Service]
Type=simple
ExecStart=/opt/rustmond/rustmond
Restart=on-failure
RestartSec=5
User=root

[Install]
WantedBy=multi-user.target
```

Enable and start:

```bash
sudo systemctl daemon-reload
sudo systemctl enable rustmond
sudo systemctl start rustmond
sudo systemctl status rustmond
```

---

## Tech Stack

| Component | Crate | Purpose |
|---|---|---|
| Async runtime | [`tokio`](https://crates.io/crates/tokio) | Task scheduling, networking, timers |
| HTTP framework | [`axum`](https://crates.io/crates/axum) | API routing and request handling |
| System metrics | [`sysinfo`](https://crates.io/crates/sysinfo) | Cross-platform CPU/memory sampling |
| File watching | [`notify`](https://crates.io/crates/notify) | File-system event monitoring |
| Serialization | [`serde`](https://crates.io/crates/serde) / [`serde_json`](https://crates.io/crates/serde_json) | JSON serialization |
| Logging | [`tracing`](https://crates.io/crates/tracing) | Structured, async-aware logging |
| Error handling | [`anyhow`](https://crates.io/crates/anyhow) | Ergonomic error propagation |

---

## Roadmap

- [ ] Apache access log tailing and parsing
- [ ] Requests-per-minute and endpoint frequency metrics
- [ ] Prometheus-compatible `/metrics` export
- [ ] Persistent metric storage (time-series)
- [ ] Web-based dashboard
- [ ] Configuration file support (TOML)
- [ ] Multi-server aggregation

---

## Contributing

Contributions, issues, and feature requests are welcome. Feel free to open an issue or submit a pull request.

---

## License

This project is licensed under the [MIT License](LICENSE).

---

<div align="center">

Built by [David Martin](https://github.com/djmartin2019)

</div>
