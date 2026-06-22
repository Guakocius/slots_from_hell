#!/usr/bin/env bash

RUSTDOCFLAGS="-Z unstable-options --show-coverage" cargo +nightly doc --no-deps
