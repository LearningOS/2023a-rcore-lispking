//! SBI call wrappers

use core::arch::asm;

const SBI_SET_TIMER: usize = 0;
const SBI_CONSOLE_PUTCHAR: usize = 1;
// const SBI_SHUTDOWN: usize = 8;

const SYSTEM_RESET_EXTENSION: usize = 0x53525354;
const SYSTEM_RESET_FUNCTION: usize = 0;
enum SystemResetType {
    Shutdown = 0,
    // ColdReboot = 1,
    // WarmReboot = 2
}
enum SystemResetReason {
    NoReason = 0,
    // SystemFailure = 1
}

/// general sbi call
#[inline(always)]
fn sbi_call(eid: usize, fid: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") arg0 => ret,
            in("x11") arg1,
            in("x12") arg2,
            in("x16") fid,
            in("x17") eid,
        );
    }
    ret
}

/// use sbi call to set timer
pub fn set_timer(timer: usize) {
    sbi_call(SBI_SET_TIMER, 0, timer, 0, 0);
}

/// use sbi call to putchar in console (qemu uart handler)
pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, 0, c, 0, 0);
}

/// use sbi call to shutdown the kernel
pub fn shutdown() -> ! {
    sbi_call(SYSTEM_RESET_EXTENSION, SYSTEM_RESET_FUNCTION, 
        SystemResetType::Shutdown as usize, 
        SystemResetReason::NoReason as usize, 
        0);
    panic!("It should shutdown!");
}
