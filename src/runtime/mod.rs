use core::{
    default::Default,
    fmt::{self, Write},
};

use crate::drivers::io::vgat_out::VgatOut;
use x86_64::structures::idt::InterruptDescriptorTable;

/// The default IDT implementation.
pub const DEFAULT_IDT: InterruptDescriptorTable = {
    let mut idt = InterruptDescriptorTable::new();
    idt.
    idt
};

/// Different modules can be loaded into the kernel, all of which are optional.
pub type KernelModule<T> = Option<T>;

/// A configuration of various core services provided by the kernel.
pub struct Core<'a> {
    /// The writer used by the runtime for stdout
    stdout: &'a mut (dyn Write),

    /// A sequence printed before the runtime starts.
    startup_greeter: KernelModule<&'a str>,

    /// The descriptor table that should be used to handle interrupts
    idt: InterruptDescriptorTable,
}

impl<'a> Core<'a> {
    pub fn new(
        stdout: KernelModule<&'a mut (dyn Write)>,
        startup_greeter: KernelModule<&'a str>,
        idt: KernelModule<InterruptDescriptorTable>,
    ) -> Self {
        Self {
            stdout.unwrap_or,
            startup_greeter,
            idt: idt.unwrap_or(DEFAULT_IDT),
        }
    }

    /// Obtains a handle for the kernel's standard output.
    pub fn stdout() -> KernelModule<&'a mut (dyn Write)> {
        self.stdout
    }

    /// Obtains a reference to the kernel's greeter.
    pub fn greeter(&self) -> KernelModule<&'a str> {
        self.startup_greeter
    }

    /// Obtains a non-mutable reference to the kernel's interruptor descriptor table.
    pub fn idt() -> InterruptDescriptorTable {
        self.idt
    } 

    #[doc(hidden)]
    pub fn _print(&mut self, args: fmt::Arguments<'_>) {
        if let Some(stdout) = &mut self.stdout {
            stdout.write_fmt(args);
        }
    }
}

#[macro_export]
macro_rules! println {
    ($($rt:ident)?) => (print!($rt, "\n"));
    ($rt:ident,$($arg:tt)*) => ({
        $rt._print(core::format_args_nl!($($arg)*));
    });
}

#[macro_export]
macro_rules! print {
    ($rt:ident,$($arg:tt)*) => ($rt._print(core::format_args!($($arg)*)));
}
