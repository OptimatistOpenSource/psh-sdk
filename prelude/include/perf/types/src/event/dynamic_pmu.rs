use alloc::boxed::Box;
use rkyv::{Archive, Deserialize, Serialize};
use crate::event::Event;

#[derive(Archive, Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
pub enum KprobeConfig {
    FuncAndOffset {
        /// `[u8]` should be a valid `CStr`
        kprobe_func: Box<[u8]>,
        probe_offset: u64,
    },
    KprobeAddr(u64),
}

#[derive(Archive, Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
pub struct UprobeConfig {
    /// `[u8]` should be a valid `CStr`
    pub uprobe_path: Box<[u8]>,
    pub probe_offset: u64,
}

#[derive(Archive, Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
pub enum DynamicPmuEvent {
    Other {
        /// The content of `/sys/bus/event_source/devices/*/type`
        r#type: u32,
        /// See: `/sys/bus/event_source/devices/*/format/*`
        /// and `/sys/bus/event_source/devices/*/events/*`
        config: u64,
    },
    Kprobe {
        /// The content of `/sys/bus/event_source/devices/kprobe/type`
        r#type: u32,
        /// See `/sys/bus/event_source/devices/kprobe/format/retprobe`
        retprobe: bool,
        cfg: KprobeConfig,
    },
    Uprobe {
        /// The content of `/sys/bus/event_source/devices/uprobe/type`
        r#type: u32,
        /// See `/sys/bus/event_source/devices/uprobe/format/retprobe`
        retprobe: bool,
        cfg: UprobeConfig,
    },
}

impl From<DynamicPmuEvent> for Event {
    fn from(value: DynamicPmuEvent) -> Self {
        Self::DynamicPmu(value)
    }
}
