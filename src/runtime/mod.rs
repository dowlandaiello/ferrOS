/// Different modules can be loaded into the kernel, all of which are optional.
pub type KernelModule = Option;

/// A configuration of various core services provided by the kernel.
pub struct Core<'a> {
    stdout: KernelModule<&'a mut
}
