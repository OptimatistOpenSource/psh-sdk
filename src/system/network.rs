use std::time::Duration;

use crate::wit::profiling::system::network;

pub use network::{stat, NetworkStat};

use super::{StatKey, Update};

impl StatKey for NetworkStat {
    type Key = String;

    fn key(&self) -> Self::Key {
        self.name.clone()
    }
}

/// Network type exposed to sdk user
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
///         system.refresh()?;
///         let mut nets: Vec<_> = system.networks().values().filter(|net| net.name != "lo").collect();
///         // filter out loopback device and sort networks by recv_bps
///         nets.sort_unstable_by(|lhs, rhs| rhs.recv_bps.total_cmp(&lhs.recv_bps));
///         for net in nets {
///             println!("{:12} Sent: {:8.2}KiB/s Recv: {:8.2}KiB/s", net.name, net.sent_bps / 1024.0, net.recv_bps / 1024.0);
///         }
///         println!();
///     }
/// }
/// ```
pub struct Network {
    /// name of network interface, example: wlan0
    pub name: String,
    /// sent speed: byte per second
    pub sent_bps: f64,
    /// receive speed: byte per second
    pub recv_bps: f64,
    /// sent speed: packet per second
    pub sent_pps: f64,
    /// receive speed: packet per second
    pub recv_pps: f64,

    // private stat kept for usage calculation
    stat: NetworkStat,
}

impl TryFrom<NetworkStat> for Network {
    type Error = String;

    fn try_from(stat: NetworkStat) -> Result<Self, Self::Error> {
        let seconds = 1.0;
        let name = stat.name.clone();
        let sent_bps = stat.sent_bytes as f64 / seconds;
        let recv_bps = stat.recv_bytes as f64 / seconds;
        let sent_pps = stat.sent_packets as f64 / seconds;
        let recv_pps = stat.recv_packets as f64 / seconds;
        Ok(Self {
            stat,
            name,
            sent_bps,
            recv_bps,
            sent_pps,
            recv_pps,
        })
    }
}

impl Update for Network {
    type Stat = NetworkStat;

    fn update(&mut self, stat: Self::Stat, duration: Duration) -> bool {
        let seconds = duration.as_millis() as f64 / 1000.0;
        self.sent_bps = (stat.sent_bytes - self.stat.sent_bytes) as f64 / seconds;
        self.recv_bps = (stat.recv_bytes - self.stat.recv_bytes) as f64 / seconds;
        self.sent_pps = (stat.sent_packets - self.stat.sent_packets) as f64 / seconds;
        self.recv_pps = (stat.recv_packets - self.stat.recv_packets) as f64 / seconds;
        self.stat = stat;
        true
    }
}
