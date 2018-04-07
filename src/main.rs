#![no_std]
#![no_main]

// Required to replace the global allocator.
#![feature(global_allocator)]

// Required to use the `alloc` crate and its types, the `abort` intrinsic, and a
// custom panic handler.
#![feature(alloc, core_intrinsics, lang_items)]

extern crate rlibc;
extern crate alloc;
extern crate wee_alloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Need to provide a tiny `panic_fmt` lang-item implementation for `#![no_std]`.
// This implementation will translate panics into traps in the resulting
// WebAssembly.
#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt(
    _args: ::core::fmt::Arguments,
    _file: &'static str,
    _line: u32
) -> ! {
    use core::intrinsics;
    unsafe {
        intrinsics::abort();
    }
}

#[lang = "eh_personality"] extern fn eh_personality() {}

use alloc::Vec;

#[no_mangle]
pub extern "C" fn WinMainCRTStartup() -> () {
	WinMain()
}

#[no_mangle]
pub extern "C" fn WinMain() -> () {
	let mut vec: Vec<i32> = Vec::new();
	vec.push(1);
	vec.push(2);
}

