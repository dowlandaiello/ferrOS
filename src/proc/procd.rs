use super::{super::util::iter::Chained, ProcDetails, ProcManager, PID};
use core::{iter::Iterator, slice::Iter};

pub const N_PROCS_PER_BKT: usize = 69;

/// Manages the processes on the system. TODO: Use a read-write mutex for this
pub struct Manager<'a> {
    root: ProcGroupSupervisor<'a>,
    n_procs: usize,
    head: &'a mut ProcGroupSupervisor<'a>,
}

enum ProcGroupSupervisor<'a> {
    /// The immediate children of the group supervisor, along with the number
    /// of alive children
    WithChildren([Option<ProcDetails<'a>>; N_PROCS_PER_BKT], usize),
    /// Downstream supervisors that were spawned by this node to resize the tree
    WithGrandchildren(&'a ProcGroupSupervisor<'a>, &'a ProcGroupSupervisor<'a>),
    Empty,
}

impl<'a> ProcGroupSupervisor<'a> {
    pub fn procs_running(&self) -> Chained<impl Iterator, impl Iterator> {
        match self {
            // Immediate children can be returned as an iter immediately
            Self::WithChildren(children, _) => &mut children
                .into_iter()
                .filter(|maybe_child| maybe_child.is_some())
                .map(|child| child.unwrap().pid),
            // Recursively obtain this node's children
            Self::WithGrandchildren(a, b) => {
                Chained::new(a.procs_running(), Some(b.procs_running()))
            }
        }
    }
}

impl Manager<'_> {
    pub fn new() -> Self {
        let root_node = ProcGroupSupervisor::WithChildren([None; N_PROCS_PER_BKT], 0);

        Self {
            root: root_node,
            n_procs: 0,
            head: &mut root_node,
        }
    }
}

impl<'a> ProcManager<'a> for Manager<'a> {
    fn spawn_proc(&mut self, env: &'a str, cmd: &'a str) -> PID {
        self.n_procs += 1;

        self.n_procs
    }

    fn procs_running<Chained>(&self) -> Chained {
        // Recursively obtain a list of running processes on the system
        self.root.procs_running()
    }

    fn proc_details(&self, proc: PID) -> Option<ProcDetails<'a>> {
        None
    }
}
