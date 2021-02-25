#![no_std]
#![no_main]
#![feature(format_args_nl)]

use core::{fmt::Write, panic::PanicInfo};
use ferr_os::{
    drivers::io::{
        self,
        vgat_out::{VgatChar, VgatOut, DEFAULT_VGA_TEXT_BUFF_HEIGHT, DEFAULT_VGA_TEXT_BUFF_WIDTH},
    },
    osattrs,
    runtime::{Core},
    println,
};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut vgatout = VgatOut::default();
    let mut rt: Core<'static> = Core::new(Some(osattrs::FERROS_BANNER), None);
    let greeter = rt.greeter().unwrap();
    println!(rt, "{}", greeter);

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
