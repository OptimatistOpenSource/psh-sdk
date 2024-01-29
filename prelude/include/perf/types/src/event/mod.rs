use rkyv::{Archive, Deserialize, Serialize};

mod breakpoint;
mod dynamic_pmu;
mod hardware;
mod raw;
mod scope;
mod software;
mod tracepoint;

pub use breakpoint::*;
pub use dynamic_pmu::*;
pub use hardware::*;
pub use raw::*;
pub use scope::*;
pub use software::*;
pub use tracepoint::*;

#[derive(Archive, Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
pub enum Event {
    Hardware(HardwareEvent),
    Software(SoftwareEvent),
    Raw(RawEvent),
    Tracepoint(TracepointEvent),
    Breakpoint(BreakpointEvent),
    DynamicPmu(DynamicPmuEvent),
}
