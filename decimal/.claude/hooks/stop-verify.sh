#!/bin/bash
set -e

# Claude Code Stop hook: block the agent from finishing its loop until verify.sh passes.
# Exit codes (Claude Code Stop hook contract):
#   0 -> verification passed, agent may stop
#   2 -> blocking: agent receives stderr as feedback and must continue fixing
#   1 -> non-blocking error shown to user (used when giving up after MAX_ATTEMPTS)

MAX_ATTEMPTS=3
MAX_OUTPUT_LINES=150
COUNTER_FILE="${TMPDIR:-/tmp}/claude-stop-verify-attempts-rust-decimal"
GREEN_FILE="${TMPDIR:-/tmp}/claude-stop-verify-green-rust-decimal"

# CLAUDE_PROJECT_DIR is normally set by Claude Code to this crate's dir. The
# git-toplevel fallback resolves to the parent "rust" repo root (this crate has
# no .git of its own), so fall back to the script's own location instead.
cd "${CLAUDE_PROJECT_DIR:-$(cd "$(dirname "$0")/../.." && pwd)}" || exit 1

# Fingerprint everything verify.sh depends on: source tree (tracked + untracked,
# content-hashed), manifests, and the script itself. Identical fingerprint to
# the last green run -> identical verdict -> safe to skip.
# The fingerprint omits Cargo.lock so a dependency bump wouldn't invalidate the cache. 
# Thats fine for an exercism crate, you may adapt this to your own needs.
compute_fingerprint() {
  {
    rustc --version
    {
      git ls-files -coz --exclude-standard src tests
      printf '%s\0' Cargo.toml verify.sh
    } | xargs -0 shasum -a 256
  } | shasum -a 256 | cut -d' ' -f1
}

# An empty fingerprint (computation failed) never matches -> full verification runs.
inputs_hash=$(compute_fingerprint 2>/dev/null || true)

if [ -n "$inputs_hash" ] && [ "$(cat "$GREEN_FILE" 2>/dev/null)" = "$inputs_hash" ]; then
  rm -f "$COUNTER_FILE"
  exit 0
fi

# "|| status=$?" keeps set -e from aborting here; failure must reach the exit-2 logic below
status=0
output=$(bash verify.sh 2>&1) || status=$?

if [ "$status" -eq 0 ]; then
  rm -f "$COUNTER_FILE"
  # Recompute instead of reusing inputs_hash: cargo may have touched tracked files.
  compute_fingerprint > "$GREEN_FILE" 2>/dev/null || rm -f "$GREEN_FILE"
  exit 0
fi

attempts=$(($(cat "$COUNTER_FILE" 2>/dev/null || echo 0) + 1))
echo "$attempts" > "$COUNTER_FILE"

if [ "$attempts" -ge "$MAX_ATTEMPTS" ]; then
  # Give up blocking to avoid an infinite loop; surface failure to the user instead.
  rm -f "$COUNTER_FILE"
  echo "verify.sh still failing after $MAX_ATTEMPTS consecutive attempts; giving up. Last output:" >&2
  echo "$output" | tail -n "$MAX_OUTPUT_LINES" >&2
  exit 1
fi

echo "verify.sh failed (attempt $attempts/$MAX_ATTEMPTS). Fix the issues below before stopping:" >&2
echo "$output" | tail -n "$MAX_OUTPUT_LINES" >&2
exit 2
