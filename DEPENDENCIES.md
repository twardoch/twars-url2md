---
this_file: DEPENDENCIES.md
---

# Dependency Overview

The table below tracks direct dependencies declared in `Cargo.toml`, why they exist, and whether they are runtime-critical or dev-only.

| Crate | Purpose | Runtime |
| --- | --- | --- |
| anyhow | Ergonomic error propagation across async + sync layers. | ✅ |
| base64 | Encode inline assets when needed for Markdown output. | ✅ |
| clap | Structured CLI parsing with help/version handling. | ✅ |
| cssparser | HTML cleanup helpers that rely on CSS parsing. | ✅ |
| curl | Reliable cross-platform HTTP fetches with CDN-friendly headers. | ✅ |
| encoding_rs | Decode multi-encoding responses into UTF-8. | ✅ |
| futures | Stream utilities for async batching. | ✅ |
| html5ever | Robust HTML parsing underpinning link extraction. | ✅ |
| htmd | Primary HTML→Markdown converter. | ✅ |
| indicatif | Progress bar + user feedback during batch runs. | ✅ |
| linkify | Detect inline URLs inside arbitrary text. | ✅ |
| markup5ever | Shared DOM traits required by html5ever. | ✅ |
| markup5ever_rcdom | rcdom tree used while traversing parsed HTML. | ✅ |
| num_cpus | Determine concurrency limits per host. | ✅ |
| once_cell | New lazy statics for regex + LinkFinder caches. | ✅ |
| rayon | Parallel helpers for CPU heavy fallbacks. | ✅ |
| regex | High-performance validation of potential file paths. | ✅ |
| scraper | Structured DOM scraping used in html module. | ✅ |
| sha2 | Hashing for dedupe and metadata. | ✅ |
| tokio | Async runtime covering fetch + file I/O. | ✅ |
| tracing | Structured logging. | ✅ |
| tracing-subscriber | Logging sink + env filter. | ✅ |
| url | URL parsing/normalization. | ✅ |
| tempfile | Fixture helper in tests. | dev |
| mockito | HTTP mocking in tests. | dev |
| monolith | HTML snapshot testing helper. | dev |

> Note: `curl`, `tokio`, and `htmd` remain the three critical runtime pillars (fetching, async orchestration, conversion). `once_cell` was introduced in this iteration to cache expensive regex/LinkFinder state without global mutable singletons.
