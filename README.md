# rustmond

**rustmond** is a lightweight Rust-based monitoring daemon designed to
run directly on a Linux VPS.

It collects system metrics and web traffic data and exposes them through
a simple HTTP API for inspection or future dashboards.

The project exists primarily as a learning exercise in:

-   Systems programming
-   Linux server internals
-   Rust async architecture
-   Observability tooling

In simpler terms:

> Stop building SaaS toys and start building tools servers actually care
> about.

------------------------------------------------------------------------

# Features (MVP)

The current MVP includes:

### System Metrics Collector

Collects host metrics using the `sysinfo` crate:

-   CPU usage
-   Memory usage
-   System resource statistics

Metrics are sampled periodically and stored in memory.

### Apache Traffic Collector (Stub)

A background collector designed to ingest Apache access logs.\
The MVP currently includes a placeholder collector that will later
parse:

-   request method
-   endpoint
-   status codes
-   response size
-   client IP

### HTTP Metrics API

An HTTP server exposes monitoring data.

Available endpoints:

    GET /health
    GET /metrics/system

Example response:

``` json
{
  "cpu": 21.3,
  "memory_used": 1245184000,
  "memory_total": 8589934592
}
```

------------------------------------------------------------------------

# Architecture

The monitoring agent is composed of three main subsystems.

    Apache logs ──► Apache Collector
                         │
                         ▼
                    Metrics Store
                         ▲
    System Metrics ─► System Collector
                         │
                         ▼
                       API Server

## Components

### Collectors

Background tasks responsible for collecting data:

-   `collectors/system.rs` -- gathers system metrics
-   `collectors/apache.rs` -- parses Apache access logs (future)

### Metrics Store

A shared in‑memory state store accessed by all collectors and the API.

Implemented with:

    Arc<RwLock<MetricsStore>>

Collectors write metrics.\
The API reads metrics.

### API Server

A lightweight HTTP server built with **Axum** that exposes monitoring
data.

------------------------------------------------------------------------

# Project Structure

    src
    ├ api
    │  ├ mod.rs
    │  └ server.rs
    │
    ├ collectors
    │  ├ mod.rs
    │  ├ apache.rs
    │  └ system.rs
    │
    ├ metrics
    │  ├ mod.rs
    │  └ store.rs
    │
    ├ lifecycle
    │  ├ mod.rs
    │  └ shutdown.rs
    │
    ├ config
    │  └ mod.rs
    │
    └ main.rs

------------------------------------------------------------------------

# Running Locally

Install Rust if needed:

    curl https://sh.rustup.rs -sSf | sh

Run the project:

    cargo run

The API server will start on:

    http://localhost:8080

Test endpoints:

    curl http://localhost:8080/health
    curl http://localhost:8080/metrics/system

------------------------------------------------------------------------

# Deployment

The intended deployment target is a Linux VPS.

Typical workflow:

    git clone https://github.com/<user>/rustmond
    cd rustmond
    cargo build --release

Binary location:

    target/release/rustmond

------------------------------------------------------------------------

# Running as a Linux Daemon

Example `systemd` service:

    /etc/systemd/system/rustmond.service

    [Unit]
    Description=Rust Monitoring Agent
    After=network.target

    [Service]
    ExecStart=/opt/rustmond/rustmond
    Restart=always
    User=root

    [Install]
    WantedBy=multi-user.target

Enable service:

    sudo systemctl daemon-reload
    sudo systemctl enable rustmond
    sudo systemctl start rustmond

------------------------------------------------------------------------

# Future Roadmap

Planned improvements:

-   Apache log tailing
-   Requests per minute metrics
-   Endpoint frequency statistics
-   Prometheus compatible metrics
-   Web dashboard
-   Persistent metrics storage
-   Multi-server monitoring

The long-term goal is to evolve this project into a lightweight
observability stack.

------------------------------------------------------------------------

# Why Rust

Rust is ideal for infrastructure tooling because it provides:

-   native performance
-   memory safety
-   zero-cost abstractions
-   single static binaries
-   excellent async runtime support

Many modern observability tools are written in Rust for these reasons.

------------------------------------------------------------------------

# License

MIT

------------------------------------------------------------------------

# Author

David Martin\
https://github.com/djmartin2019

