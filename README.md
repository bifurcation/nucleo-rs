# nucleo-rs

This is an attempt to get a Rust environment working on the NUCLEO-L432KC
development board.  The various examples are experiments with exercising
different aspects of the board.

Quickstart on macOS:

```
brew install armmbed/formulae/arm-none-eabi-gcc
brew install openocd

rustup target add thumbv7em-none-eabihf
rustup component add llvm-tools-preview

# In one window...
openocd

# In another window...
cargo run --example hello
cargo run --example blink
cargo run --example i2c
```

This setup is based on the
[cortex-m-quickstart](https://github.com/rust-embedded/cortex-m-quickstart)
example provided by the embedded Rust community.
