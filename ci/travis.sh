#!/usr/bin/env bash

set -e -o pipefail

cargo test $FEATURES
