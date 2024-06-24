# `lehmer-rs`

## [Small|Medium|Big]Crush

Install testu01 via your package manager or compile it yourself.

Then:
```
cargo build
gcc -std=c99 -Wall -O3 -o lehmer_crush.o lehmer_crush.c -Iinclude -Llib -ltestu01 -lprobdist -lmylib -lm -llehmer -L./../target/debug
LD_LIBRARY_PATH=./../target/debug ./lehmer_crush.o -h
```
`-[s|m|b]` for small, normal and big crush respectively.

Static way:
```
cargo build
gcc -std=c99 -Wall -o lehmer_crush.o lehmer_crush.c -Iinclude -Llib -ltestu01 ../target/debug/liblehmer.so
./lehmer_crush.o -h
```
