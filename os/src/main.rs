#![no_main]
#![no_std]
#![feature(panic_info_message)]


use core::arch::global_asm;


#[macro_use]
mod console;
mod sbi;
mod lang_items;
mod logger;
use log::*;
mod sync;
mod batch;
mod syscall;
mod trap;


global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    logger::init(LevelFilter::Trace).expect("Logger initialize failed");

    os_info();

    trap::init();
    batch::init();
    batch::run_next_app();

    panic!("Unreachable in rust_main!");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}

fn os_info() {
    extern "C" {
        fn skernel();
        fn ekernel();
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
    }
    info!("[kernel] kernel\t[{:#x}, {:#x})", skernel as usize, ekernel as usize);
    info!("[kernel] .text\t[{:#x}, {:#x})", stext as usize, etext as usize);
    info!("[kernel] .rodata\t[{:#x}, {:#x})", srodata as usize, erodata as usize);
    info!("[kernel] .data\t[{:#x}, {:#x})", sdata as usize, edata as usize);
    info!("[kernel] .bss\t[{:#x}, {:#x})", sbss as usize, ebss as usize);
}