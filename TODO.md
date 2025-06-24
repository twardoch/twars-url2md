# [ ] `twars-url2md` Modernization TODO List

# [ ] Modernization Goals

This document outlines the steps to modernize the `twars-url2md` codebase, focusing on improving maintainability, robustness, developer experience, and leveraging modern Rust practices.

## [ ] 1. Implement smart processing

Unless **`-a` / `--all`** is specified, the scraper applies the following heuristic pipeline to isolate what humans would intuitively regard as the _main_ portion of an HTML page.

### [ ] 1.1. Pre‑flight cleanup

1. **Strip non‑content tags**: remove every `script`, `style`, `noscript`, `template`, `iframe`, `svg`, and `canvas` node.
2. **Normalise whitespace**: collapse consecutive whitespace and new‑line characters so that subsequent length measurements are stable.
3. **Discard empty elements**: recursively prune elements that are now empty or contain fewer than _N_ visible characters (default =`25`).

### [ ] 1.2. Single‑element fast path

_If_ exactly **one** `<main>` _or_ **one** `<article>` element remains **after** the pre‑flight cleanup, that element is assumed to hold the canonical content.

- Return its **inner HTML** as the result and **abort** the algorithm.

### [ ] 1.3. Structural pruning

If the fast path did **not** trigger:

**Drop obvious chrome**: delete every `header`, `footer`, `aside`, `nav`, and `form` element (these overwhelmingly host site navigation, sidebars, ads, or comment boxes).

### [ ] 1.4. Behaviour of `-a / --all`

Passing **`-a`** bypasses _all_ of the above logic: the tool will emit the fully cleaned `<body>` after _Pre‑flight cleanup_ (step 1) but before any structural heuristics are applied. This is useful when you explicitly want _everything_ that could conceivably be part of the page's content, for example when converting documentation sites that embed code samples in sidebars.

## [ ] 2. Critical Bug Fixes (HIGH PRIORITY)

### [ ] 2.1. Fix Help Option Not Working (REGRESSION)

- **Issue:** Running `twars-url2md -h` or `twars-url2md --help` produces no output
- **Root Cause:** The custom argument parsing in `cli.rs:parse_args()` exits with code 0 but doesn't let Clap print the help message
- **Tasks:**
  - [ ] Fix the help handling in `parse_args()` to ensure Clap's help text is displayed before exiting
  - [ ] Test that both `-h` and `--help` work correctly
  - [ ] Ensure `--version` also displays properly

### [x] 2.2. Enhance Error Message for Missing Input

- **Issue:** When running `twars-url2md` without arguments, the error message is too brief
- **Current:** "Error: Either --stdin or --input must be specified\nRun with --help for usage information"
- **Tasks:**
  - [x] Update error message to include a brief usage example
  - [x] Consider showing a condensed help message when no arguments are provided
  - [x] Make the error message more helpful for first-time users

### [ ] 2.3. Fix URL Processing Stalling Issue

- **Issue:** Processing URLs (e.g., `echo "https://helpx.adobe.com/pl/indesign/using/using-fonts.html" | twars-url2md --stdin`) stalls after printing "Processing URL:" message
- **Root Cause Analysis:**
  - The code creates directories but doesn't complete the processing
  - The monolith processing might be hanging or timing out
  - No timeout is set for the HTTP requests or monolith processing
- **Tasks:**
  - [ ] Add comprehensive timeout handling for HTTP requests
  - [ ] Add timeout for monolith processing in `spawn_blocking` task
  - [ ] Add more detailed progress logging to identify where the stall occurs
  - [ ] Ensure error messages are properly propagated when processing fails
  - [ ] Test with various URLs to ensure robustness
  - [x] Refactor `fetch_html` in `src/html.rs` to handle errors from `monolith` gracefully without relying on `std::panic::catch_unwind`. Explore `monolith`'s error reporting capabilities or use it in a way that's less prone to panicking.
  - [ ] Remove the top-level `panic::set_hook` and `panic::catch_unwind` in `main.rs` if underlying issues are resolved. The goal is for the application to exit cleanly with an error message via `Result` propagation.

### [ ] 2.4. Fix Output Writing Issues

- **Issue:** When using `-p out.md` or `-o out`, no output files are created
- **Root Cause:** Need to verify the output writing logic in both pack and regular modes
- **Tasks:**
  - [ ] Debug why files aren't being written when output options are specified
  - [ ] Ensure parent directories are created properly
  - [ ] Add logging to confirm when files are successfully written
  - [ ] Test all output modes: stdout, single file, directory structure, and pack mode
  - [ ] Add helper `fn output_mode(path: &Path) -> OutputMode` where `enum OutputMode { Directory(PathBuf), SingleFile(PathBuf) }`.
  - [ ] In `Cli::create_config` detect `path.extension() == Some("md".as_ref())` ⇒ `SingleFile` else `Directory`.
  - [ ] Propagate this choice via `Config` (replace the boolean pair `single_file`/`has_output` with `output_kind: OutputKind`).
  - [ ] Update `lib::process_urls` to:
     • write each URL into its own file when `Directory`, preserving the current hierarchy logic;  
     • write (append) all Markdown into the single file when `SingleFile`, prefixed with `# {url}` separators (same formatter used by `--pack`).
  - [ ] Make `--pack` mutually exclusive with single-file mode. Clap can enforce this via `.conflicts_with("output")` when `output` ends in `.md`.
  - [ ] Ensure parent directories exist for both modes using `tokio::fs::create_dir_all` with error propagation.
  - [ ] Add extensive logging: `tracing::debug!(%path, "wrote file")` for every successful write; `tracing::error!` for failures.
  - [ ] Integration tests (`tests/output_modes.rs`) covering:
       - directory mode (`-o outdir`);
       - single-file mode (`-o out.md`);
       - pack mode (`-p combined.md` with multiple URLs);
       verify file existence and that the first 10 chars of content are non-empty Markdown.

## [ ] 3. Logging Framework Integration

- **Goal:** Replace current `eprintln!`-based verbose/debug logging with a structured logging framework.
- **Tasks:**
  - [x] Add `tracing` and `tracing-subscriber` to `Cargo.toml`.
  - [x] Configure `tracing-subscriber` (e.g., using `fmt` layer and `EnvFilter` for level control via environment variable like `RUST_LOG`).
  - [x] Initialize the subscriber early in `main()`.
  - [x] Replace all instances of `eprintln!` used for debugging, verbose output, or warnings with appropriate `tracing` macros (`trace!`, `debug!`, `info!`, `warn!`, `error!`).
  - [ ] Update `README.md` or provide usage instructions on how to control log levels.

## [ ] 4. Error Handling Refinement

- **Goal:** Standardize error handling using `anyhow` and eliminate panics for recoverable errors.
- **Tasks:**
  - [x] Review `src/error.rs`. Decide if the custom `Error` enum provides significant benefits over `anyhow::Error`.
    - If kept, ensure it implements `std::error::Error` and has clean `From` implementations for `anyhow::Error` and other error types.
    - If not, remove it and use `anyhow::Error` or `anyhow::Result` directly.
  - [ ] Systematically replace all `unwrap()`, `expect()`, and potential panicking operations with `?` or `Result::map_err`/`with_context`.
  - [x] Refactor `fetch_html`

## [ ] 5. Dependency Review and Management

- **Goal:** Ensure dependencies are optimal, up-to-date, and managed effectively.
- **Tasks:**
  - [ ] **(Major)** Investigate replacing `openssl` (vendored) with `rustls` for TLS handling in `reqwest`.
    - Change features for `reqwest` in `Cargo.toml`.
    - Test thoroughly on all target platforms (Linux, macOS, Windows).
    - This could simplify cross-compilation and reduce binary size/non-Rust dependencies.
  - [ ] Re-evaluate `monolith`'s role.
    - Determine if its full capabilities are necessary or if a lighter HTML fetching/cleaning approach would suffice, potentially reducing complexity and improving robustness.
    - If kept, ensure its integration is as fault-tolerant as possible.
  - [ ] Update other dependencies to their latest compatible versions using `cargo update`.
  - [ ] Run `cargo audit` to check for security vulnerabilities in dependencies and address any findings. This should be added to CI.

## [ ] 6. Code Refactoring and Optimization

- **Goal:** Improve code clarity, reduce duplication, and enhance performance where sensible.
- **Tasks:**
  - [x] In `src/html.rs`:
    - Refactor `process_url_async` and `process_url_with_content` to share common logic and reduce code duplication. One function could call the other or both could call a common internal helper.
    - Improve the robustness of `fetch_html`, especially the fallback logic if `monolith` processing fails or is bypassed.
  - [x] In `src/url.rs`:
    - Review URL extraction functions (`extract_urls_from_text`, `extract_urls_from_html_efficient`, `extract_urls_from_html`) for clarity and potential performance bottlenecks. Consolidate if possible.
    - Consider moving URL _processing_ logic (like `process_url_with_retry`, `process_url_with_content`) to `src/html.rs` or a new `src/processor.rs` module, keeping `src/url.rs` focused on URL parsing and path generation.
  - [ ] Review use of `Arc<Mutex<Vec>>` for `packed_content` in `src/lib.rs`. Ensure this is the most efficient way to collect results, especially concerning locking. For collecting results from async tasks, channels (`tokio::sync::mpsc`) might be an alternative, though the current approach might be fine given the context.

## [ ] 7. Enhanced Testing Strategy

- **Goal:** Increase test coverage and improve the reliability of tests.
- **Tasks:**
  - [ ] Write more unit tests for:
    - `src/url.rs`: Edge cases in URL parsing, base URL resolution, local file path detection.
    - `src/html.rs`: Different HTML structures, error conditions during fetching/processing, content-type handling.
    - `src/markdown.rs`: Various HTML inputs and their expected Markdown output.
    - `src/cli.rs`: Argument parsing logic and configuration creation.
  - [ ] Integrate `mockito` (already a dev-dependency) or `wiremock-rs` to mock HTTP server responses. This will allow testing of `src/html.rs`'s network interaction logic (retries, error handling) reliably without actual network calls.
  - [ ] Develop integration tests:
    - These tests should compile the binary and run it as a subprocess.
    - Test various CLI argument combinations.
    - Verify file outputs (existence, naming, content) and stdout.
    - Test with sample input files containing URLs and local paths.
  - [ ] Set up code coverage reporting using `cargo-tarpaulin` or `grcov`.

## [ ] 8. Build and CI/CD Enhancements

- **Goal:** Make the build process more robust, secure, and informative.
- **Tasks:**
  - [ ] Add code coverage reporting (e.g., `cargo-tarpaulin`) to the CI workflow (`.github/workflows/ci.yml`). Upload coverage reports to a service like Codecov or Coveralls, or as a GitHub artifact.
  - [ ] Add `cargo audit` to the CI workflow to check for vulnerable dependencies.
  - [ ] Ensure `cargo fmt --check` and `cargo clippy --all-targets --all-features -- -D warnings` are strictly enforced in CI.

## [ ] 9. Streamline Release Process

- **Goal:** Automate and simplify the steps involved in releasing new versions.
- **Tasks:**
  - [ ] Introduce `cargo-release` as a development dependency or recommend its global installation for maintainers.
  - [ ] Update `README.md` (`Development` → `Publishing` section) with instructions on using `cargo-release` to manage version bumps, tagging, and publishing.
  - [ ] Evaluate if the GitHub Actions `publish-crate` job can be simplified or triggered more directly by `cargo-release` conventions (e.g., publishing on tag push is already good).

## [ ] 10. Documentation Improvements

- **Goal:** Ensure documentation is comprehensive, up-to-date, and useful for both users and contributors (including AI).
- **Tasks:**
  - [ ] Create `AGENTS.md` in the repository root. This file should include:
    - Guidance on coding style (e.g., "follow `rustfmt` and `clippy` recommendations").
    - Instructions for running tests, including any specific setup for integration tests.
    - Key architectural decisions or module responsibilities.
    - Notes on how to handle dependencies or build issues.
  - [ ] Review and enhance inline code documentation (Rustdoc comments `///` and `//!`). Focus on public APIs, complex functions, and any non-obvious logic.
  - [ ] Ensure `README.md` is updated to reflect any changes in CLI options, behavior, or build/contribution process resulting from this modernization effort.

## [ ] 11. Final Review and Submission

- **Goal:** Ensure all changes are correct, well-tested, and meet the project's standards.
- **Tasks:**
  - [ ] Perform a full round of testing: `cargo test --all-features`.
  - [ ] Run linters and formatters: `cargo fmt`, `cargo clippy --all-targets --all-features -- -D warnings`.
  - [ ] Manually test the CLI with a diverse set of inputs (various URLs, local files, stdin, different output options).
  - [ ] Review all code changes for correctness, clarity, and adherence to the modernization goals.
  - [ ] Commit changes with a clear, descriptive message and submit.

This `TODO.md` will serve as a checklist for the modernization effort.