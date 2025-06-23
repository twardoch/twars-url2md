# TODO: `twars-url2md` Codebase Modernization Plan

This document outlines the steps to modernize the `twars-url2md` codebase, focusing on improving maintainability, robustness, developer experience, and leveraging modern Rust practices.

## 1. Logging Framework Integration

*   **Goal:** Replace current `eprintln!`-based verbose/debug logging with a structured logging framework.
*   **Tasks:**
    *   [ ] Add `tracing` and `tracing-subscriber` to `Cargo.toml`.
    *   [ ] Configure `tracing-subscriber` (e.g., using `fmt` layer and `EnvFilter` for level control via environment variable like `RUST_LOG`).
    *   [ ] Initialize the subscriber early in `main()`.
    *   [ ] Replace all instances of `eprintln!` used for debugging, verbose output, or warnings with appropriate `tracing` macros (`trace!`, `debug!`, `info!`, `warn!`, `error!`).
    *   [ ] Update `README.md` or provide usage instructions on how to control log levels.

## 2. Error Handling Refinement

*   **Goal:** Standardize error handling using `anyhow` and eliminate panics for recoverable errors.
*   **Tasks:**
    *   [ ] Review `src/error.rs`. Decide if the custom `Error` enum provides significant benefits over `anyhow::Error`.
        *   If kept, ensure it implements `std::error::Error` and has clean `From` implementations for `anyhow::Error` and other error types.
        *   If not, remove it and use `anyhow::Error` or `anyhow::Result` directly.
    *   [ ] Systematically replace all `unwrap()`, `expect()`, and potential panicking operations with `?` or `Result::map_err`/`with_context`.
    *   [ ] Refactor `fetch_html` in `src/html.rs` to handle errors from `monolith` gracefully without relying on `std::panic::catch_unwind`. Explore `monolith`'s error reporting capabilities or use it in a way that's less prone to panicking.
    *   [ ] Remove the top-level `panic::set_hook` and `panic::catch_unwind` in `main.rs` if underlying issues are resolved. The goal is for the application to exit cleanly with an error message via `Result` propagation.

## 3. Dependency Review and Management

*   **Goal:** Ensure dependencies are optimal, up-to-date, and managed effectively.
*   **Tasks:**
    *   [ ] **(Major)** Investigate replacing `openssl` (vendored) with `rustls` for TLS handling in `reqwest`.
        *   Change features for `reqwest` in `Cargo.toml`.
        *   Test thoroughly on all target platforms (Linux, macOS, Windows).
        *   This could simplify cross-compilation and reduce binary size/non-Rust dependencies.
    *   [ ] Re-evaluate `monolith`'s role.
        *   Determine if its full capabilities are necessary or if a lighter HTML fetching/cleaning approach would suffice, potentially reducing complexity and improving robustness.
        *   If kept, ensure its integration is as fault-tolerant as possible.
    *   [ ] Update other dependencies to their latest compatible versions using `cargo update`.
    *   [ ] Run `cargo audit` to check for security vulnerabilities in dependencies and address any findings. This should be added to CI.

## 4. Code Refactoring and Optimization

*   **Goal:** Improve code clarity, reduce duplication, and enhance performance where sensible.
*   **Tasks:**
    *   [ ] In `src/html.rs`:
        *   Refactor `process_url_async` and `process_url_with_content` to share common logic and reduce code duplication. One function could call the other or both could call a common internal helper.
        *   Improve the robustness of `fetch_html`, especially the fallback logic if `monolith` processing fails or is bypassed.
    *   [ ] In `src/url.rs`:
        *   Review URL extraction functions (`extract_urls_from_text`, `extract_urls_from_html_efficient`, `extract_urls_from_html`) for clarity and potential performance bottlenecks. Consolidate if possible.
        *   Consider moving URL *processing* logic (like `process_url_with_retry`, `process_url_with_content`) to `src/html.rs` or a new `src/processor.rs` module, keeping `src/url.rs` focused on URL parsing and path generation.
    *   [ ] Review use of `Arc<Mutex<Vec>>` for `packed_content` in `src/lib.rs`. Ensure this is the most efficient way to collect results, especially concerning locking. For collecting results from async tasks, channels (`tokio::sync::mpsc`) might be an alternative, though the current approach might be fine given the context.

## 5. Enhanced Testing Strategy

*   **Goal:** Increase test coverage and improve the reliability of tests.
*   **Tasks:**
    *   [ ] Write more unit tests for:
        *   `src/url.rs`: Edge cases in URL parsing, base URL resolution, local file path detection.
        *   `src/html.rs`: Different HTML structures, error conditions during fetching/processing, content-type handling.
        *   `src/markdown.rs`: Various HTML inputs and their expected Markdown output.
        *   `src/cli.rs`: Argument parsing logic and configuration creation.
    *   [ ] Integrate `mockito` (already a dev-dependency) or `wiremock-rs` to mock HTTP server responses. This will allow testing of `src/html.rs`'s network interaction logic (retries, error handling) reliably without actual network calls.
    *   [ ] Develop integration tests:
        *   These tests should compile the binary and run it as a subprocess.
        *   Test various CLI argument combinations.
        *   Verify file outputs (existence, naming, content) and stdout.
        *   Test with sample input files containing URLs and local paths.
    *   [ ] Set up code coverage reporting using `cargo-tarpaulin` or `grcov`.

## 6. Build and CI/CD Enhancements

*   **Goal:** Make the build process more robust, secure, and informative.
*   **Tasks:**
    *   [ ] Add code coverage reporting (e.g., `cargo-tarpaulin`) to the CI workflow (`.github/workflows/ci.yml`). Upload coverage reports to a service like Codecov or Coveralls, or as a GitHub artifact.
    *   [ ] Add `cargo audit` to the CI workflow to check for vulnerable dependencies.
    *   [ ] Ensure `cargo fmt --check` and `cargo clippy --all-targets --all-features -- -D warnings` are strictly enforced in CI.

## 7. Streamline Release Process

*   **Goal:** Automate and simplify the steps involved in releasing new versions.
*   **Tasks:**
    *   [ ] Introduce `cargo-release` as a development dependency or recommend its global installation for maintainers.
    *   [ ] Update `README.md` (`Development` -> `Publishing` section) with instructions on using `cargo-release` to manage version bumps, tagging, and publishing.
    *   [ ] Evaluate if the GitHub Actions `publish-crate` job can be simplified or triggered more directly by `cargo-release` conventions (e.g., publishing on tag push is already good).

## 8. Documentation Improvements

*   **Goal:** Ensure documentation is comprehensive, up-to-date, and useful for both users and contributors (including AI).
*   **Tasks:**
    *   [ ] Create `AGENTS.md` in the repository root. This file should include:
        *   Guidance on coding style (e.g., "follow `rustfmt` and `clippy` recommendations").
        *   Instructions for running tests, including any specific setup for integration tests.
        *   Key architectural decisions or module responsibilities.
        *   Notes on how to handle dependencies or build issues.
    *   [ ] Review and enhance inline code documentation (Rustdoc comments `///` and `//!`). Focus on public APIs, complex functions, and any non-obvious logic.
    *   [ ] Ensure `README.md` is updated to reflect any changes in CLI options, behavior, or build/contribution process resulting from this modernization effort.

## 9. Final Review and Submission

*   **Goal:** Ensure all changes are correct, well-tested, and meet the project's standards.
*   **Tasks:**
    *   [ ] Perform a full round of testing: `cargo test --all-features`.
    *   [ ] Run linters and formatters: `cargo fmt`, `cargo clippy --all-targets --all-features -- -D warnings`.
    *   [ ] Manually test the CLI with a diverse set of inputs (various URLs, local files, stdin, different output options).
    *   [ ] Review all code changes for correctness, clarity, and adherence to the modernization goals.
    *   [ ] Commit changes with a clear, descriptive message and submit.

This `TODO.md` will serve as a checklist for the modernization effort.
