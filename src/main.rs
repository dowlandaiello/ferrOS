#![no_std]
#![no_main]

use core::{fmt::Write, panic::PanicInfo};
use ferr_os::{
    drivers::io::{
        self,
        vgat_out::{VgatChar, VgatOut, DEFAULT_VGA_TEXT_BUFF_HEIGHT, DEFAULT_VGA_TEXT_BUFF_WIDTH},
    },
    osattrs,
    runtime::Core,
};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut vgatout = VgatOut::default();
    let rt = Core::new(Some(&mut vgatout), Some(osattrs::FERROS_BANNER));

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
