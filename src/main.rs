#![no_std]
#![no_main]

use core::{panic::PanicInfo, fmt::Write};
use ferr_os::drivers::io::vgat_out::VgatOut;

const MSG: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut stdout = VgatOut::default();
    stdout.write_str("Hello, world!").expect("failed to write to stdout");

    loop {} 
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
