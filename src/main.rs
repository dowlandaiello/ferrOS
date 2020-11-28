#![no_std]
#![no_main]

use core::panic::PanicInfo;

const MSG: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vid_buf = 0xb8000 as *mut u8;

    for (i, &byte) in MSG.iter().enumerate() {
        unsafe {
            *vid_buf.offset(i as isize * 2) = byte;
            *vid_buf.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {} 
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
