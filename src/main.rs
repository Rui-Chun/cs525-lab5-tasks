#![no_std]
#![no_main]
#![feature(asm)]

use core::{u32, usize};

use panic_halt as _;

// for printing msg on host computer
// for task code, this can only print string. No value printing allowed.
use cortex_m_semihosting::{hprintln, hprint};

static TASK_ID: u32 = 1;

 
unsafe extern "C" fn asm_fn() {
    static mut COUNT: usize = 0;
    static mut TEST: usize = 42;
    COUNT += 1;
    // two pointers to msp
    let tmp_ptr = 0x20013F00 as *mut u8;
    *tmp_ptr = ((COUNT + TEST) % 256) as u8;
    asm!(
        "SVC  0x0",
    )
}

pub fn do_something (num: u32) -> u32 {
    return num + 1;
}

#[no_mangle]
// #[link_section = ".example_section"]
// #[export_name = "exported_symbol_name"]
pub extern "C" fn task_main() -> ! {
    let mut num: u32 = 1;
    // can not print varaiables for some reason.
    // hprintln!("Hello, world! from task #{}, num  = {}", TASK_ID, num).unwrap();
    loop {
        if num % 2 == 0 {
            hprint!("T").unwrap();
        }
        else {
            hprint!("t").unwrap();
        }
        num = do_something(num);
        unsafe{
            asm_fn();
        }
    }
}