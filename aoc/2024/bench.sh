#!/bin/bash

set -ex

cargo build --release
./target/release/aoc-2024
