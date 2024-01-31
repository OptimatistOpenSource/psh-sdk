use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
pub enum Cpu {
    Any,
    Id(u32),
}

#[derive(Archive, Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
pub enum Process {
    Any,
    Current,
    Pid(u32),
}
