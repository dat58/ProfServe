# ProfServe - HTTP-Enabled Profiling for Rust

[![Documentation](https://docs.rs/profserve/badge.svg)](https://docs.rs/profserve)
[![Crates.io](https://img.shields.io/crates/v/profserve.svg)](https://crates.io/crates/profserve)
[![License](https://img.shields.io/crates/l/profserve.svg)](https://github.com/dat58/ProfServe/blob/main/LICENSE)
[![Download](https://img.shields.io/crates/d/profserve.svg)](https://crates.io/crates/profserve)

ProfServe is a Rust crate that provides HTTP endpoints for performance profiling, built on top of `pprof-rs`. It enables remote profiling capabilities for your Rust applications with flamegraph generation and pprof data export.

## Features

- 🚀 **HTTP API** for remote profiling
- 🔥 **Flamegraph generation** in SVG format
- 🔒 **Async-ready** for modern Rust applications

## Quick Start

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
profserve = "0.1"
```

### Basic Usage

```rust
fn main() {
    std::thread::spawn(|| profserve::http::serve(8080).expect("Failed to start profserve"));
    // your code goes here
}
```

### Making Requests

Get a 10-second profile as flamegraph:

```bash
curl -s "http://localhost:8080/prof/cpu?seconds=10&frequency=100&output=flamegraph" > profile.svg
```

Get raw text pprof data:

```bash
curl -s "http://localhost:8080/prof/cpu?seconds=10&output=text" > profile.txt
```

## API Endpoints

| Endpoint         | Method | Parameters                       | Description |
|------------------|--------|----------------------------------|-------------|
| `/prof/cpu`      | GET | `seconds`, `output`, `frequency` | Capture profile |
