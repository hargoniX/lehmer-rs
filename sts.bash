#!/bin/bash
# Expects:
# - a built `sts` binary from https://github.com/arcetri/sts
# - generated testvector files for the desired iteration count
#   (e.g. cargo run $ITERATIONS generate)
set -xe

ITERATIONS=${1:-1}
GENERATORS=(
    "BoroshNiederreiter"
    "Crawford"
    "CrayRanf"
    "FastU32"
    "Fishman18"
    "LEcuyer"
    "NaiveParkMiller"
    "NaiveParkMillerOld"
    "NaiveU32"
    "ParkMillerEfficient"
    "Randu"
    "Waterman"
    "INMOS"
)

for generator in ${GENERATORS[@]}; do
    ./sts -v 1 -s -i ${ITERATIONS} -I 1 -w nist/${generator}_${ITERATIONS} -F a testdata/${generator}_${ITERATIONS}.binstr
done
