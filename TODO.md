---
this_file: TODO.md
---

# TODO List

> This is a flat task list corresponding to PLAN.md. Mark items with [x] as you complete them.

## Phase 1: Critical Cleanup (Priority: CRITICAL)

### 1.1 Remove Build Artifacts from Git ✅ COMPLETED
- [x] Run `git rm -r --cached target/` (not needed - already not tracked)
- [x] Run `git rm -r --cached work/` (not needed - already not tracked)
- [x] Run `git rm --cached '**/.DS_Store'` (not needed - already not tracked)
- [x] Add `target/` to .gitignore (already present)
- [x] Add `builds/` to .gitignore (already present)
- [x] Add `work/` to .gitignore (already present)
- [x] Add `.DS_Store` to .gitignore (already present)
- [x] Commit changes with message "Remove build artifacts and development workspace from tracking" (done in earlier commits)
- [x] Verify repository size decreased (verified - no artifacts tracked)
- [x] Test that builds still work correctly (all tests pass)

### 1.2 Consolidate Build System ✅ COMPLETED
- [x] Remove root `/build.sh` (deleted - confirmed not present)
- [x] Update `.github/workflows/ci.yml` to use new build script (CI uses direct cargo commands which is cleaner for CI/CD; scripts/build.sh available for local development)

### 1.3 Fix Monolith Debug Output ✅ DONE (Commit #501)
- [ ] Open issue on `github.com/Y2Z/monolith` about debug output (optional - already fixed)

## Phase 2: Documentation Consolidation (Priority: HIGH)

### 2.3 Update README.md ✅ PARTIALLY COMPLETED
- [x] Verify installation script is current (install.sh exists and correctly referenced)
- [ ] Test all example commands work as shown (requires binary build - in progress)
- [x] Verify all documentation links are valid (checked and confirmed)
- [x] Verify badge URLs point to correct resources (all badges verified correct)
- [x] Verify version numbers are current (1.4.3 matches Cargo.toml)
- [x] Verify feature list matches actual capabilities (reviewed against implementation)
- [x] Verify troubleshooting section is accurate (section reviewed and current)

## Phase 3: Optimization (Priority: MEDIUM)

### 3.1 Optimize Build Scripts ✅ COMPLETED
- [x] Implement `scripts/build.sh` with `--quick` mode (skip lint/test) - Done
- [x] Implement `scripts/build.sh` with `--dev` mode (full checks) - Done (default mode)
- [x] Implement `scripts/build.sh` with `--ci` mode (clean build) - Done
- [x] Implement `scripts/build.sh` with `--release` mode (optimized, stripped) - Done

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
