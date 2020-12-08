#![no_std]
#![no_main]

use core::{fmt::Write, panic::PanicInfo};
use ferr_os::{
    drivers::io::{vgat_out::{VgatChar, VgatOut, DEFAULT_VGA_TEXT_BUFF_HEIGHT, DEFAULT_VGA_TEXT_BUFF_WIDTH},self},
    osattrs,
};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    *io::STDOUT.lock() = Some();

    let mut stdout = VgatOut::default();
    stdout.write_str(osattrs::FERROS_BANNER);

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
