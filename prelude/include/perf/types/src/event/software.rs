use rkyv::{Archive, Deserialize, Serialize};
use crate::event::Event;

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
