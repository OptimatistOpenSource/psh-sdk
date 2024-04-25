use std::time::Duration;

use crate::wit::profiling::system::process as proc;

pub use proc::{all, current, ProcessStat, ProcessState};

use super::{StatKey, Update};

impl StatKey for ProcessStat {
    type Key = i32;

    fn key(&self) -> Self::Key {
        self.pid
    }
}

/// Process is the type exposed to sdk user, user can access public field
/// of this type to retrieve certain information belong to specific process
/// # Example
///
/// ```no_run
/// use profiling::system::System;
/// use std::error::Error;
///
/// fn main() -> Result<(), Box<dyn Error>> {
///     let mut system = System::everything()?;
///     loop {
///         std::thread::sleep(std::time::Duration::from_secs(1));
///         system.refresh()?;
///         let mut procs: Vec<_> = system.processes().values().collect();
///         // sort process by cpu_usage and take the top 5 process uses the most cpu
///         procs.sort_unstable_by(|lhs, rhs| rhs.cpu_usage.total_cmp(&lhs.cpu_usage));
///         for proc in procs.iter().take(5) {
///             println!("{:20} {:6.2}%", proc.name, proc.cpu_usage * 100.0);
///         }
///         println!();
///     }
/// }
/// ```
#[derive(Debug)]
pub struct Process {
    /// process id
    pub pid: i32,
    /// process name
    pub name: String,
    /// command line arguments used to start this process
    pub cmd: Vec<String>,
    /// executable
    pub exe: String,
    /// environment variables
    pub environ: Vec<(String, String)>,
    /// current working directory
    pub cwd: String,
    /// root directory
    pub root: String,
    /// user id
    pub user_id: u32,
    /// cpu occupation percentage per core, usually in range [0, 1.0]
    pub cpu_usage: f64,
    /// memory usage in bytes
    pub mem_usage: u64,
    /// disk write, byte per second
    pub write_bps: f64,
    /// disk read, byte per second
    pub read_bps: f64,
    /// process priority for schedular
    pub priority: i64,
    /// process niceness for schedular
    pub nice: i64,
    /// number of threads belongs to process
    pub num_threads: i64,
    /// current process state
    pub state: ProcessState,
    /// virtual memory usage in bytes
    pub virtual_memory_usage: u64,
    /// parent process id
    pub parent_id: i32,

    // private stat kept for usage calculation
    stat: ProcessStat,
}

impl TryFrom<ProcessStat> for Process {
    type Error = String;

    fn try_from(stat: ProcessStat) -> Result<Self, Self::Error> {
        let seconds = 1.0;
        Ok(Self {
            pid: stat.pid,
            name: stat.name.clone(),
            cmd: stat.proc.cmd()?,
            exe: stat.proc.exe()?,
            environ: stat.proc.environ()?,
            cwd: stat.proc.cwd()?,
            root: stat.proc.root()?,
            user_id: stat.proc.user_id()?,
            // treat time period as 1 sec when init
            // utime/stime use ms unit
            cpu_usage: (stat.utime + stat.stime) as f64 / (seconds * 1000.0),
            mem_usage: stat.memory_usage,
            write_bps: stat.written_bytes as f64 / seconds,
            read_bps: stat.read_bytes as f64 / seconds,
            priority: stat.priority,
            nice: stat.nice,
            num_threads: stat.num_threads,
            state: stat.state,
            virtual_memory_usage: stat.virtual_memory_usage,
            parent_id: stat.parent_id,
            stat,
        })
    }
}

impl Update for Process {
    type Stat = ProcessStat;
    fn update(&mut self, stat: ProcessStat, duration: Duration) -> bool {
        let ms = duration.as_millis() as f64;
        let seconds = ms / 1000.0;
        let cpu = ((stat.stime + stat.utime) - (self.stat.stime + self.stat.utime)) as f64 / ms;
        let read = (stat.read_bytes - self.stat.read_bytes) as f64 / seconds;
        let write = (stat.written_bytes - self.stat.written_bytes) as f64 / seconds;
        self.mem_usage = stat.memory_usage;
        self.virtual_memory_usage = stat.virtual_memory_usage;
        self.priority = stat.priority;
        self.nice = stat.nice;
        self.num_threads = stat.num_threads;
        self.state = stat.state;
        self.parent_id = stat.parent_id;
        self.cpu_usage = cpu;
        self.write_bps = write;
        self.read_bps = read;
        self.stat = stat;
        true
    }
}
