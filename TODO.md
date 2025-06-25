# twars-url2md v1.0 MVP Streamlining Tasks

## 🎯 Focus: Create a lean, performant tool that excels at URL-to-Markdown conversion

## 🔥 High Priority - Core Cleanup

### Remove Non-Functional Features
- [x] Delete src/content_extractor.rs entirely
- [x] Remove `mod content_extractor` from lib.rs
- [x] Remove `extract_all` field from Config struct
- [x] Remove `--all` flag from CLI
- [x] Update all Config instantiations
- [x] Delete content_extractor tests

### Consolidate Duplicate Code
- [x] Create generic retry wrapper function
- [x] Replace process_url_with_retry with generic version
- [ ] Replace process_url_content_with_retry with generic version (kept due to complexity)
- [x] Merge process_url_async and process_url_with_content
- [x] Update all function callers

### Simplify Error Handling
- [x] Remove panic hook from main.rs
- [x] Simplify main() function
- [x] Remove catch_unwind references
- [ ] Simplify Monolith panic handling in html.rs (kept for safety)

## 🧹 Medium Priority - Code Quality

### Remove Dead Code
- [x] Delete fetch_html function in html.rs
- [x] Remove unused imports throughout
- [x] Delete commented-out test code
- [x] Remove unused test infrastructure

### Streamline Documentation
- [ ] Remove obvious inline comments
- [ ] Reduce verbose function documentation
- [ ] Simplify module-level docs
- [ ] Keep only "why" comments, remove "what" comments

### Reduce Logging
- [ ] Remove redundant debug logs
- [ ] Remove function entry/exit logging
- [ ] Consolidate similar log messages
- [ ] Keep only essential info/warn/error logs

## 📁 Low Priority - Final Cleanup

### Project Structure
- [x] Delete research/ folder
- [ ] Clean up test fixtures
- [ ] Remove obsolete issue files
- [ ] Update .gitignore

### Configuration Simplification
- [ ] Simplify progress bar setup
- [ ] Remove unnecessary build.rs complexity
- [ ] Streamline version information

### Documentation Updates
- [ ] Update README.md to reflect MVP scope
- [ ] Simplify CONTRIBUTING.md
- [ ] Update CHANGELOG.md
- [ ] Clean up code examples

## ✅ Success Criteria

- [ ] All existing tests pass
- [ ] Binary size reduced by 20%+
- [ ] ~2000 lines of code removed
- [ ] No functionality regression
- [ ] Faster build times

## 🚫 Out of Scope for v1.0

- Smart content extraction
- Advanced HTML processing options
- Plugin system
- Authentication features
- JavaScript rendering

---

**Last updated: 2025-06-25**