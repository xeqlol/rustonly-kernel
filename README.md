# Rustonly kernel

Tiny OS kernel written only in Rust (without assembler, except inline). It simply prints "OK" on green background and then hangs. Just download master and run "make run" in terminal.
### linux dependencies
- `nasm`: assembler (assembly -> binary)
- `ld`: linker (makes binary out of other files)
- `grub`: creates the bootable iso
- `xorriso`: req'd by grub, filesystem manipulator
- `QEMU`: fake-computer emulator
### QEMU output
![alt tag](https://github.com/xeqlol/rustonly-kernel/blob/master/qemu.png)
