#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Define a trait for dynamic dispatch
pub trait Action {
    fn perform(&self) -> &'static str;
}

// Implement the trait for different types
pub struct SayHello;
impl Action for SayHello {
    fn perform(&self) -> &'static str {
        "Hello!\n"
    }
}

pub struct SayGoodbye;
impl Action for SayGoodbye {
    fn perform(&self) -> &'static str {
        "Goodbye!\n"
    }
}

// A function using dynamic dispatch with &dyn Action
fn execute_action(action: &dyn Action) {
    let message = action.perform();
    // Use syscall to write the message to stdout
    syscall_write(message);
}

// Syscall to write a message to stdout
fn syscall_write(message: &str) {
    let fd: usize = 1; // File descriptor for stdout
    let syscall_number: usize = 1; // Syscall number for write
    let _ = syscall(
        syscall_number,
        fd,
        message.as_ptr() as usize,
        message.len(),
    );
}

// Direct syscall wrapper
fn syscall(number: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    let ret;
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") number,
            in("rdi") arg1,
            in("rsi") arg2,
            in("rdx") arg3,
            lateout("rax") ret,
        );
    }
    ret
}

// Entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let hello = SayHello;
    let goodbye = SayGoodbye;

    // Execute actions using dynamic dispatch
    execute_action(&hello);
    execute_action(&goodbye);

    // Exit syscall
    exit(0);
}

// Exit syscall
fn exit(code: usize) -> ! {
    let syscall_number: usize = 60; // Syscall number for exit
    syscall(syscall_number, code, 0, 0);
    loop {}
}

// Panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
