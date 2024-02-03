use crate::op::perf::convert::Wrap;

type FromT = profiling_prelude_perf_types::event::EventScope;
type IntoT = perf_event_rs::event::EventScope;

impl From<&FromT> for Wrap<IntoT> {
    fn from(value: &FromT) -> Self {
        #[rustfmt::skip]
        let val = match value {
            FromT::User            => IntoT::User,
            FromT::Kernel          => IntoT::Kernel,
            FromT::Hv              => IntoT::Hv,
            FromT::Idle            => IntoT::Idle,
            FromT::Host            => IntoT::Host,
            FromT::Guest           => IntoT::Guest,
        };
        Self(val)
    }
}
