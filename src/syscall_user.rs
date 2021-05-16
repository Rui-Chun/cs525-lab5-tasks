use core::usize;

// the syscall module for user task programs


static SYSCALL_ARG_ADDR: usize = 0x2000_ff00;

enum SyscallTypes {
    ToggleBlue = 0,
    ToggleRed,
    ToggleGreen,
    ToggleOrange,
}

pub fn toggle_blue (task_id: u8) {
    let call_type = SyscallTypes::ToggleBlue;
    unsafe {
        asm_svcall(task_id, call_type);
    }
}

pub fn toggle_red (task_id: u8) {
    let call_type = SyscallTypes::ToggleRed;
    unsafe {
        asm_svcall(task_id, call_type);
    }
}

pub fn toggle_green (task_id: u8) {
    let call_type = SyscallTypes::ToggleGreen;
    unsafe {
        asm_svcall(task_id, call_type);
    }
}

pub fn toggle_orange (task_id: u8) {
    let call_type = SyscallTypes::ToggleOrange;
    unsafe {
        asm_svcall(task_id, call_type);
    }
}

unsafe fn asm_svcall(task_id: u8, call_type: SyscallTypes) {
    let mut arg_ptr = SYSCALL_ARG_ADDR as *mut u8;
    *arg_ptr = task_id;   // assign syscall arg
    *arg_ptr.add(1) = call_type as u8;
    asm!(
        "SVC  0x0",
    )
}