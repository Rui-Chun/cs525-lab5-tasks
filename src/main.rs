#![no_std]
#![no_main]

use panic_halt as _;

// for printing msg on host computer
use cortex_m_semihosting::{hprintln, hprint};

static TASK_INFO: u32 = 1;

#[no_mangle]
// #[link_section = ".example_section"]
// #[export_name = "exported_symbol_name"]
pub extern "C" fn task_main() {
    let num = 42;
    hprintln!("Hello, world! from task #{}, num  = {}", TASK_INFO, num);
    for i in 0..100 {
        hprintln!("Task #{}, couting {}", TASK_INFO, i);
    }
}