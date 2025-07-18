# https://github.com/twardoch/twars-url2md

## 1. Project Overview

`twars-url2md` is a Rust CLI application that downloads URLs and converts them to clean, readable Markdown files. It can extract URLs from various text formats and process them concurrently.

## 2. Commands

### 2.1. Development
```bash
# Run tests
cargo test --all-features

# Format code
cargo fmt

# Run linter
cargo clippy --all-targets --all-features

# Build release binary
cargo build --release

# Run with arguments
cargo run -- [OPTIONS]
```

### 2.2. Publishing
```bash
# Verify package before publishing
cargo package

# Publish to crates.io
cargo publish
```

## 3. Architecture

The codebase follows a modular design with clear separation of concerns:

- `src/main.rs` - Entry point with panic handling wrapper
- `src/lib.rs` - Core processing logic and orchestration
- `src/cli.rs` - Command-line interface using Clap
- `src/url.rs` - URL extraction and validation logic
- `src/html.rs` - HTML fetching and cleaning using Monolith
- `src/markdown.rs` - HTML to Markdown conversion using htmd

### 3.1. Key Design Decisions

1. **Panic Recovery**: The application catches panics from the Monolith library to prevent crashes during HTML processing (see `src/main.rs:10-20`).

2. **Concurrent Processing**: Uses Tokio with adaptive concurrency based on available CPUs for parallel URL processing (see `src/lib.rs:107-120`).

3. **Error Handling**: Uses `anyhow` for error propagation with custom error types and retry logic for network failures.

4. **Output Organization**: Generates file paths based on URL structure when using `-o` option, creating a repository-like structure.

## 4. Testing

Run tests with `cargo test --all-features`. The test module is in `src/tests.rs` and includes unit tests for URL extraction and processing logic.

## 5. Dependencies

- **Async Runtime**: tokio with full features
- **HTTP Client**: reqwest with vendored OpenSSL
- **HTML Processing**: monolith for cleaning, htmd for Markdown conversion
- **CLI**: clap with derive feature
- **Utilities**: indicatif (progress bars), rayon (parallelism)

## 6. Active Development

See `TODO.md` for the modernization plan, which includes:
- Migration from OpenSSL to rustls
- Integration of structured logging with tracing
- Enhanced error reporting
- Performance optimizations

When implementing new features, ensure compatibility with the async architecture and maintain the modular structure.

# Working principles for software development

## 7. When you write code (in any language)

- Iterate gradually, avoiding major changes 
- Minimize confirmations and checks
- Preserve existing code/structure unless necessary
- Use constants over magic numbers
- Check for existing solutions in the codebase before starting
- Check often the coherence of the code you’re writing with the rest of the code. 
- Focus on minimal viable increments and ship early
- Write explanatory docstrings/comments that explain what and WHY this does, explain where and how the code is used/referred to elsewhere in the code
- Analyze code line-by-line 
- Handle failures gracefully with retries, fallbacks, user guidance
- Address edge cases, validate assumptions, catch errors early
- Let the computer do the work, minimize user decisions 
- Reduce cognitive load, beautify code
- Modularize repeated logic into concise, single-purpose functions
- Favor flat over nested structures
- Consistently keep, document, update and consult the holistic overview mental image of the codebase:
  - README.md (purpose and functionality) 
  - CHANGELOG.md (past changes)
  - TODO.md (future goals)
  - PROGRESS.md (detailed flat task list)

## 8. Use MCP tools if you can

Before and during coding (if have access to tools), you should: 

- consult the `context7` tool for most up-to-date software package documentation;
- ask intelligent questions to the `deepseek/deepseek-r1-0528:free` model via the `chat_completion` tool to get additional reasoning;
- also consult the `openai/o3` model via the `chat_completion` tool for additional reasoning and help with the task;
- use the `sequentialthinking` tool to think about the best way to solve the task; 
- use the `perplexity_ask` and `duckduckgo_web_search` tools to gather up-to-date information or context;

## 9. Keep track of paths

In each source file, maintain the up-to-date `this_file` record that shows the path of the current file relative to project root. Place the `this_file` record near the top of the file, as a comment after the shebangs, or in the YAML Markdown frontmatter. 

## 10. If you write Python

- PEP 8: Use consistent formatting and naming
- Write clear, descriptive names for functions and variables
- PEP 20: Keep code simple and explicit. Prioritize readability over cleverness
- Use type hints in their simplest form (list, dict, | for unions)
- PEP 257: Write clear, imperative docstrings
- Use f-strings. Use structural pattern matching where appropriate
- ALWAYS add "verbose" mode logugu-based logging, & debug-log
- For CLI Python scripts, use fire & rich, and start the script with 

```
#!/usr/bin/env -S uv run -s
# /// script
# dependencies = ["PKG1", "PKG2"]
# ///
# this_file: PATH_TO_CURRENT_FILE
```

After Python changes run:

```
uzpy run .; fd -e py -x autoflake {}; fd -e py -x pyupgrade --py312-plus {}; fd -e py -x ruff check --output-format=github --fix --unsafe-fixes {}; fd -e py -x ruff format --respect-gitignore --target-version py312 {}; python -m pytest;
```

## 11. Additional guidelines

Ask before extending/refactoring existing code in a way that may add complexity or break things. 

When you’re finished, print "Wait, but" to go back, think & reflect, revise & improvement what you’ve done (but don’t invent functionality freely). Repeat this. But stick to the goal of "minimal viable next version". 

## 12. Virtual team work

Be creative, diligent, critical, relentless & funny! Lead two experts: "Ideot" for creative, unorthodox ideas, and "Critin" to critique flawed thinking and moderate for balanced discussions. The three of you shall illuminate knowledge with concise, beautiful responses, process methodically for clear answers, collaborate step-by-step, sharing thoughts and adapting. If errors are found, step back and focus on accuracy and progress.

## 13. Development Guidelines

- Only modify code directly relevant to the specific request. Avoid changing unrelated functionality.
- Never replace code with placeholders like `# ... rest of the processing ...`. Always include complete code.
- Break problems into smaller steps. Think through each step separately before implementing.
- Always provide a complete PLAN with REASONING based on evidence from code and logs before making changes.
- Explain your OBSERVATIONS clearly, then provide REASONING to identify the exact issue. Add console logs when needed to gather more information.


The URL-to-markdown conversion system consists of three primary business components:

### 13.1. URL Processing and Content Extraction (Importance: 95)
- Collects URLs from multiple input sources (stdin, files)
- Validates and resolves URLs against base URLs when needed
- Implements a robust retry mechanism for handling transient network failures
- Processes URLs concurrently with dynamic CPU core adaptation
- Located in `src/cli.rs` and `src/url.rs`

### 13.2. HTML Processing Pipeline (Importance: 90)
- Fetches HTML content using Monolith for specialized content extraction
- Implements fallback mechanisms for non-HTML content
- Handles custom HTML processing with configurable options
- Transforms complex HTML structures into clean markdown
- Located in `src/html.rs`

### 13.3. Content Organization and Output Management (Importance: 85)
- Creates structured output paths mirroring URL hierarchies
- Supports packing mode for combining multiple URLs into single documents
- Maintains original URL ordering in packed output
- Handles both remote URLs and local file paths consistently
- Located in `src/lib.rs` and `src/markdown.rs`

### 13.4. Integration Points

The system connects these components through:

1. URL Collection -> HTML Processing
- URLs are validated and normalized before processing
- Concurrent processing queue manages HTML fetching
- Retry logic handles failed attempts

2. HTML Processing -> Content Organization
- Processed HTML is transformed to markdown
- Output paths are generated based on URL structure
- Content is either packed or distributed based on mode

The build system embeds version and build metadata for traceability, ensuring each deployment can be traced to its source state.

To read the full codebase, run `npx repomix -o llms.txt .` and then read `llms.txt`

$END$



If you work with Python, use 'uv pip' instead of 'pip', and use 'uvx hatch test' instead of 'python -m pytest'. 

When I say /report, you must: Read all `./TODO.md` and `./PLAN.md` files and analyze recent changes. Document all changes in `./CHANGELOG.md`. From `./TODO.md` and `./PLAN.md` remove things that are done. Make sure that `./PLAN.md` contains a detailed, clear plan that discusses specifics, while `./TODO.md` is its flat simplified itemized `- [ ]`-prefixed representation. When I say /work, you must work in iterations like so: Read all `./TODO.md` and `./PLAN.md` files and reflect. Work on the tasks. Think, contemplate, research, reflect, refine, revise. Be careful, curious, vigilant, energetic. Verify your changes. Think aloud. Consult, research, reflect. Then update `./PLAN.md` and `./TODO.md` with tasks that will lead to improving the work you’ve just done. Then '/report', and then iterate again.