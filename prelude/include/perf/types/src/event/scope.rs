use alloc::vec;
use alloc::vec::Vec;
use core::ops::Not;
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
pub enum EventScope {
    User,
    Kernel,
    Hv,
    Idle,
    Host,
    Guest,
}

impl EventScope {
    pub fn all() -> Vec<Self> {
        vec![
            Self::User,
            Self::Kernel,
            Self::Hv,
            Self::Idle,
            Self::Host,
            Self::Guest,
        ]
    }

    pub fn all_but_exclude<'t>(scopes: impl IntoIterator<Item = &'t Self>) -> Vec<Self> {
        let excludes = scopes.into_iter().collect::<Vec<_>>();
        Self::all()
            .iter()
            .filter(|s| excludes.contains(s).not())
            .cloned()
            .collect()
    }
}
