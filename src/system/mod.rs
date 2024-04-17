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

// abstract identification of some system resource such as Process/Network/Disk
// example: pid for Process, network interface name for network devices
trait StatKey {
    type Key;
    fn key(&self) -> Self::Key;
}

// abstract interface for update resource with some stat
// example: Update Process with ProcessStat
// return value indicates whether this resource should be kept in final resource table
trait Update {
    type Stat;
    fn update(&mut self, stat: Self::Stat, duration: Duration) -> bool;
}

/// System is the main type sdk user need to use to retrieve information about system
/// such as processes/disks/networks
/// # Example
///
/// ```rust
/// use profiling::system::System;
/// use std::error::Error;
///
/// fn main() -> Result<(), Box<dyn Error>> {
///     let mut system = System::new()?;
///     loop {
///         std::thread::sleep(std::time::Duration::from_secs(1));
///         // refresh system stat before retrieve any information
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
pub struct System {
    timestamp: Instant,
    procs: HashMap<i32, process::Process>,
    nets: HashMap<String, network::Network>,
    dsks: HashMap<String, disk::Disk>,
}

impl System {
    // generic function for update Resource table with Vec of stat and duration
    fn update<K, V, S>(resources: &mut HashMap<K, V>, stats: Vec<S>, duration: Duration)
    where
        K: Hash + Eq,
        S: StatKey<Key = K>,
        V: Update<Stat = S> + TryFrom<S>,
    {
        let mut table: HashMap<K, S> = stats.into_iter().map(|s| (s.key(), s)).collect();
        // for a `(key, resource)` pair in `resources`, it no longer exists if it's key
        // is not contained in `table`, in which case it will be removed from `reources`,
        // if it exist in `table`, the corresponding `stat` will be removed from table
        // and used to update `resource`
        resources.retain(|key, res| {
            table
                .remove(key)
                .map_or(false, |stat| res.update(stat, duration))
        });
        // table only contains new `stat` after previous line,
        // we need to convert `stat` to `resource` and insert these new resources to hash table
        // in case some of these conversion failed, we want to filter it out rather than return Err
        // so we use filter_map rather than map&collect
        resources.extend(
            table
                .into_iter()
                .filter_map(|(key, stat)| Some(key).zip(V::try_from(stat).ok())),
        );
    }

    // generic function for initialize a resource table with Vec of stat
    fn init<K, V, S>(stats: Vec<S>) -> HashMap<K, V>
    where
        K: Hash + Eq,
        S: StatKey<Key = K>,
        V: TryFrom<S>,
    {
        // in case some of TryFrom failed, we want to filter it out rather than return Err
        // so we use filter_map&collect rather than map&collect
        stats
            .into_iter()
            .filter_map(|stat| Some(stat.key()).zip(V::try_from(stat).ok()))
            .collect()
    }

    /// create a new system instance for resource query
    pub fn new() -> Result<Self, String> {
        let timestamp = Instant::now();
        let procs = process::all()?;
        let networks = network::stat()?;
        let disks = disk::stat()?;
        Ok(Self {
            timestamp,
            procs: System::init(procs),
            nets: System::init(networks),
            dsks: System::init(disks),
        })
    }

    /// refresh system snapshots,
    /// you should call this before query information such as processes
    pub fn refresh(&mut self) -> Result<(), String> {
        let timestamp = Instant::now();
        let duration = timestamp - self.timestamp;
        let procs = process::all()?;
        let networks = network::stat()?;
        let disks = disk::stat()?;
        System::update(&mut self.procs, procs, duration);
        System::update(&mut self.nets, networks, duration);
        System::update(&mut self.dsks, disks, duration);
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
