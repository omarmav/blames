#![no_std]
#![no_main]

extern crate alloc;

mod graphics;
mod debug;

// All alloc related libraries
use alloc::borrow::ToOwned;
use alloc::vec;
use alloc::vec::Vec;
use linked_list_allocator::LockedHeap;
// Core libraries
use core::arch::asm;
use core::panic::PanicInfo;
// Limine
use limine::request::FramebufferRequest;
use limine::BaseRevision;
// Debug
use debug::qemu_log;

#[used]
#[link_section = ".requests"]
static BASEREVISION: BaseRevision = BaseRevision::new();

#[used]
#[link_section = ".requests"]
static FRAMEBUFFER: FramebufferRequest = FramebufferRequest::new();

// Set up our allocator
#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

// Function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Entry point for our operating system
#[no_mangle]
pub extern "C" fn _start() -> ! {
    qemu_log("_start reached, entering main");
    main();
    //halt_and_clear();
}


// Setups ends here, and below this line every function, enum, struct and more should be.
//
// Any code which isnt "setup" shall be moved down here!

fn main() -> ! {
    //let mut double_framebuffer: Vec<u32>;
    assert!(BASEREVISION.is_supported());
    qemu_log("Base revision is supported, getting framebuffer next");
    if let Some(framebuffer_response) = FRAMEBUFFER.get_response() {
        // Now that we've gotten the framebuffer we can actually create our double buffer
        /*if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
            let buffer_size = framebuffer.width() * framebuffer.height() * 4;
            double_framebuffer = vec![0x0; buffer_size as usize];
            for y in 0..framebuffer.height() {
                for x in 0..framebuffer.width() {
                    // Calculate the pixel offset using the framebuffer information we obtained above.
                    // We skip `i` scanlines (pitch is provided in bytes) and add `i * 4` to skip `i` pixels forward.
                    // x + y

                    let pixel_offset = x * 4 + y * framebuffer.pitch();

                    /*unsafe {
                        *(framebuffer.addr().add(pixel_offset as usize) as *mut u32) =
                            if x % 4 != 0 { 0xFFF88F00 } else { 0xFF00FF00 };
                    }*/
                    if let Some(offset) = double_framebuffer.get_mut(pixel_offset as usize) {
                        *offset = if x % 4 != 0 { 0xFFF88F00 } else { 0xFF00FF00 };
                    }
                }
            }
            unsafe {
                write_into_framebuffer(framebuffer.addr(), double_framebuffer);
            }
        }*/
    }

    loop {}
}

/*unsafe fn write_into_framebuffer(framebuffer: *mut u8, double_buffer: Vec<u32>) {
    for (offset, byte) in double_buffer.iter().enumerate() {
        *(framebuffer.add(offset) as *mut u32) = byte.to_owned();
    }
}*/

enum VideoType {
    None = 0x00,
    Colour = 0x20,
    Mono = 0x30,
}

impl VideoType {
    fn from_bios_bit_flags(val: u8) -> VideoType {
        match val {
            0x20 => VideoType::Colour,
            0x30 => VideoType::Mono,
            _ => VideoType::None,
        }
    }
}

fn get_video_address() -> VideoType {
    unsafe {
        let bios_bit_flags = 0x410 as *mut u8;
        return VideoType::from_bios_bit_flags(*bios_bit_flags & 0x30);
    }
}

fn halt_and_clear() -> ! {
    unsafe {
        asm!("cli");
        loop {
            asm!("hlt");
        }
    }
}
