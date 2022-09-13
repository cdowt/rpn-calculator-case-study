# RPN Calculator Case Study

This repository contains the same application, an RPN calculator,
written in both Rust and C for the purpose of comparing the two. The
target device is an STM32F407G.

## Building

Prerequisites:

- A full rust toolchain (including Cargo) with the
  `thumbv7em-none-eabihf` target installed.
- GCC and binutils targeting `arm-none-eabi`
- A POSIX-compliant Make

To build the Rust program:

```shell
cd rs/
cargo build
```

And to build the C one:

```
cd c/
make
```

To build the Rust program with optimisations enabled, pass the
`--release` flag, i.e. `cargo build --release`. Optimisations are
always enabled for the C program.

## Flashing

Prerequisites:

- OpenOCD
- GDB, either `arm-none-eabi` or multiarch

Start OpenOCD in the background, using the config file in the root of
the repo:

```shell
openocd -f openocd.cfg >openocd.log 2>&1 &
```

Then, to flash the rust program:

```shell
cd rs/
cargo run
load
```

And to flash the C program (assuming it's built):

```shell
cd c/
gdb bin/rpn-calculator
target remote :3333
load
```
