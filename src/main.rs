#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static TEXT: &[u8] = b"Hello from bare metal world!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // initialize raw pointer at vga buffer location
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in TEXT.iter().enumerate() {
        unsafe {
            // write char byte
            *vga_buffer.offset(i as isize * 2) = byte;
            // write color byte (light cyan)
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
