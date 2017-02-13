#![feature(asm)]
#![feature(lang_items)]
#![feature(naked_functions)]
#![feature(linkage)]
#![no_std]

mod multiboot;

#[cold]
#[no_mangle]
#[naked]
#[linkage = "external"]
#[link_section = ".text"]
pub unsafe extern "C" fn _start() {
    *(0xb8000 as *mut u16) = 0x2f4f; // O
    *(0xb8002 as *mut u16) = 0x2f4b; // K
    unsafe {
        asm!("hlt");
    }
}

use core::fmt;

#[lang = "eh_personality"] 
pub extern fn eh_personality() {}

#[lang = "panic_fmt"] 
#[no_mangle] 
pub extern fn panic_fmt(args: fmt::Arguments, file: &'static str, line: u32) -> ! {
	loop {}
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn _Unwind_Resume()
{
	loop{}
}

#[cfg_attr(all(feature = "weak", not(windows), not(target_os = "macos")), linkage = "weak")]
#[no_mangle]
pub unsafe extern fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *dest.offset(i as isize) = *src.offset(i as isize);
        i += 1;
    }
    return dest;
}