use super::{PID, ProcManager};

/// Manages the processes on the system.
pub struct Manager {
}

impl<'a> ProcManager<'a> for Manager {
    fn procs_running(&self) -> &[PID] {
    }
}
