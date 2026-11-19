# Bot HQ in Rust

A portfolio project about porting [Nick Vasilescu’s Bot HQ](https://github.com/nickvasilescu/bot-hq) to Rust. The engineering goal is to preserve its interface and behavior across Windows, Android, iOS and the web while letting two people keep separate profiles on a shared computer.

## Run the Rust demo

```sh
cargo run --locked
cargo test --locked
```

Two fictional owners submit five jobs to one computer slot. The program prints their turns. Sam gets a turn before Alex’s second job, and each owner’s jobs keep their submission order. The example runs locally with the Rust standard library. It uses no network connections, API keys or hosting.

## Engineering stack

| Area | Technologies | Status |
|---|---|---|
| Public scheduling example | Rust, Cargo, standard-library queues, unit tests | Runnable here |
| Shared client behavior | Rust screen-state labels, fallback dimensions and capacity rules | Initial rules ported and unit-tested in the private project |
| Server and persistence | Axum, Tokio, SQLx, SQLite, HTTP and server-sent events | Planned |
| Clients | Dioxus, WebAssembly, system WebViews, shared CSS, Android and Xcode packaging | Planned |
| Local computer runtime | Debian Linux, X11, VNC, Chromium, Hermes, MCP | Planned |
| Future deployment | Tailscale, Cloudflare Access and Tunnel, encrypted usage-based object storage | Deferred |

## What will be demonstrated next

The first client milestone compares the upstream and Rust profile workflows using synthetic data. Subsequent milestones will add reproducible screenshots, platform results and performance measurements as the implementation produces them. The scheduler here illustrates a design decision. The production scheduler, applications and operational configuration belong to the private implementation.

**Attribution.** The original Bot HQ project is by Nick Vasilescu. This Rust port and its offline demonstration are independent work.
