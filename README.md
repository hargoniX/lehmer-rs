# `lehmer-rs`

## Crush it
The crushes are a collection of common statistical tests for PRNGs.

Install testu01 via your package manager or compile it yourself.
See: <https://simul.iro.umontreal.ca/testu01/tu01.html>
```sh
sudo apt install libtestu01-0-dev -y
sudo pacman -S testu01
```

### Calling Crushes via Rust
```sh
cargo run --release -- [small-crush|crush|big-crush]
```

Depending on the generator:
- SmallCrush takes some ten seconds
- MediumCrush takes around 20-40 minutes
- BigCrush takes a couple of hours

Also, the current approach doesnt natively work with 64 bit generators since we initialize the generator by the last number.
Remember that one can mark a generator to generate less bits.
Less than 31 bits results in some automatic failures though.

### Interpreting the results from BigCrush
1. smarsa:
  - SerialOver: overlapping t-tuple test
  - CollisionOver: overlapping collision test
  - BirthdaySpacings
2. snpair:
  - ClosePairs: distance between the closest points
3. sknuth:
  - CouponCollector: generates a random sequence of integers and counts how long til all got generated
  - Gap: take an interval and look how many sequences of successive values dont fall into that interval at all
  - Permutation: n non-overlapping vectors of t successive values from the generator - then ordering them and counting the permutations
  - CollisionPermut: how many permutations map to the same value
  - MaxOft: n groups of t values and computes the max for each group
4. svaria:
  - SumCollector: add a sequence of uniforms til their sum exceeds g
5. sspectral:
  - Fourier3:
6. sstring:
  - PeriodsInStrings: sth about the distribution of the correlations in bit strings of a certain length

## NIST
A Statistical Test Suite for Random and Pseudorandom Number Generators for Cryptographic Applications

Requirements to check every available generator via `sts.bash`:
- a built `sts` binary from https://github.com/arcetri/sts
- generated testvector files for the desired iteration count
  (e.g. `cargo run $ITERATIONS generate`)

For the summary of the results, see the directory `nist/`.
In general LCGs are not cryptographically secure and that some configurations pass the testsuite doesnt mean that we have next bit unpredictability in praxis.
