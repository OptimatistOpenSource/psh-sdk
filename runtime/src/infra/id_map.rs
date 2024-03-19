use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::{Debug, Formatter};
use std::ops::Not;

// TODO: can be optimized
pub struct IdMap<T> {
    ids: BTreeSet<u32>,
    map: HashMap<u32, T>,
}

impl<T> Debug for IdMap<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut map = BTreeMap::<u32, Option<&T>>::new();
        self.map
            .iter()
            .map(|(k, v)| (k, Some(v)))
            .chain(self.ids.iter().map(|id| (id, None)))
            .for_each(|(k, v)| {
                if map.contains_key(k).not() {
                    map.insert(*k, v);
                }
            });
        f.debug_map().entries(map.iter()).finish()
    }
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

    pub fn consume_id(&mut self, id: u32) {
        self.ids.insert(id);
    }

    pub fn produce_id(&mut self, id: u32) {
        self.ids.remove(&id);
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
