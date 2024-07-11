#!/bin/bash
set -xe

ITERATIONS=${1:-1}
GENERATORS=(
    "BoroshNiederreiter"
    "Crawford"
    "CrayRanf"
    "FastU32"
    "Fishman18"
    "INMOS"
    "LEcuyer"
    "Lemire"
    "NaiveParkMiller"
    "NaiveParkMillerOld"
    "NaiveU32"
    "ParkMillerEfficient"
    "Randu"
    "Waterman"
)

for generator in ${GENERATORS[@]}; do
    ./sts -v 1 -s -i ${ITERATIONS} -I 1 -w nist/${generator}_${ITERATIONS} -F a testdata/${generator}_${ITERATIONS}.binstr
done
