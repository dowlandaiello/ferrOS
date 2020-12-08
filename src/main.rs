#![no_std]
#![no_main]

use core::{fmt::Write, panic::PanicInfo};
use ferr_os::{
    drivers::io::vgat_out::{VgatChar, VgatOut},
    osattrs,
};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut stdout = VgatOut::default();
    stdout.write_str(osattrs::FERROS_BANNER);

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
