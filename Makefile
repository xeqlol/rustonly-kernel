target ?= x86_64-unknown-linux-gnu
kernel := rustonly_kernel.bin
iso := rustonly_kernel.iso

rust_os := target/$(target)/debug/librustonly_kernel.a
linker_script := linker.ld
grub_cfg := grub.cfg

# reserve names
.PHONY: all clean run iso

# make
all: $(kernel)

# make clean
clean:
	@rm -r build

# make run
run: $(iso)
	@qemu-system-x86_64 -cdrom $(iso)

# make iso
iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	@mkdir -p build/isofiles/boot/grub
	@cp $(kernel) build/isofiles/boot/kernel.bin
	@cp $(grub_cfg) build/isofiles/boot/grub
	@grub-mkrescue -o $(iso) build/isofiles 2> /dev/null
	@rm -r build/isofiles

$(kernel): cargo $(rust_os) $(linker_script)
	@ld -n --gc-sections -T $(linker_script) -o $(kernel) $(rust_os)

# cargo build
cargo:
	@cargo build --target $(target)