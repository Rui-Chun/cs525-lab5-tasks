#![no_std]
#![no_main]
#![feature(asm)]

use core::{usize};

use panic_halt as _;

// for printing msg on host computer
// for task code, this can only print string. No value printing allowed.
use cortex_m_semihosting::{hprintln, hprint};

mod syscall_user;

static TASK_ID: u8 = 1;

fn delay_ms (ms: usize) {
    // hand crafted loop count
    // make is close to accurate
    for _ in 0..ms * 80 {
        unsafe {
            asm!("nop");
        }
    }
}

#[no_mangle]
// #[link_section = ".example_section"]
// #[export_name = "exported_symbol_name"]
pub extern "C" fn task_main() -> ! {
    // can not print varaiables for some reason.
    hprintln!("Task #1 is loaded.").unwrap();
    loop {
        syscall_user::toggle_blue(TASK_ID);
        delay_ms(50);
    }
}