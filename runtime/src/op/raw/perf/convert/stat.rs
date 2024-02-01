use crate::op::raw::perf::convert::Wrap;

type FromT = perf_event_rs::counting::CounterStat;
type IntoT = profiling_prelude_perf_types::counting::CounterStat;

impl From<&FromT> for Wrap<IntoT> {
    fn from(value: &FromT) -> Self {
        #[rustfmt::skip]
        let val = IntoT {
            event_id:     value.event_id,
            event_count:  value.event_count,
            time_enabled: value.time_enabled,
            time_running: value.time_running,
        };
        Self(val)
    }
}
