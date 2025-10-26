#!/bin/bash
# this_file: tests/scripts_build_help_test.sh

set -euo pipefail

# Smoke test to ensure scripts/build.sh --help exits early without invoking cargo.
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TMP_BIN="$(mktemp -d)"
trap 'rm -rf "$TMP_BIN"' EXIT

# Provide a stub cargo binary so this test never triggers a real build.
cat <<'EOF' > "${TMP_BIN}/cargo"
#!/bin/bash
exit 0
EOF
chmod +x "${TMP_BIN}/cargo"
export PATH="${TMP_BIN}:${PATH}"

OUTPUT="$(TWARS_BUILD_SKIP_CARGO=1 bash "${REPO_ROOT}/scripts/build.sh" --help 2>&1 || true)"
if [[ "${OUTPUT}" != *"Usage: ./scripts/build.sh [MODE]"* ]]; then
    echo "Expected help text, got:" >&2
    echo "${OUTPUT}" >&2
    exit 1
fi
