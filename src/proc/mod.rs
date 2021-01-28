use core::iter::Iterator;

/// The default process manager employed by the system.
pub mod procd;

/// A unique identifier attached to each process on the system.
pub type PID = usize;

/// Any system capable of managing the processes running on the system and
/// providing relevant information when required. May or may not be the same
/// system as the init system.
pub trait ProcManager<'a> {
    /// Executes a new process and adds it to the procmanager by spawning a
    /// process with the indicated tty, environment, command & working dir
    fn spawn_proc(&mut self, env: &'a str, cmd: &'a str) -> PID;

    /// Obtains a list of PIDs representing the processes running on the system
    /// at this point in time.
    fn procs_running<T: Iterator<Item = &'a PID>>(&self) -> T;

    /// Obtains the PID, TTY, status, runtime, memory usage and target. Returns
    /// None if the process can't be found.
    fn proc_details(&self, proc: PID) -> Option<ProcDetails<'a>>;

    /// Determines whether or not a process with the given PID is running.
    fn is_running(&self, proc: PID) -> bool {
        self.proc_details(proc).is_some()
    }
}

/// Details of a process running on the system.
pub struct ProcDetails<'a> {
    pid: PID,
    tty: &'a str,
    status: ProcStatus,

    // Env variables
    env: &'a str,

    // Target
    cmd: &'a str,

    // Working directory
    cwd: &'a str,
}

impl ProcDetails<'_> {
    pub fn pid(&self) -> PID {
        self.pid
    }

    pub fn tty(&self) -> &str {
        self.tty
    }

    pub fn status(&self) -> ProcStatus {
        self.status
    }

    pub fn env(&self) -> &str {
        self.env
    }

    pub fn cmd(&self) -> &str {
        self.cmd
    }

    pub fn cwd(&self) -> &str {
        self.cwd
    }
}

/// The state of a process running on the system.
#[derive(Clone, Copy)]
pub enum ProcStatus {
    Running,
    Sleeping,
    SleepingUnint,
    Dead,
}
