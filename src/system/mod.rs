use std::{
    cmp::Eq,
    collections::HashMap,
    hash::Hash,
    time::{Duration, Instant},
};

pub mod cpu;
pub mod disk;
pub mod interrupt;
pub mod memory;
pub mod network;
pub mod os;
pub mod process;
pub mod rps;

// abstract identification of some system resource, i.e., Process/Network/Disk
// example: pid for Process, network interface name for network devices
trait StatKey {
    type Key;
    fn key(&self) -> Self::Key;
}

// abstract interface for update resource with some stat
// example: Update Process with ProcessStat
trait Update {
    type Stat;
    fn update(&mut self, stat: Self::Stat, duration: Duration) -> bool;
}

bitflags::bitflags! {
    /// Composable ResourceKind type to specify what resources System instance should track.
    #[derive(Debug, Clone, Copy)]
    pub struct ResourceKind : u32 {
        const Disk    = 0b0000_0000_0001;
        const Network = 0b0000_0000_0010;
        const Process = 0b0000_0000_0100;
    }
}

/// System is the main type sdk user need to use to retrieve information about system
/// such as processes/disks/networks
/// # Example
///
/// ```no_run
/// use profiling::system::{ResourceKind, System};
/// use std::error::Error;
///
/// fn main() -> Result<(), Box<dyn Error>> {
///     // initialize system with Process/Disk ResourceKind enabled
///     let mut system = System::with_resource_kinds(ResourceKind::Process | ResourceKind::Disk)?;
///     loop {
///         std::thread::sleep(std::time::Duration::from_secs(1));
///         // refresh system stat before retrieve any information
///         system.refresh()?;
///         let mut procs: Vec<_> = system
///             .processes()
///             .values()
///             // keep only processes start directly by init
///             .filter(|proc| proc.parent_id == 1)
///             .collect();
///         // sort process by cpu_usage and take the top 5 process uses the most cpu
///         procs.sort_unstable_by(|lhs, rhs| rhs.cpu_usage.total_cmp(&lhs.cpu_usage));
///         for proc in procs.iter().take(5) {
///             println!(
///                 "{:20}[{:6}/{:6}] CPU: {:6.2}% | MEM: {:10.2}KiB | REA: {:8.2}KiB/s | WRI: {:8.2}KiB/s",
///                 proc.name,
///                 proc.pid,
///                 proc.parent_id,
///                 proc.cpu_usage * 100.0,
///                 proc.mem_usage as f64 / 1024.0,
///                 proc.read_bps / 1024.0,
///                 proc.write_bps / 1024.0,
///             );
///         }
///         println!();
///     }
/// }
/// ```
pub struct System {
    timestamp: Instant,
    kind: ResourceKind,
    // since we often move resource when updating, we use Box<T> to reduce move cost
    procs: HashMap<i32, Box<process::Process>>,
    nets: HashMap<String, Box<network::Network>>,
    dsks: HashMap<String, Box<disk::Disk>>,
}

impl System {
    // generic function for updating resource hashmap,
    // this will move resources out of old resource hashmap and create
    // new hashmap for performance reasons
    // NOTE: we use this for initialization as well, since initialization is just
    // updating with an empty old resource hashmap and an arbitrary duration
    // this unifies our logic for initialization and update
    fn resources_iteration<K, V, S>(
        old: &mut HashMap<K, Box<V>>,
        stats: Vec<S>,
        duration: Duration,
    ) -> HashMap<K, Box<V>>
    where
        K: Hash + Eq,
        S: StatKey<Key = K>,
        V: Update<Stat = S> + TryFrom<S>,
    {
        stats
            .into_iter()
            .filter_map(|stat| {
                let key = stat.key();
                match old.remove(&key) {
                    Some(mut res) => {
                        res.update(stat, duration);
                        Some((key, res))
                    }
                    None => Some(key).zip(V::try_from(stat).ok().map(Box::new)),
                }
            })
            .collect()
    }

    fn get_processes_stats(kind: ResourceKind) -> Result<Vec<process::ProcessStat>, String> {
        if kind.contains(ResourceKind::Process) {
            process::all()
        } else {
            Ok(vec![])
        }
    }

    fn get_networks_stats(kind: ResourceKind) -> Result<Vec<network::NetworkStat>, String> {
        if kind.contains(ResourceKind::Network) {
            network::stat()
        } else {
            Ok(vec![])
        }
    }

    fn get_disks_stats(kind: ResourceKind) -> Result<Vec<disk::DiskStat>, String> {
        if kind.contains(ResourceKind::Disk) {
            disk::stat()
        } else {
            Ok(vec![])
        }
    }

    /// create a new system instance with every resource kind enabled
    pub fn everything() -> Result<Self, String> {
        let kind = ResourceKind::all();
        Self::with_resource_kinds(kind)
    }

    /// create a new system instance with specified resource kinds
    pub fn with_resource_kinds(kind: ResourceKind) -> Result<Self, String> {
        let timestamp = Instant::now();
        let procs = Self::get_processes_stats(kind)?;
        let networks = Self::get_networks_stats(kind)?;
        let disks = Self::get_disks_stats(kind)?;
        // this duration won't be used inside resources_iteration here
        let duration = std::time::Duration::from_secs(1);
        let procs = Self::resources_iteration(&mut HashMap::new(), procs, duration);
        let nets = Self::resources_iteration(&mut HashMap::new(), networks, duration);
        let dsks = Self::resources_iteration(&mut HashMap::new(), disks, duration);
        Ok(Self {
            timestamp,
            procs,
            nets,
            dsks,
            kind,
        })
    }

    /// refresh system snapshots,
    /// you should call this before query information, i.e., processes,
    /// for performance and accuracy reasons, you should wait at least 100ms between
    /// each refresh calls, 1000ms is recommanded.
    pub fn refresh(&mut self) -> Result<(), String> {
        let timestamp = Instant::now();
        let duration = timestamp - self.timestamp;
        let procs = Self::get_processes_stats(self.kind)?;
        let networks = Self::get_networks_stats(self.kind)?;
        let disks = Self::get_disks_stats(self.kind)?;
        self.procs = Self::resources_iteration(&mut self.procs, procs, duration);
        self.nets = Self::resources_iteration(&mut self.nets, networks, duration);
        self.dsks = Self::resources_iteration(&mut self.dsks, disks, duration);
        // update timestamp after updated other resources
        self.timestamp = timestamp;
        Ok(())
    }

    /// retrieve processes
    pub fn processes(&self) -> &HashMap<i32, Box<process::Process>> {
        &self.procs
    }

    /// retrieve networks
    pub fn networks(&self) -> &HashMap<String, Box<network::Network>> {
        &self.nets
    }

    /// retrieve disks
    pub fn disks(&self) -> &HashMap<String, Box<disk::Disk>> {
        &self.dsks
    }
}
