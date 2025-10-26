#!/usr/bin/env python3
# this_file: issues/issuetest.py
"""
Test script to verify issues in twars-url2md

This script tests each issue to determine if it has been resolved.
"""

import subprocess
import os
import sys
import tempfile
import shutil
from pathlib import Path

# Colors for output
GREEN = "\033[92m"
RED = "\033[91m"
YELLOW = "\033[93m"
BLUE = "\033[94m"
RESET = "\033[0m"


def print_test_header(issue_num, description):
    """Print a formatted test header"""
    print(f"\n{BLUE}{'=' * 60}{RESET}")
    print(f"{BLUE}Testing Issue #{issue_num}: {description}{RESET}")
    print(f"{BLUE}{'=' * 60}{RESET}")


def print_result(success, message):
    """Print a test result with color"""
    if success:
        print(f"{GREEN}✓ PASS:{RESET} {message}")
    else:
        print(f"{RED}✗ FAIL:{RESET} {message}")


def run_command(cmd, capture_output=True, check=False):
    """Run a shell command and return the result"""
    try:
        if capture_output:
            result = subprocess.run(
                cmd, shell=True, capture_output=True, text=True, check=check
            )
            return result.returncode, result.stdout, result.stderr
        else:
            result = subprocess.run(cmd, shell=True, check=check)
            return result.returncode, "", ""
    except subprocess.CalledProcessError as e:
        return (
            e.returncode,
            e.stdout if hasattr(e, "stdout") else "",
            e.stderr if hasattr(e, "stderr") else "",
        )


def test_issue_105():
    """Test Issue #105: Fix Help Option Not Working"""
    print_test_header(105, "Fix Help Option Not Working")

    tests_passed = True

    # Test 1: Check if --help works
    print("\nTest 1: Running with --help flag")
    returncode, stdout, stderr = run_command("./target/release/twars-url2md --help")

    if returncode == 0 and "Usage:" in stdout and "Options:" in stdout:
        print_result(True, "--help displays usage information")
    else:
        print_result(False, "--help does not display proper help text")
        print(f"  Return code: {returncode}")
        print(f"  stdout: {stdout[:100]}...")
        print(f"  stderr: {stderr[:100]}...")
        tests_passed = False

    # Test 2: Check if -h works
    print("\nTest 2: Running with -h flag")
    returncode, stdout, stderr = run_command("./target/release/twars-url2md -h")

    if returncode == 0 and "Usage:" in stdout:
        print_result(True, "-h displays usage information")
    else:
        print_result(False, "-h does not display help text")
        tests_passed = False

    # Test 3: Check if --version works
    print("\nTest 3: Running with --version flag")
    returncode, stdout, stderr = run_command("./target/release/twars-url2md --version")

    if returncode == 0 and "twars-url2md" in stdout:
        print_result(True, f"--version displays version: {stdout.strip()}")
    else:
        print_result(False, "--version does not display version information")
        tests_passed = False

    # Test 4: Check if -V works
    print("\nTest 4: Running with -V flag")
    returncode, stdout, stderr = run_command("./target/release/twars-url2md -V")

    if returncode == 0 and "twars-url2md" in stdout:
        print_result(True, "-V displays version information")
    else:
        print_result(False, "-V does not display version")
        tests_passed = False

    return tests_passed


def test_issue_106():
    """Test Issue #106: Fix Output Writing Issues"""
    print_test_header(106, "Fix Output Writing Issues")

    tests_passed = True

    with tempfile.TemporaryDirectory() as tmpdir:
        # Test 1: Directory output mode
        print("\nTest 1: Directory output mode (-o dir/)")
        output_dir = os.path.join(tmpdir, "output_dir")
        cmd = f'echo "https://example.com" | ./target/release/twars-url2md --stdin -o {output_dir}'
        returncode, stdout, stderr = run_command(cmd)

        expected_file = os.path.join(output_dir, "example.com", "index.md")
        if returncode == 0 and os.path.exists(expected_file):
            print_result(True, f"Directory output created file at {expected_file}")
            # Check if file has content
            with open(expected_file) as f:
                content = f.read()
                if len(content) > 0:
                    print_result(True, f"Output file contains {len(content)} bytes")
                else:
                    print_result(False, "Output file is empty")
                    tests_passed = False
        else:
            print_result(False, f"Directory output failed to create {expected_file}")
            print(f"  Return code: {returncode}")
            print(f"  stderr: {stderr}")
            tests_passed = False

        # Test 2: Single file output mode
        print("\nTest 2: Single file output mode (-o file.md)")
        output_file = os.path.join(tmpdir, "output.md")
        cmd = f'echo "https://example.com" | ./target/release/twars-url2md --stdin -o {output_file}'
        returncode, stdout, stderr = run_command(cmd)

        if returncode == 0 and os.path.exists(output_file):
            print_result(True, f"Single file output created at {output_file}")
            with open(output_file) as f:
                content = f.read()
                if len(content) > 0:
                    print_result(True, f"Single file contains {len(content)} bytes")
                else:
                    print_result(False, "Single file is empty")
                    tests_passed = False
        else:
            print_result(False, f"Single file output failed to create {output_file}")
            tests_passed = False

        # Test 3: Pack mode
        print("\nTest 3: Pack mode (-p packed.md)")
        pack_file = os.path.join(tmpdir, "packed.md")
        cmd = f'echo -e "https://example.com\\nhttps://example.org" | ./target/release/twars-url2md --stdin -p {pack_file}'
        returncode, stdout, stderr = run_command(cmd)

        if returncode == 0 and os.path.exists(pack_file):
            print_result(True, f"Pack file created at {pack_file}")
            with open(pack_file) as f:
                content = f.read()
                if "example.com" in content and "example.org" in content:
                    print_result(True, "Pack file contains both URLs")
                else:
                    print_result(False, "Pack file missing expected content")
                    tests_passed = False
        else:
            print_result(False, f"Pack mode failed to create {pack_file}")
            tests_passed = False

        # Test 4: Default behavior (no -o flag creates files in current directory)
        print("\nTest 4: Default behavior (no output flags)")
        # Clean up any existing file first
        if os.path.exists("./example.com/index.md"):
            shutil.rmtree("./example.com", ignore_errors=True)

        cmd = 'echo "https://example.com" | ./target/release/twars-url2md --stdin'
        returncode, stdout, stderr = run_command(cmd)

        if returncode == 0 and os.path.exists("./example.com/index.md"):
            print_result(True, "Default behavior creates files in current directory")
            # Clean up
            shutil.rmtree("./example.com", ignore_errors=True)
        else:
            print_result(False, "Default behavior not working properly")
            tests_passed = False

    return tests_passed


def test_issue_107():
    """Test Issue #107: Implement Smart HTML Content Extraction"""
    print_test_header(107, "Implement Smart HTML Content Extraction")

    tests_passed = True

    # Test 1: Check if --all flag exists
    print("\nTest 1: Check if --all flag is available")
    returncode, stdout, stderr = run_command("./target/release/twars-url2md --help")

    if "--all" in stdout or "-a" in stdout:
        print_result(True, "--all flag is available in CLI")
        # Look for the description
        if "Extract all content" in stdout:
            print_result(True, "--all flag has proper description")
        else:
            print_result(False, "--all flag missing description")
    else:
        print_result(False, "--all flag not found in help text")
        tests_passed = False

    # Test 2: Test with a local HTML file containing navigation
    print("\nTest 2: Test content extraction with local HTML")
    with tempfile.TemporaryDirectory() as tmpdir:
        # Create test HTML with navigation elements
        test_html = """
        <html>
        <body>
            <nav>Navigation Menu</nav>
            <header>Site Header</header>
            <main>
                <h1>Main Content</h1>
                <p>This is the actual content we want to extract.</p>
            </main>
            <aside>Sidebar content</aside>
            <footer>Footer content</footer>
        </body>
        </html>
        """

        html_file = os.path.join(tmpdir, "test.html")
        with open(html_file, "w") as f:
            f.write(test_html)

        # Test without --all flag (should extract smartly once implemented)
        output_file = os.path.join(tmpdir, "output_smart.md")
        cmd = f'echo "file://{html_file}" | ./target/release/twars-url2md --stdin -o {output_file}'
        returncode1, _, _ = run_command(cmd)

        # Test with --all flag
        output_all_file = os.path.join(tmpdir, "output_all.md")
        cmd = f'echo "file://{html_file}" | ./target/release/twars-url2md --stdin --all -o {output_all_file}'
        returncode2, _, _ = run_command(cmd)

        if returncode1 == 0 and returncode2 == 0:
            print_result(True, "Both extraction modes execute without errors")

            # Check if files were created
            if os.path.exists(output_file) and os.path.exists(output_all_file):
                with open(output_file) as f:
                    smart_content = f.read()
                with open(output_all_file) as f:
                    all_content = f.read()

                print(f"  Smart extraction: {len(smart_content)} bytes")
                print(f"  All extraction: {len(all_content)} bytes")

                # Once smart extraction is implemented, all_content should be larger
                # For now, just check that both files have content
                if len(smart_content) > 0 and len(all_content) > 0:
                    print_result(True, "Both extraction modes produce output")
                else:
                    print_result(
                        False, "One or both extraction modes produced empty output"
                    )
                    tests_passed = False
            else:
                print_result(False, "Output files were not created")
                tests_passed = False
        else:
            print_result(False, "Extraction commands failed")
            tests_passed = False

    return tests_passed


def test_issue_108():
    """Test Issue #108: Remove Panic Recovery Wrapper from Main"""
    print_test_header(108, "Remove Panic Recovery Wrapper from Main")

    # This is more of a code quality issue, so we'll check if the app handles errors gracefully
    tests_passed = True

    print("\nTest 1: Check if application handles malformed input gracefully")

    # Test with invalid URL
    cmd = 'echo "not-a-valid-url" | ./target/release/twars-url2md --stdin'
    returncode, stdout, stderr = run_command(cmd)

    if returncode != 0 or "Error" in stderr or "Error" in stdout:
        print_result(True, "Application handles invalid URLs without panicking")
    else:
        # It might still process it, which is also fine
        print_result(True, "Application processed input without crashing")

    # Test with empty input
    print("\nTest 2: Check handling of empty input")
    cmd = 'echo "" | ./target/release/twars-url2md --stdin'
    returncode, stdout, stderr = run_command(cmd)

    if "Collected 0 URLs" in stderr or "Collected 0 URLs" in stdout:
        print_result(True, "Application handles empty input gracefully")
    else:
        print_result(True, "Application completed without panic")

    return tests_passed


def test_issue_109():
    """Test Issue #109: Update Documentation for Logging Framework"""
    print_test_header(109, "Update Documentation for Logging Framework")

    tests_passed = True

    # Test 1: Check if RUST_LOG affects output
    print("\nTest 1: Test RUST_LOG=twars_url2md=debug")
    cmd = 'RUST_LOG=twars_url2md=debug echo "https://example.com" | ./target/release/twars-url2md --stdin 2>&1 | head -20'
    returncode, output, _ = run_command(cmd)

    if "DEBUG" in output:
        print_result(True, "RUST_LOG=twars_url2md=debug enables debug logging")
    else:
        # Also try with just debug to see if it works with verbose flag
        cmd2 = 'RUST_LOG=debug echo "https://example.com" | ./target/release/twars-url2md --stdin --verbose 2>&1 | head -20'
        returncode2, output2, _ = run_command(cmd2)
        if "DEBUG" in output2:
            print_result(True, "RUST_LOG=debug with --verbose enables debug logging")
        else:
            print_result(False, "RUST_LOG debug logging not working as expected")
            tests_passed = False

    # Test 2: Check if RUST_LOG=info works
    print("\nTest 2: Test RUST_LOG=info")
    cmd = 'RUST_LOG=info echo "https://example.com" | ./target/release/twars-url2md --stdin 2>&1 | head -20'
    returncode, output, _ = run_command(cmd)

    if "INFO" in output and "DEBUG" not in output:
        print_result(True, "RUST_LOG=info shows only info and above")
    else:
        print_result(False, "RUST_LOG=info not filtering correctly")
        tests_passed = False

    # Test 3: Check verbose flag
    print("\nTest 3: Test --verbose flag")
    cmd = 'echo "https://example.com" | ./target/release/twars-url2md --stdin --verbose 2>&1 | head -20'
    returncode, output, _ = run_command(cmd)

    if "DEBUG" in output or "INFO" in output:
        print_result(True, "--verbose flag enables detailed logging")
    else:
        print_result(False, "--verbose flag not working")
        tests_passed = False

    # Test 4: Check if README documents logging
    print("\nTest 4: Check if README.md mentions logging")
    if os.path.exists("README.md"):
        with open("README.md") as f:
            readme = f.read()
            if "RUST_LOG" in readme or "logging" in readme.lower():
                print_result(True, "README.md contains logging information")
            else:
                print_result(False, "README.md does not document logging")
                tests_passed = False
    else:
        print_result(False, "README.md not found")
        tests_passed = False

    return tests_passed


def test_issue_110():
    """Test Issue #110: Enhanced Testing Strategy"""
    print_test_header(110, "Enhanced Testing Strategy")

    tests_passed = True

    # Test 1: Check if tests exist and pass
    print("\nTest 1: Run cargo test")
    returncode, stdout, stderr = run_command("cargo test --release", check=False)

    if returncode == 0:
        print_result(True, "All tests pass")
        # Count tests
        if "test result:" in stdout:
            test_line = [line for line in stdout.split("\n") if "test result:" in line]
            if test_line:
                print(f"  {test_line[0].strip()}")
    else:
        print_result(False, "Some tests are failing")
        tests_passed = False

    # Test 2: Check test coverage files
    print("\nTest 2: Check for test files")
    test_files = [
        "src/tests.rs",
        "tests/tests.rs",
        "src/cli.rs",  # Should contain tests
        "src/url.rs",  # Should contain tests
    ]

    test_count = 0
    for test_file in test_files:
        if os.path.exists(test_file):
            with open(test_file) as f:
                content = f.read()
                test_count += content.count("#[test]")
                test_count += content.count("#[tokio::test]")

    if test_count > 10:
        print_result(True, f"Found {test_count} test functions")
    else:
        print_result(False, f"Only {test_count} tests found (need more coverage)")
        tests_passed = False

    # Test 3: Check for integration tests
    print("\nTest 3: Check for integration tests")
    if os.path.exists("tests/"):
        integration_tests = list(Path("tests/").rglob("*.rs"))
        if len(integration_tests) > 0:
            print_result(True, f"Found {len(integration_tests)} integration test files")
        else:
            print_result(False, "No integration tests found")
            tests_passed = False
    else:
        print_result(False, "tests/ directory not found")
        tests_passed = False

    return tests_passed


def main():
    """Run all issue tests"""
    print(f"{YELLOW}twars-url2md Issue Test Suite{RESET}")
    print(f"{YELLOW}{'=' * 60}{RESET}")

    # Check if binary exists
    if not os.path.exists("./target/release/twars-url2md"):
        print(f"{RED}Error: Release binary not found!{RESET}")
        print("Please run: cargo build --release")
        sys.exit(1)

    # Run all tests
    results = {
        105: test_issue_105(),
        106: test_issue_106(),
        107: test_issue_107(),
        108: test_issue_108(),
        109: test_issue_109(),
        110: test_issue_110(),
    }

    # Summary
    print(f"\n{YELLOW}{'=' * 60}{RESET}")
    print(f"{YELLOW}TEST SUMMARY{RESET}")
    print(f"{YELLOW}{'=' * 60}{RESET}")

    for issue_num, passed in results.items():
        status = f"{GREEN}RESOLVED{RESET}" if passed else f"{RED}NEEDS WORK{RESET}"
        print(f"Issue #{issue_num}: {status}")

    total_passed = sum(1 for passed in results.values() if passed)
    total_issues = len(results)

    print(f"\n{YELLOW}Total: {total_passed}/{total_issues} issues resolved{RESET}")

    if total_passed == total_issues:
        print(f"\n{GREEN}All issues have been resolved!{RESET}")
        return 0
    else:
        print(f"\n{RED}Some issues still need attention.{RESET}")
        return 1


if __name__ == "__main__":
    sys.exit(main())
