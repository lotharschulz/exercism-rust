#!/usr/bin/env bash
# Auto-inject a Criterion benchmark harness into an exercise if not present.
# Usage: ./../.github/scripts/inject_benchmark.sh <exercise_dir> <function_symbol>
# Example: .github/scripts/inject_benchmark.sh accumulate accumulate
#
# Requirements: criterion added to dev-dependencies by this script if absent.
set -euo pipefail

if [ $# -lt 2 ]; then
  echo "Usage: $0 <exercise_dir> <function_symbol>" >&2
  exit 1
fi

EX_DIR="$1"
FUNC="$2"
TEMPLATE_PATH=".github/templates/benchmark-template.rs"

if [ ! -d "$EX_DIR" ]; then
  echo "Exercise directory '$EX_DIR' not found" >&2
  exit 1
fi

if [ ! -f "$TEMPLATE_PATH" ]; then
  echo "Benchmark template missing at $TEMPLATE_PATH" >&2
  exit 1
fi

BENCH_DIR="${EX_DIR}/benches"
BENCH_FILE="${BENCH_DIR}/$(basename "$EX_DIR")_bench.rs"

mkdir -p "$BENCH_DIR"

if [ -f "$BENCH_FILE" ]; then
  echo "Benchmark file already exists: $BENCH_FILE (skipping)" >&2
  exit 0
fi

# Copy template and patch in function symbol.
cp "$TEMPLATE_PATH" "$BENCH_FILE"
sed -i.bak "s|// use <exercise_crate_name>::target_function;|use $(basename "$EX_DIR")::${FUNC};|" "$BENCH_FILE"
rm -f "$BENCH_FILE.bak"

# Append a real benchmark function snippet.
cat >> "$BENCH_FILE" <<EOF

// Auto-injected benchmark calling ${FUNC}
fn ${FUNC}_bench(c: &mut criterion::Criterion) {
    c.bench_function("${FUNC}", |b| {
        b.iter(|| {
            // Adjust arguments if needed
            criterion::black_box(${FUNC}(/* args if any */));
        })
    });
}

// Extend existing group if user wants to add this automatically (user may edit)
// To activate: modify criterion_group!(benches, baseline_example, ${FUNC}_bench);
EOF

echo "Benchmark harness injected at $BENCH_FILE"

# Ensure dev-dependency criterion exists.
CARGO_TOML="${EX_DIR}/Cargo.toml"
if ! grep -q "\[dev-dependencies\]" "$CARGO_TOML"; then
  printf '\n[dev-dependencies]\n' >> "$CARGO_TOML"
fi
if ! grep -q '^criterion' "$CARGO_TOML"; then
  echo 'criterion = "0.5"' >> "$CARGO_TOML"
  echo "Added criterion dev-dependency to $CARGO_TOML"
fi

# Append bench table if missing.
if ! grep -q "^\[\[bench\]\]" "$CARGO_TOML"; then
  cat >> "$CARGO_TOML" <<EOT

[[bench]]
name = "$(basename "$EX_DIR")_bench"
harness = false
EOT
  echo "Added bench configuration to $CARGO_TOML"
fi

echo "Done. Run: (cd $EX_DIR && cargo bench)"
