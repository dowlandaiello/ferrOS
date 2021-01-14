use super::{ProcDetails, ProcManager, PID};
use core::{
    iter::{Iterator, Map},
    slice::Iter,
};

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

impl ProcGroupSupervisor<'_> {
    pub fn procs_running(&self) -> &[PID] {
        match self {
            // Immediate children can be returned as an iter immediately
            Self::WithChildren(children, _) => children
                .into_iter()
                .filter(|maybe_child| maybe_child.is_some())
                .map(|child| child.unwrap().pid)
                .collect::<&[PID]>(),
            // Recursively obtain this node's children
            Self::WithGrandchildren(a, b) => a.iter().chain(b.iter()).collect::<&[PID]>(),
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

    fn procs_running<Map>(&self) -> Map {
        // Recursively obtain a list of running processes on the system
        self.root.procs_running()
    }

    fn proc_details(&self, proc: PID) -> Option<ProcDetails<'a>> {
        None
    }
}
