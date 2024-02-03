use crate::op::perf::convert::Wrap;

use std::ffi::CString;
use std::rc::Rc;

use perf_event_rs::event::{
    BreakpointEvent as RawBpEv, BreakpointType as RawBpTy, CacheOp as RawCacheOp,
    CacheOpResult as RawCacheOpResult, Event as RawEv, HardwareEvent as RawHwEv,
    KprobeConfig as RawKpCfg, RawEvent as RawRawEv, SoftwareEvent as RawSwEv,
    TracepointEvent as RawTpEv, UprobeConfig as RawUpCfg,
};
use perf_event_rs::{BreakpointLen as RawBpLen, DynamicPmuEvent as RawDpEv};
use profiling_prelude_perf_types::event::{
    BreakpointLen as BpLen, BreakpointType as BpTy, CacheOp, CacheOpResult,
    DynamicPmuEvent as DpEv, Event as Ev, HardwareEvent as HwEv, KprobeConfig as KpCfg,
    SoftwareEvent as SwEv,
};

type FromT = Ev;
type IntoT = RawEv;

impl From<&FromT> for Wrap<IntoT> {
    fn from(value: &FromT) -> Self {
        let val = into_raw_event(value);
        Self(val)
    }
}

#[rustfmt::skip]
fn into_raw_cache_op(val: &CacheOp) -> RawCacheOp {
    match val {
        CacheOp::Read     => RawCacheOp::Read,
        CacheOp::Write    => RawCacheOp::Write,
        CacheOp::Prefetch => RawCacheOp::Prefetch,
    }
}

#[rustfmt::skip]
fn into_raw_cache_op_result(val: &CacheOpResult) -> RawCacheOpResult {
    match val {
        CacheOpResult::Access => RawCacheOpResult::Access,
        CacheOpResult::Miss   => RawCacheOpResult::Miss,
    }
}

fn into_raw_bp_len(val: &BpLen) -> RawBpLen {
    match val {
        BpLen::Len1 => RawBpLen::Len1,
        BpLen::Len2 => RawBpLen::Len2,
        BpLen::Len3 => RawBpLen::Len3,
        BpLen::Len4 => RawBpLen::Len4,
        BpLen::Len5 => RawBpLen::Len5,
        BpLen::Len6 => RawBpLen::Len6,
        BpLen::Len7 => RawBpLen::Len7,
        BpLen::Len8 => RawBpLen::Len8,
    }
}

fn into_raw_event(ev: &Ev) -> RawEv {
    let val = match ev {
        #[rustfmt::skip]
        Ev::Hardware(ev) => RawEv::Hardware(match ev {
            HwEv::CpuCycles             => RawHwEv::CpuCycles,
            HwEv::Instructions          => RawHwEv::Instructions,
            HwEv::CacheReferences       => RawHwEv::CacheReferences,
            HwEv::CacheMisses           => RawHwEv::CacheMisses,
            HwEv::BranchInstructions    => RawHwEv::BranchInstructions,
            HwEv::BranchMisses          => RawHwEv::BranchMisses,
            HwEv::BusCycles             => RawHwEv::BusCycles,
            HwEv::StalledCyclesFrontend => RawHwEv::StalledCyclesFrontend,
            HwEv::StalledCyclesBackend  => RawHwEv::StalledCyclesBackend,
            HwEv::RefCpuCycles          => RawHwEv::RefCpuCycles,
            HwEv::CacheL1d (o, r)       => RawHwEv::CacheL1d (into_raw_cache_op(o), into_raw_cache_op_result(r)),
            HwEv::CacheL1i (o, r)       => RawHwEv::CacheL1i (into_raw_cache_op(o), into_raw_cache_op_result(r)),
            HwEv::CacheLl  (o, r)       => RawHwEv::CacheLl  (into_raw_cache_op(o), into_raw_cache_op_result(r)),
            HwEv::CacheDtlb(o, r)       => RawHwEv::CacheDtlb(into_raw_cache_op(o), into_raw_cache_op_result(r)),
            HwEv::CacheItlb(o, r)       => RawHwEv::CacheItlb(into_raw_cache_op(o), into_raw_cache_op_result(r)),
            HwEv::CacheBpu (o, r)       => RawHwEv::CacheBpu (into_raw_cache_op(o), into_raw_cache_op_result(r)),
            HwEv::CacheNode(o, r)       => RawHwEv::CacheNode(into_raw_cache_op(o), into_raw_cache_op_result(r)),
        }),
        #[rustfmt::skip]
        Ev::Software(ev) => RawEv::Software(match ev {
            SwEv::CpuClock        => RawSwEv::CpuClock,
            SwEv::TaskClock       => RawSwEv::TaskClock,
            SwEv::PageFaults      => RawSwEv::PageFaults,
            SwEv::ContextSwitches => RawSwEv::ContextSwitches,
            SwEv::CpuMigrations   => RawSwEv::CpuMigrations,
            SwEv::PageFaultsMin   => RawSwEv::PageFaultsMin,
            SwEv::PageFaultsMaj   => RawSwEv::PageFaultsMaj,
            SwEv::AlignmentFaults => RawSwEv::AlignmentFaults,
            SwEv::EmulationFaults => RawSwEv::EmulationFaults,
            SwEv::Dummy           => RawSwEv::Dummy,
            SwEv::BpfOutput       => RawSwEv::BpfOutput,
            SwEv::CgroupSwitches  => RawSwEv::CgroupSwitches,
        }),
        Ev::Raw(ev) => RawEv::Raw(unsafe { RawRawEv::new(ev.as_u64()) }),
        Ev::Tracepoint(ev) => RawEv::Tracepoint(RawTpEv::new(ev.id)),
        Ev::Breakpoint(ev) => RawEv::Breakpoint(RawBpEv::new(match &ev.bp_type {
            BpTy::R { addr, len } => RawBpTy::R {
                addr: *addr,
                len: into_raw_bp_len(&len),
            },
            BpTy::W { addr, len } => RawBpTy::W {
                addr: *addr,
                len: into_raw_bp_len(&len),
            },
            BpTy::Rw { addr, len } => RawBpTy::Rw {
                addr: *addr,
                len: into_raw_bp_len(&len),
            },
            BpTy::X { addr } => RawBpTy::X { addr: *addr },
        })),
        Ev::DynamicPmu(ev) => match ev {
            DpEv::Other { r#type, config } => RawEv::DynamicPmu(RawDpEv::Other {
                r#type: *r#type,
                config: *config,
            }),
            DpEv::Kprobe {
                r#type,
                retprobe,
                cfg,
            } => {
                let cfg = match cfg {
                    KpCfg::FuncAndOffset {
                        kprobe_func,
                        probe_offset,
                    } => RawKpCfg::FuncAndOffset {
                        kprobe_func: Rc::new(unsafe {
                            CString::from_vec_unchecked(kprobe_func.iter().cloned().collect())
                        }),
                        probe_offset: *probe_offset,
                    },
                    KpCfg::KprobeAddr(a) => RawKpCfg::KprobeAddr(*a),
                };
                RawEv::DynamicPmu(RawDpEv::Kprobe {
                    r#type: *r#type,
                    retprobe: *retprobe,
                    cfg,
                })
            }
            DpEv::Uprobe {
                r#type,
                retprobe,
                cfg,
            } => {
                let cfg = RawUpCfg {
                    uprobe_path: Rc::new(unsafe {
                        CString::from_vec_unchecked(cfg.uprobe_path.iter().cloned().collect())
                    }),
                    probe_offset: cfg.probe_offset,
                };
                RawEv::DynamicPmu(RawDpEv::Uprobe {
                    r#type: *r#type,
                    retprobe: *retprobe,
                    cfg,
                })
            }
        },
    };
    val
}