# Rustonly kernel

> Tiny OS kernel written only in Rust (without assembler, except inline). 

After i saw this [masterpiece](https://github.com/hawkw/sos-kernel) written by Eliza Weisman, i wondered: damn, how can it works without bootloader written in asm? So, i decide to find minimal kernel that can be compiled, works fine and written only in Rust. Here it is. It simply prints "OK" on green background and then hangs. 

### build
- Just download master and run "make run" in terminal.

### linux dependencies
- `ld`: linker (makes binary out of other files)
- `grub`: creates the bootable iso
- `xorriso`: req'd by grub, filesystem manipulator
- `QEMU`: fake-computer emulator
- `cargo/rustc`: Rust lang compiler (!) nightly version is necessary (!)

### QEMU output
![alt tag](https://github.com/xeqlol/rustonly-kernel/blob/master/qemu.png)
