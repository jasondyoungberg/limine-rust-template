#![no_std]
#![no_main]

use core::arch::asm;
use limine::LimineBootInfoRequest;

mod writer;

static BOOTLOADER_INFO: LimineBootInfoRequest = LimineBootInfoRequest::new(0);

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    println!("hello, world!");

    if let Some(bootinfo) = BOOTLOADER_INFO.get_response().get() {
        println!(
            "bootloader: {} v{}",
            bootinfo.name.to_str().unwrap().to_str().unwrap(),
            bootinfo.version.to_str().unwrap().to_str().unwrap(),
        );
    }

    hcf();
}

#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    println!("KERNEL PANIC:\n{info}");
    hcf();
}

fn hcf() -> ! {
    unsafe {
        asm!("cli");
        loop {
            asm!("hlt");
        }
    }
}
