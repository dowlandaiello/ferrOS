use super::{ProcDetails, ProcManager, PID};
use core::{
    default::Default,
    iter::{FilterMap, Iterator},
    slice::Iter,
};

/// The maximum number or processes that may be spawned.
pub const MAX_PROCS: usize = u8::MAX as usize;

/// A fixed-size collection of processes.
type ProcCollection<'a> = [Option<ProcDetails<'a>>; MAX_PROCS];

/// Manages the processes on the system. TODO: Use a read-write mutex for this
pub struct Manager<'a> {
    procs: ProcCollection<'a>,
    head: usize,
}

impl Default for Manager<'_> {
    fn default() -> Self {
        Self {
            procs: [None; MAX_PROCS],
            head: 0,
        }
    }
}

impl<'a> ProcManager<'a> for Manager<'a> {
    type I = impl Iterator<Item = &'a PID>;

    fn spawn_proc(&mut self, env: &'a str, cmd: &'a str) -> PID {
        let pid = self.head;
        self.head += 1;

        pid
    }

    // Note: self is explicitly bound to the a lifetime to prevent the results
    // from this function from outliving the process manager itself.
    fn procs_running(&'a self) -> Self::I {
        self.procs
            .iter()
            .filter_map(|proc: &'a Option<ProcDetails>| {
                proc.as_ref().map(|details: &'a ProcDetails| &details.pid)
            })
    }

    fn proc_details(&self, proc: PID) -> Option<ProcDetails<'a>> {
        None
    }
}
