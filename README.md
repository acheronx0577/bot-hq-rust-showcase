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
| Upstream parity baseline | Frozen SHA `2ff0934` inventory (72 routes, 12 events, 17 tools, 19 tables, desktop/phone viewports) | Source inventory prepared in private repo; rendered verification pending |
| Server and persistence | Axum, Tokio, SQLx, SQLite, HTTP and server-sent events | Planned |
| Clients | Dioxus, WebAssembly, system WebViews, shared CSS, Android and Xcode packaging | Planned |
| Local computer runtime | Debian Linux, X11, VNC, Chromium, Hermes, MCP | Planned |
| Future deployment | Tailscale, Cloudflare Access and Tunnel, encrypted usage-based object storage | Deferred |

## Synthetic two-owner cache rehearsal

The private Rust implementation was exercised with two fictional owners and five synthetic bots per owner against one loopback server. After two cold owner reads populated the cache, the test issued 200 concurrent warm reads. Both owners continued to receive only their own five bots.

| Measurement | Debian 13 local run |
|---|---:|
| Synthetic bots / owners | 10 / 2 |
| Concurrent warm reads | 200 |
| Server requests across cold + warm reads | 2 |
| Warm cache hit rate | 99% |
| Warm read p95 | 54 μs |
| Test process peak RSS | 23.4 MiB |
| Total rehearsal time | 10 ms |

These numbers describe one short local fixture run. The p95 measures in-process cached client reads, not a remote request, and peak RSS includes the test process. It is not a sustained bot-execution or production capacity claim. This public repository publishes the synthetic summary only; it does not include the private application or its test harness.

## Portfolio sample scope

The scheduler here is a runnable standard-library example of per-owner FIFO order with round-robin turns. The full application, server, platform clients, operational configuration and provider integrations remain in the private implementation. Future public examples will use synthetic inputs and reproducible measurements only.

**Attribution.** The original Bot HQ project is by Nick Vasilescu. This Rust port and its offline demonstration are independent work.
