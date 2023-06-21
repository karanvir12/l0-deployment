#!/bin/bash

# Runs all benchmarks for all pallets, for each of the runtimes specified below
# Should be run on a reference machine to gain accurate benchmarks
# current reference machine: https://github.com/karanvir12/substrate36.git/pull/5848

runtimes=(
  polkadot

)

for runtime in "${runtimes[@]}"; do
  "$(dirname "$0")/run_benches_for_runtime.sh" "$runtime"
done
