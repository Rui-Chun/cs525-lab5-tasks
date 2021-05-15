#![no_std]
#![no_main]
#![feature(asm)]

use core::{u32, usize};

use panic_halt as _;

// for printing msg on host computer
// for task code, this can only print string. No value printing allowed.
use cortex_m_semihosting::{hprintln, hprint};

static TASK_ID: u32 = 1;
static SYSCALL_ARG_ADDR: usize = 0x2000_ff00;
 
unsafe extern "C" fn asm_fn() {
    let tmp_ptr = SYSCALL_ARG_ADDR as *mut u8;
    *tmp_ptr = 1;   // assign syscall arg
    asm!(
        "SVC  0x0",
    )
}

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
    // hprintln!("Hello, world! from task #{}, num  = {}", TASK_ID, num).unwrap();
    loop {
        unsafe{
            asm_fn();
        }
        delay_ms(250);
    }
}