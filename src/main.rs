#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;
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
    /*let hello_world = b"Hello world!";
    let vga_buffer: *mut u8 = match get_video_address() {
        VideoType::Colour => 0xb8000 as *mut u8,
        VideoType::Mono => 0xb0000 as *mut u8,
        VideoType::None => loop {}
    };
    
    for (i, &byte) in hello_world.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize *2 + 1) = 0xb;
        }
    }*/
    if let Some(framebuffer_response) = FRAMEBUFFER.get_response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
            for i in 0..framebuffer.width() {
                // Calculate the pixel offset using the framebuffer information we obtained above.
                // We skip `i` scanlines (pitch is provided in bytes) and add `i * 4` to skip `i` pixels forward.
                let pixel_offset = 10*framebuffer.pitch() + i*4;

                // Write 0xFFFFFFFF to the provided pixel offset to fill it white.
                unsafe {
                    *(framebuffer.addr().add(pixel_offset as usize) as *mut u32) = 0xFFF88F00;
                }
            }
        }
    }

    loop {}
    //halt_and_clear();
}

// Setups ends here, and below this line every function, enum, struct and more should be.
//
// Any code which isnt "setup" shall be moved down here!

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
