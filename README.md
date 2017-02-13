# Rustonly kernel

> Tiny OS kernel written only in Rust (without assembler, except inline). 

After i saw this [masterpiece written](https://github.com/hawkw/sos-kernel) written by Eliza Weisman, i wondered: damn, how can it work without bootloader written in asm? So, i decide to find minimal kernel what can be compilled, works fine and written only in Rust. So, that's it. It simply prints "OK" on green background and then hangs. Just download master and run "make run" in terminal.

### linux dependencies
- `ld`: linker (makes binary out of other files)
- `grub`: creates the bootable iso
- `xorriso`: req'd by grub, filesystem manipulator
- `QEMU`: fake-computer emulator
- `rustc`: nightly version is necessary

### QEMU output
![alt tag](https://github.com/xeqlol/rustonly-kernel/blob/master/qemu.png)
