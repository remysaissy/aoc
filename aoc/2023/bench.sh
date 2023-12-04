#!/bin/bash

set -ex

cargo build --release
time ./target/release/aoc-2023
