use crate::event::Event;
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
pub enum SoftwareEvent {
    CpuClock,
    TaskClock,
    PageFaults,
    ContextSwitches,
    CpuMigrations,
    PageFaultsMin,
    PageFaultsMaj,
    AlignmentFaults,
    EmulationFaults,
    Dummy,
    BpfOutput,
    CgroupSwitches,
}

impl From<SoftwareEvent> for Event {
    fn from(value: SoftwareEvent) -> Self {
        Self::Software(value)
    }
}
