use std::process::Command;

fn main() {
    // Get version from git tags
    if let Ok(version) = get_version_from_git() {
        println!("cargo:rustc-env=CARGO_PKG_VERSION={}", version);
    }

    built::write_built_file().expect("Failed to acquire build-time information");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=.git/refs/tags");
}

fn get_version_from_git() -> Result<String, Box<dyn std::error::Error>> {
    // Try to get version from git describe
    let output = Command::new("git")
        .args(&["describe", "--tags", "--exact-match"])
        .output();

    if let Ok(output) = output {
        if output.status.success() {
            let tag = String::from_utf8(output.stdout)?.trim().to_string();
            // Remove 'v' prefix if present
            let version = if tag.starts_with('v') {
                &tag[1..]
            } else {
                &tag
            };
            return Ok(version.to_string());
        }
    }

    // Fallback: try to get latest tag + commit count
    let output = Command::new("git")
        .args(&["describe", "--tags", "--always", "--dirty"])
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
    let describe = if describe.starts_with('v') {
        &describe[1..]
    } else {
        describe
    };

    // Handle dirty suffix
    let (describe, dirty) = if describe.ends_with("-dirty") {
        (&describe[..describe.len() - 6], true)
    } else {
        (describe, false)
    };

    // Parse format: "1.2.3-5-g1234567" or "1.2.3"
    let parts: Vec<&str> = describe.split('-').collect();

    match parts.len() {
        1 => {
            // Exact tag match
            let mut version = parts[0].to_string();
            if dirty {
                version.push_str("-dirty");
            }
            Some(version)
        }
        3 => {
            // Tag + commits + hash
            let base_version = parts[0];
            let commit_count = parts[1];
            let hash = parts[2];

            let mut version = format!("{}-dev.{}.{}", base_version, commit_count, hash);
            if dirty {
                version.push_str("-dirty");
            }
            Some(version)
        }
        _ => None,
    }
}
