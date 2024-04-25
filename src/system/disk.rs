use std::time::Duration;

use crate::wit::profiling::system::disk;

pub use disk::{stat, DiskOperationStat, DiskStat};

use super::{StatKey, Update};

impl StatKey for DiskStat {
    type Key = String;

    fn key(&self) -> Self::Key {
        self.name.clone()
    }
}

/// Disk type exposed to sdk users
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
///         let mut disks: Vec<_> = system.disks().values().collect();
///         // sort disks by write_bps
///         disks.sort_unstable_by(|lhs, rhs| rhs.write_bps.total_cmp(&lhs.write_bps));
///         for disk in disks {
///             println!("{:20} Write: {:8.2}KiB/s Read: {:8.2}KiB/s", disk.name, disk.write_bps / 1024.0, disk.read_bps / 1024.0);
///         }
///         println!();
///     }
/// }
/// ```
pub struct Disk {
    /// disk interface name such as sda1
    pub name: String,
    /// disk read byte per second
    pub read_bps: f64,
    /// disk write byte per second
    pub write_bps: f64,
    /// disk read operation per second
    pub read_ops: f64,
    /// disk write operation per second
    pub write_ops: f64,

    // private stat kept for usage calculation
    stat: DiskStat,
}

impl TryFrom<DiskStat> for Disk {
    type Error = String;

    fn try_from(stat: DiskStat) -> Result<Self, Self::Error> {
        let seconds = 1.0;
        let read_bps = stat.read.sectors as f64 * 512.0 / seconds;
        let write_bps = stat.write.sectors as f64 * 512.0 / seconds;
        let read_ops = stat.read.operations as f64 / seconds;
        let write_ops = stat.write.operations as f64 / seconds;
        Ok(Self {
            name: stat.name.clone(),
            read_bps,
            write_bps,
            read_ops,
            write_ops,
            stat,
        })
    }
}

impl Update for Disk {
    type Stat = DiskStat;
    fn update(&mut self, stat: Self::Stat, duration: Duration) -> bool {
        let seconds = duration.as_millis() as f64 / 1000.0;
        self.read_bps = (stat.read.sectors - self.stat.read.sectors) as f64 * 512.0 / seconds;
        self.write_bps = (stat.write.sectors - self.stat.write.sectors) as f64 * 512.0 / seconds;
        self.read_ops = (stat.read.operations - self.stat.read.operations) as f64 / seconds;
        self.write_ops = (stat.write.operations - self.stat.write.operations) as f64 / seconds;
        self.stat = stat;
        true
    }
}
