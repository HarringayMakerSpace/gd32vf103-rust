# Minimal example of running Rust on Sipeed Longan Nano RISC-V Dev board

See: https://pramode.in/2019/10/07/rust-on-riscv-board-sipeed-longan-nano/

Rust Crate: https://crates.io/crates/gd32vf103-hal

Get the RISC-V target for Rust
```
rustup target add riscv32imac-unknown-none-elf
rustup default nightly
rustup target add riscv32imac-unknown-none-elf
```

Clone this repo, then:
```
cargo build
tools\riscv-nuclei-elf-objcopy.exe -O binary target\riscv32imac-unknown-none-elf\debug\gd32vf103-test target\test.bin
tools\dfu-util.exe -a 0 -s 0x08000000:leave -D target\test.bin
```

the blue LED on the Longan Nano should switch on.
