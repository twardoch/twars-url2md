// this_file: build.rs
//
// Cargo Build Script for twars-url2md
//
// PURPOSE:
// This build script runs before compilation and performs two critical tasks:
//
// 1. VERSION EXTRACTION FROM GIT
//    Automatically determines the package version from git tags, enabling
//    semantic versioning without manual Cargo.toml updates.
//
// 2. BUILD METADATA GENERATION
//    Captures build-time information (timestamp, rustc version, etc.) using
//    the `built` crate, making it available to the binary via built.rs.
//
// GIT VERSION LOGIC:
//
// The script tries these strategies in order:
//
// 1. Exact Tag Match: `git describe --tags --exact-match`
//    - If HEAD is exactly at a tag (e.g., "v1.4.3")
//    - Returns: "1.4.3" (strips 'v' prefix)
//    - Example: When you run `git tag v1.5.0 && git checkout v1.5.0`
//
// 2. Development Version: `git describe --tags --always --dirty`
//    - If HEAD is past a tag (commits since last tag)
//    - Parses format: "v1.4.3-5-g1234567" or "v1.4.3-dirty"
//    - Returns: "1.4.3-dev.5.g1234567" or "1.4.3-dev.0.g1234567-dirty"
//    - Example outputs:
//      * "1.4.3-dev.2.gabcd123" = 2 commits past v1.4.3
//      * "1.4.3-dev.0.gabcd123-dirty" = at v1.4.3 but with uncommitted changes
//
// 3. Fallback: Returns "0.0.0-dev"
//    - Used when git is unavailable or no tags exist
//    - Ensures builds always succeed even without git history
//
// REBUILD TRIGGERS:
// The build script re-runs when these files change:
// - build.rs itself (this file)
// - .git/HEAD (branch switches, commits)
// - .git/refs/ (tag or branch updates)
//
// USAGE IN CODE:
// The version is available via the CARGO_PKG_VERSION environment variable:
//   const VERSION: &str = env!("CARGO_PKG_VERSION");
//
// Build metadata is available via the generated built.rs module:
//   use crate::built;
//   println!("Built at: {}", built::BUILT_TIME_UTC);
//

use std::process::Command;

fn main() {
    // Get version from git tags
    if let Ok(version) = get_version_from_git() {
        println!("cargo:rustc-env=CARGO_PKG_VERSION={}", version);
    }

    // Write build-time information
    if let Err(e) = built::write_built_file() {
        eprintln!("Warning: Failed to acquire build-time information: {}", e);
        // Don't fail the build, just warn
    }

    // Rebuild triggers
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs/");
}

fn get_version_from_git() -> Result<String, Box<dyn std::error::Error>> {
    // Try to get version from git describe
    let output = Command::new("git")
        .args(["describe", "--tags", "--exact-match"])
        .output();

    if let Ok(output) = output {
        if output.status.success() {
            let tag = String::from_utf8(output.stdout)?.trim().to_string();
            // Remove 'v' prefix if present
            let version = if let Some(stripped) = tag.strip_prefix('v') {
                stripped
            } else {
                &tag
            };
            return Ok(version.to_string());
        }
    }

    // Fallback: try to get latest tag + commit count
    let output = Command::new("git")
        .args(["describe", "--tags", "--always", "--dirty"])
        .output();

    if let Ok(output) = output {
        if output.status.success() {
            let describe = String::from_utf8(output.stdout)?.trim().to_string();
            // Parse git describe output (e.g., "v1.2.3-5-g1234567" or "v1.2.3")
            if let Some(version) = parse_git_describe(&describe) {
                return Ok(version);
            }
        }
    }

    // Final fallback: use default version
    Ok("0.0.0-dev".to_string())
}

fn parse_git_describe(describe: &str) -> Option<String> {
    // Remove 'v' prefix if present
    let describe = describe.strip_prefix('v').unwrap_or(describe);

    // Handle dirty suffix
    let (describe, dirty) = if let Some(stripped) = describe.strip_suffix("-dirty") {
        (stripped, true)
    } else {
        (describe, false)
    };

    // Parse format: "1.2.3-5-g1234567" or "1.2.3"
    let parts: Vec<&str> = describe.split('-').collect();

    let version = match parts.len() {
        1 => {
            // Exact tag match (e.g., "1.2.3")
            parts[0].to_string()
        }
        3 if parts[2].starts_with('g') => {
            // Tag + commits + hash (e.g., "1.2.3-5-g1234567")
            let base_version = parts[0];
            let commit_count = parts[1];
            let hash = parts[2];
            format!("{}-dev.{}.{}", base_version, commit_count, hash)
        }
        _ => {
            // Fallback for unexpected formats
            describe.to_string()
        }
    };

    let final_version = if dirty {
        format!("{}-dirty", version)
    } else {
        version
    };

    Some(final_version)
}
