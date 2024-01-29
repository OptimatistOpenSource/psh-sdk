use rkyv::{Archive, Deserialize, Serialize};
use crate::event::Event;

#[derive(Archive, Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
pub enum CacheOp {
    Read,
    Write,
    Prefetch,
}

#[derive(Archive, Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
pub enum CacheOpResult {
    Access,
    Miss,
}

#[derive(Archive, Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
pub enum HardwareEvent {
    CpuCycles,
    Instructions,
    CacheReferences,
    CacheMisses,
    BranchInstructions,
    BranchMisses,
    BusCycles,
    StalledCyclesFrontend,
    StalledCyclesBackend,
    RefCpuCycles,
    CacheL1d(CacheOp, CacheOpResult),
    CacheL1i(CacheOp, CacheOpResult),
    CacheLl(CacheOp, CacheOpResult),
    CacheDtlb(CacheOp, CacheOpResult),
    CacheItlb(CacheOp, CacheOpResult),
    CacheBpu(CacheOp, CacheOpResult),
    CacheNode(CacheOp, CacheOpResult),
}

impl From<HardwareEvent> for Event {
    fn from(value: HardwareEvent) -> Self {
        Self::Hardware(value)
    }
}
