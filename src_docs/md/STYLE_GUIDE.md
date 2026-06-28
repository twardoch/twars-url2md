---
this_file: src_docs/md/STYLE_GUIDE.md
---

# Style Guide

Conventions for contributing code, tests, and documentation to `twars-url2md`.

---

## Rust code

### Formatting and linting

- **`cargo fmt`** is mandatory before every commit.  The CI `cargo fmt --check` step enforces it.
- **`cargo clippy --all-targets -- -D warnings`** must produce zero diagnostics.
- **Edition 2021** — use modern idioms (`let`-`else`, `if let` chains, `?` everywhere).

### Naming

| Item | Convention | Example |
|------|-----------|---------|
| Types, traits, enums | `UpperCamelCase` | `Config`, `ProcessingError` |
| Functions, methods, variables | `snake_case` | `process_urls`, `output_base` |
| Constants | `SCREAMING_SNAKE_CASE` | `USER_AGENT_STRING`, `PROGRESS_FILE` |
| Modules | `snake_case` | `html`, `markdown`, `url` |

### Error handling

- Use `anyhow::Result` and `?` for propagation in application code.
- Error messages must follow **what → why → how to fix**:
  ```
  "Cannot create output dir {path}: {cause}\n\
   Check write permissions or specify a different --output location."
  ```
- Never `unwrap()` in production code paths; use `expect("invariant explanation")` only in `build.rs` or truly impossible cases.

### Async code

- `async fn` in `lib.rs` and `html.rs`; blocking I/O goes inside `tokio::task::spawn_blocking`.
- `Arc<T>` for shared data across task boundaries; `tokio::sync::Mutex` when mutation is needed.
- Keep closures passed to `stream::iter(...).map(...)` as `async move` blocks — clone `Arc` handles before the closure, not inside.

### Documentation comments

- Every `pub` item needs a `///` doc comment with at least one sentence.
- Private helpers that have non-obvious semantics need an inline `//` comment.
- Module-level `//!` docs must describe the module's role in the pipeline and reference the other modules it interacts with.
- `# Examples` sections in public API docs must compile (`cargo test --doc`).

---

## Tests

### Test naming

```rust
fn test_<function>_when_<condition>_then_<result>()
```

Examples:
- `test_parse_progress_url_valid`
- `test_load_progress_nonexistent_dir_returns_empty`
- `test_progress_file_restarts_on_output_dir_change`

### Test categories

| Category | Location | Scope |
|----------|----------|-------|
| Unit | `src/<module>.rs` `#[cfg(test)]` | Single function, no I/O |
| Integration | `tests/integration/` | Full pipeline, mockito HTTP server |
| URL extraction | `tests/unit/url_tests.rs` | `extract_urls_from_text` edge cases |
| Benchmarks | `tests/benchmarks.rs` | Wall-clock assertions, not `criterion` |

### Network tests

- Integration tests use `mockito` to intercept HTTP calls — no real network access.
- Local `file://` URLs are used for unit tests that need HTML conversion without network.
- Tests that require a live internet connection must be marked `#[ignore]` with a doc comment explaining why.

### Assert messages

Every `assert!` and `assert_eq!` must carry a descriptive message:

```rust
assert!(path.exists(), "progress file should be created in output dir after processing");
assert_eq!(errors.len(), 0, "all URLs should succeed; got: {:?}", errors);
```

---

## Git commits

- One logical change per commit.
- Commit message: imperative mood, present tense, ≤ 72 chars on the first line.
- Body (optional): *why* the change was made, not *what* — the diff already shows what.

Examples:
```
Add crash-safe JSONL progress tracking
Fix CDN bot-detection by adding Sec-Fetch headers
Bump tokio minimum version to 1.38
```

---

## Documentation pages

- Source lives in `src_docs/md/`.  The built site goes to `docs/` (git-tracked for GitHub Pages).
- Build with: `mkdocs build -f src_docs/mkdocs.yml`
- Serve locally with: `mkdocs serve -f src_docs/mkdocs.yml`
- Use Mermaid for architecture diagrams; keep diagrams up to date when module structure changes.
- Code snippets in docs must reflect the actual implementation, not aspirational pseudocode.

### Page front-matter

Every Markdown page should include a `this_file` front-matter field:

```yaml
---
this_file: src_docs/md/page-name.md
---
```

---

## Dependency management

- Prefer the **smallest** crate that solves the problem; check star count and last-commit date.
- Add new dependencies to `DEPENDENCIES.md` with a one-line rationale.
- Run `cargo deny check` after adding or updating dependencies to verify licence compatibility and absence of known advisories.
- Keep `[dev-dependencies]` separate from `[dependencies]` — dev-only crates must not appear in the release binary.

---

## Releasing

See `publish.sh` for the release workflow.  In brief:

1. Update `CHANGELOG.md` under `## [Unreleased]`.
2. Tag: `git tag v<version>`.
3. `cargo publish --dry-run` — verify the package.
4. `cargo publish`.
5. `mkdocs build -f src_docs/mkdocs.yml` then push `docs/`.
