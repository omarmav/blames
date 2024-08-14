use core::arch::asm;

pub fn qemu_log(message: &str) {
    unsafe {
        for byte in message.bytes() {
            asm!("out 0xe9, al", in("al") byte, options(nostack))
        }
        asm!("out 0xe9, al", in("al") b'\n', options(nostack))
    }
}
