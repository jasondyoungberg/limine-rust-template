#![no_std]
#![no_main]

use core::arch::asm;

use limine::LimineFramebufferRequest;

static FRAMEBUFFER_REQUEST: LimineFramebufferRequest = LimineFramebufferRequest::new(0);

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    // Ensure we got a framebuffer.
    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response().get() {
        if framebuffer_response.framebuffer_count < 1 {
            hcf();
        }

        // Get the first framebuffer's information.
        let framebuffer = &framebuffer_response.framebuffers()[0];

        for i in 0..100_usize {
            // Calculate the pixel offset using the framebuffer information we obtained above.
            // We skip `i` scanlines (pitch is provided in bytes) and add `i * 4` to skip `i` pixels forward.
            let pixel_offset = i * framebuffer.pitch as usize + i * 4;

            // Write 0xFFFFFFFF to the provided pixel offset to fill it white.
            // We can safely unwrap the result of `as_ptr()` because the framebuffer address is
            // guaranteed to be provided by the bootloader.
            unsafe {
                *(framebuffer.address.as_ptr().unwrap().offset(pixel_offset as isize) as *mut u32) = 0xFFFFFFFF;
            }
        }
    }

    hcf();
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
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
