# TODO List

> This is a flat task list corresponding to PLAN.md. Mark items with [x] as you complete them.

## Phase 1: Critical Cleanup (Priority: CRITICAL)

### 1.1 Remove Build Artifacts from Git
- [ ] Run `git rm -r --cached target/`
- [ ] Run `git rm -r --cached work/`
- [ ] Run `git rm --cached '**/.DS_Store'`
- [ ] Add `target/` to .gitignore
- [ ] Add `builds/` to .gitignore
- [ ] Add `work/` to .gitignore
- [ ] Add `.DS_Store` to .gitignore
- [ ] Commit changes with message "Remove build artifacts and development workspace from tracking"
- [ ] Verify repository size decreased
- [ ] Test that builds still work correctly

### 1.2 Consolidate Build System
- [ ] Create `/scripts/lint.sh` for code quality checks
- [ ] Create `/scripts/generate-llms.sh` for repomix
- [ ] Update `/scripts/build.sh` to add `--quick` mode
- [ ] Update `/scripts/build.sh` to add `--dev` mode
- [ ] Update `/scripts/build.sh` to add `--ci` mode
- [ ] Update `/scripts/build.sh` to add `--release` mode
- [ ] Test all build modes work correctly
- [ ] Remove root `/build.sh`
- [ ] Update `DEVELOPMENT.md` with new build commands
- [ ] Update `CONTRIBUTING.md` with new workflow
- [ ] Update `README.md` build instructions
- [ ] Update `.github/workflows/ci.yml` to use new build script

### 1.3 Fix Monolith Debug Output
- [ ] Open issue on `github.com/Y2Z/monolith` about debug output
- [ ] Research monolith source code to find debug print statements
- [ ] Implement Option A: Suppress stdout during initialization (temporary workaround)
- [ ] Test that debug output is suppressed
- [ ] Monitor upstream issue for response
- [ ] If no response in 2 weeks, fork monolith and patch
- [ ] If forking, submit PR upstream with fix

### 1.4 Clean Up Stray Files
- [ ] Decide: Move `test_http.rs` to `tests/unit/` or delete
- [ ] Run `git rm .pre-commit-config.yaml.bak`
- [ ] Run `git rm llms.txt`
- [ ] Run `git rm md.txt`
- [ ] Add `llms.txt` to .gitignore
- [ ] Add `*.bak` to .gitignore
- [ ] Commit changes with message "Remove stray development files"

## Phase 2: Documentation Consolidation (Priority: HIGH)

### 2.1 Deduplicate LLM Configuration Files
- [ ] Keep `CLAUDE.md` as canonical file
- [ ] Run `git rm AGENTS.md`
- [ ] Run `git rm GEMINI.md`
- [ ] Run `git rm LLXPRT.md`
- [ ] Run `git rm QWEN.md`
- [ ] Search for references: `grep -r "AGENTS.md\|GEMINI.md\|LLXPRT.md\|QWEN.md" . --exclude-dir=.git`
- [ ] Update any references found
- [ ] Commit changes with message "Deduplicate LLM configuration files"

### 2.2 Complete Project Documentation Set
- [ ] Verify `PLAN.md` is complete (this file)
- [ ] Verify `TODO.md` is complete (this file)
- [ ] Create `WORK.md` for progress tracking
- [ ] Add header explaining purpose of each planning file
- [ ] Document workflow: PLAN.md (strategic), TODO.md (tactical), WORK.md (daily)

### 2.3 Update README.md
- [ ] Verify build instructions match new build system
- [ ] Verify installation script is current
- [ ] Test all example commands work as shown
- [ ] Verify all documentation links are valid
- [ ] Verify badge URLs point to correct resources
- [ ] Verify version numbers are current
- [ ] Verify feature list matches actual capabilities
- [ ] Verify troubleshooting section is accurate
- [ ] Update build section to reference `scripts/build.sh`
- [ ] Update any references to monolith if fixed

## Phase 3: Optimization (Priority: MEDIUM)

### 3.1 Optimize Build Scripts
- [ ] Implement `scripts/build.sh` with `--quick` mode (skip lint/test)
- [ ] Implement `scripts/build.sh` with `--dev` mode (full checks)
- [ ] Implement `scripts/build.sh` with `--ci` mode (clean build)
- [ ] Implement `scripts/build.sh` with `--release` mode (optimized, stripped)
- [ ] Create `scripts/generate-llms.sh` with npx check
- [ ] Create `scripts/lint.sh` with cargo fmt and clippy
- [ ] Test all new scripts work correctly
- [ ] Update documentation to reference new scripts

### 3.2 Optimize install.sh
- [ ] Add checksum verification function
- [ ] Download and verify checksums from releases
- [ ] Add update notification function
- [ ] Check current version against latest
- [ ] Notify user if update available
- [ ] Test checksum verification on all platforms
- [ ] Test update notification works correctly

## Phase 4: Structural Improvements (Priority: LOW)

### 4.1 Extract Development Workspace
- [ ] Backup `work/html-to-text-comparison/` directory
- [ ] Create new repository for html-to-text-comparison
- [ ] Initialize git in workspace: `cd work/html-to-text-comparison && git init`
- [ ] Commit all files in workspace
- [ ] Create remote repository on GitHub
- [ ] Push workspace to new repository
- [ ] Run `git rm -r work/html-to-text-comparison` in main repo
- [ ] Add link to new repo in main README.md
- [ ] Commit removal with message "Extract html-to-text-comparison to separate repository"

### 4.2 Configure IDE/Tool-Specific Directories
- [ ] Decide: Keep `.cursor/rules/` or remove
- [ ] Add `.giga/` to .gitignore
- [ ] Add `.specstory/` to .gitignore
- [ ] Run `git rm -r --cached .giga/`
- [ ] Run `git rm -r --cached .specstory/`
- [ ] Commit changes with message "Remove personal development tool directories"

### 4.3 Verify Test Suite Integrity
- [ ] Run `cargo test --all-features --verbose` and save output
- [ ] Review test output for failures or skipped tests
- [ ] Check `tests/integration/e2e_tests.rs` for mockito issues
- [ ] Fix or document why integration tests are disabled
- [ ] Update mockito dependency if needed
- [ ] Review modified test fixtures in `tests/fixtures/expected/`
- [ ] Regenerate or commit updated fixtures
- [ ] Document fixture generation process in `TESTS.md`
- [ ] Install cargo-tarpaulin for coverage reporting
- [ ] Run coverage analysis and review results
- [ ] Verify coverage is > 70%

## Phase 5: Build System Ambiguity Resolution

### 5.1 Clarify Build Script Roles
- [ ] Document `/build.rs` purpose in comments (Cargo build script)
- [ ] Verify `/build.rs` works correctly with cargo
- [ ] Finalize `/scripts/build.sh` as primary build script
- [ ] Test all modes: `--quick`, `--dev`, `--ci`, `--release`
- [ ] Remove root `/build.sh` after functionality migrated
- [ ] Update `DEVELOPMENT.md` with build system architecture
- [ ] Document when to use each build mode
- [ ] Update `.github/workflows/ci.yml` to use new scripts
- [ ] Test CI workflow with new scripts

## Phase 6: Validation and Documentation

### 6.1 Validation Checklist
- [ ] Test `scripts/build.sh --quick` works
- [ ] Test `scripts/build.sh --dev` works (with linting/testing)
- [ ] Test `scripts/build.sh --ci` works (clean build)
- [ ] Test `scripts/build.sh --release` works (optimized)
- [ ] Test `cargo build` still works directly
- [ ] Test `cargo test` passes all tests
- [ ] Verify binary size is reasonable (< 10MB)
- [ ] Test `install.sh` downloads and installs correctly
- [ ] Test installed binary: `twars-url2md --help`
- [ ] Verify installed binary shows correct version
- [ ] Verify no debug output from monolith
- [ ] Verify `.gitignore` is comprehensive
- [ ] Verify no build artifacts in git
- [ ] Verify no `.DS_Store` files in git
- [ ] Verify repository size is < 50MB
- [ ] Verify all documentation is up-to-date
- [ ] Test all command-line options work
- [ ] Test URL processing works correctly
- [ ] Test Markdown output is clean
- [ ] Test progress bar displays properly
- [ ] Test error handling is graceful
- [ ] Verify `README.md` is accurate
- [ ] Verify `DEVELOPMENT.md` reflects new build system
- [ ] Verify `CONTRIBUTING.md` has correct workflow
- [ ] Update `CHANGELOG.md` with all changes
- [ ] Verify GitHub Actions workflows still work
- [ ] Verify multi-platform builds succeed
- [ ] Verify release process functions correctly

## Continuous Tasks
- [ ] Update `WORK.md` with daily progress
- [ ] Mark completed tasks with [x] in this file
- [ ] Remove completed items from `WORK.md` when done
- [ ] Update `CHANGELOG.md` as significant changes are made
- [ ] Test after each major change to catch issues early
- [ ] Keep documentation in sync with code changes
