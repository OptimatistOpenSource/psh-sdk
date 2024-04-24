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
/// ```rust
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
    procs: HashMap<i32, process::Process>,
    nets: HashMap<String, network::Network>,
    dsks: HashMap<String, disk::Disk>,
}

impl System {
    // generic function for updating resource hashmap
    fn update<K, V, S>(resources: &mut HashMap<K, V>, stats: Vec<S>, duration: Duration)
    where
        K: Hash + Eq,
        S: StatKey<Key = K>,
        V: Update<Stat = S> + TryFrom<S>,
    {
        // can we do better?
        let mut table: HashMap<K, S> = stats.into_iter().map(|s| (s.key(), s)).collect();
        // for a (key, resource) in hashMap resources, it will be removed if the key is not in table or update()
        // against resource failed returns false, otherwise, the hashMap resources contains latest resources status.
        resources.retain(|key, res| {
            table
                .remove(key)
                .map_or(false, |stat| res.update(stat, duration))
        });
        // after updating, extend hashMap resources to cover new resources that are remained in table.
        resources.extend(
            table
                .into_iter()
                .filter_map(|(key, stat)| Some(key).zip(V::try_from(stat).ok())),
        );
    }

    // generic function for initializing resource hashmap
    fn init<K, V, S>(stats: Vec<S>) -> HashMap<K, V>
    where
        K: Hash + Eq,
        S: StatKey<Key = K>,
        V: TryFrom<S>,
    {
        stats
            .into_iter()
            .filter_map(|stat| {
                let key = stat.key();
                Some(key).zip(V::try_from(stat).ok())
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
        Ok(Self {
            timestamp,
            procs: Self::init(procs),
            nets: Self::init(networks),
            dsks: Self::init(disks),
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
        Self::update(&mut self.procs, procs, duration);
        Self::update(&mut self.nets, networks, duration);
        Self::update(&mut self.dsks, disks, duration);
        // update timestamp after updated other resources
        self.timestamp = timestamp;
        Ok(())
    }

    /// retrieve processes
    pub fn processes(&self) -> &HashMap<i32, process::Process> {
        &self.procs
    }

    /// retrieve networks
    pub fn networks(&self) -> &HashMap<String, network::Network> {
        &self.nets
    }

    /// retrieve disks
    pub fn disks(&self) -> &HashMap<String, disk::Disk> {
        &self.dsks
    }
}
