#!/usr/bin/env bash

set -e -o pipefail

cargo test $TEST_FLAGS

for test in tests/*.rs; do
    ! cargo test --test $(basename $test .rs) --features failure
done
