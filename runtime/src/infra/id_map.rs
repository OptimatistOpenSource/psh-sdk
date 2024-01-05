use std::collections::{BTreeSet, HashMap};

// TODO: can be optimized
#[derive(Debug)]
pub struct IdMap<T> {
    ids: BTreeSet<u32>,
    map: HashMap<u32, T>,
}

impl<T> IdMap<T> {
    pub fn new() -> Self {
        Self {
            ids: BTreeSet::new(),
            map: HashMap::new(),
        }
    }
    pub fn as_hash_map(&self) -> &HashMap<u32, T> {
        &self.map
    }

    pub fn insert(&mut self, val: T) -> u32 {
        let id = match self.ids.iter().next_back() {
            None => 0,
            Some(max_id) => max_id + 1,
        };
        self.ids.insert(id);
        self.map.insert(id, val);
        id
    }

    pub fn get(&self, id: u32) -> Option<&T> {
        self.map.get(&id)
    }

    pub fn get_mut(&mut self, id: u32) -> Option<&mut T> {
        self.map.get_mut(&id)
    }

    pub fn remove(&mut self, id: u32) -> Option<T> {
        let val = self.map.remove(&id);
        self.ids.remove(&id);
        val
    }
}
