#!/usr/bin/env bash

set -e -o pipefail

cargo test $TEST_FLAGS
