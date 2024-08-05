#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;
use limine::request::FramebufferRequest;
use limine::BaseRevision;

#[used]
#[link_section = ".requests"]
static BASEREVISION: BaseRevision = BaseRevision::new();

#[used]
#[link_section = ".requests"]
static FRAMEBUFFER: FramebufferRequest = FramebufferRequest::new();

// Function is called in panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Entry point for our operating system
#[no_mangle]
pub extern "C" fn _start() -> ! {
    assert!(BASEREVISION.is_supported());
    let hello_world = b"Hello world!";
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in hello_world.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize *2 + 1) = 0xb;
        }
    }

    halt_and_clear();
}

fn halt_and_clear() -> ! {
    unsafe {
        asm!("cli");
        loop {
            asm!("hlt");
        }
    }
}
