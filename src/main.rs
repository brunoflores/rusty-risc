#![no_main]
#![no_std]

// use core::panic::PanicInfo;
//
// #[panic_handler]
// fn panic(_panic: &PanicInfo<'_>) -> ! {
//     loop {}
// }

extern crate panic_halt;
extern crate riscv_rt;

use riscv_rt::entry;

// use `main` as the entry point of this application
// `main` is not allowed to return
#[entry]
fn main() -> ! {
    // do something here
    loop {}
}
