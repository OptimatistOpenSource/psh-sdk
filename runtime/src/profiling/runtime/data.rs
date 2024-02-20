use crate::infra::IdMap;
use std::any::Any;
use std::fmt::{Debug, Formatter};

pub struct Data {
    out: Box<dyn FnMut(&str)>,
    err: Box<dyn FnMut(&str)>,
    resource_map: IdMap<Box<dyn Any>>,
}

impl Data {
    pub fn new<O, E>(out: O, err: E) -> Self
    where
        O: FnMut(&str) + 'static,
        E: FnMut(&str) + 'static,
    {
        Self {
            out: Box::new(out),
            err: Box::new(err),
            resource_map: IdMap::new(),
        }
    }

    pub fn add_resource<T: 'static>(&mut self, resource: T) -> u32 {
        let resource: Box<dyn Any> = Box::new(resource);
        self.resource_map.insert(resource)
    }

    pub fn drop_resource(&mut self, rid: u32) -> bool {
        self.resource_map.remove(rid).is_some()
    }

    pub fn get_resource<T: 'static>(&self, rid: u32) -> Option<&T> {
        self.resource_map
            .get(rid)
            .map(|it| it.as_ref())
            .and_then(|it| it.downcast_ref::<T>())
    }

    pub fn get_resource_mut<T: 'static>(&mut self, rid: u32) -> Option<&mut T> {
        self.resource_map
            .get_mut(rid)
            .map(|it| it.as_mut())
            .and_then(|it| it.downcast_mut::<T>())
    }

    pub fn take_resource<T: 'static>(&mut self, rid: u32) -> Option<Box<T>> {
        self.resource_map
            .remove(rid)
            .and_then(|it| it.downcast::<T>().ok())
    }

    pub fn out(&mut self) -> &mut dyn FnMut(&str) {
        self.out.as_mut()
    }

    pub fn err(&mut self) -> &mut dyn FnMut(&str) {
        self.err.as_mut()
    }
}

impl Debug for Data {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Data")
            .field("out", &self.out.type_id())
            .field("err", &self.err.type_id())
            .field("resource_map", &self.resource_map)
            .finish()
    }
}
