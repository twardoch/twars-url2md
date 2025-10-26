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
- [x] Create `/scripts/lint.sh` for code quality checks
- [x] Create `/scripts/generate-llms.sh` for repomix
- [x] Update `/scripts/build.sh` to add `--quick` mode
- [x] Update `/scripts/build.sh` to add `--dev` mode
- [x] Update `/scripts/build.sh` to add `--ci` mode
- [x] Update `/scripts/build.sh` to add `--release` mode
- [x] Test all build modes work correctly
- [ ] Remove root `/build.sh` (build.sh is in root, scripts/ has other utilities)
- [ ] Update `DEVELOPMENT.md` with new build commands
- [ ] Update `CONTRIBUTING.md` with new workflow
- [x] Update `README.md` build instructions
- [ ] Update `.github/workflows/ci.yml` to use new build script

### 1.3 Fix Monolith Debug Output ✅ DONE (Commit #501)
- [x] Move monolith to dev-dependencies
- [x] Test that debug output is eliminated
- [ ] Open issue on `github.com/Y2Z/monolith` about debug output (optional - already fixed)

### 1.4 Clean Up Stray Files ✅ DONE (Commit #501)
- [x] Removed test_http.rs, .pre-commit-config.yaml.bak, md.txt
- [x] Added `llms.txt` to .gitignore
- [x] Added `*.bak` to .gitignore
- [x] Enhanced .gitignore with builds/, llms.txt, *.bak patterns

## Phase 2: Documentation Consolidation (Priority: HIGH)

### 2.1 Deduplicate LLM Configuration Files ✅ DONE (Commit #501)
- [x] Kept `CLAUDE.md` as canonical file
- [x] Removed duplicate files (AGENTS.md, GEMINI.md, LLXPRT.md, QWEN.md)

### 2.2 Complete Project Documentation Set ✅ DONE
- [x] Verify `PLAN.md` is complete
- [x] Verify `TODO.md` is complete
- [x] Created `WORK.md` for progress tracking
- [x] Documented workflow in planning files

### 2.3 Update README.md
- [x] Update build instructions to reference new build.sh with modes
- [x] Update code quality section to reference scripts/lint.sh
- [ ] Verify installation script is current
- [ ] Test all example commands work as shown
- [ ] Verify all documentation links are valid
- [ ] Verify badge URLs point to correct resources
- [ ] Verify version numbers are current
- [ ] Verify feature list matches actual capabilities
- [ ] Verify troubleshooting section is accurate

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
