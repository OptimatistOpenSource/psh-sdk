use crate::infra::IdMap;
use std::any::Any;
use std::collections::VecDeque;

pub type Logs = VecDeque<String>;

#[derive(Debug)]
pub struct Data {
    resource_map: IdMap<Box<dyn Any>>,
}

impl Default for Data {
    fn default() -> Self {
        Self::new()
    }
}

impl Data {
    pub fn new() -> Self {
        let mut resource_map = IdMap::new();

        // 0 -> output log
        let output_log: Box<dyn Any> = Box::<Logs>::default();
        resource_map.insert(output_log);
        // 1 -> error log
        let error_log: Box<dyn Any> = Box::<Logs>::default();
        resource_map.insert(error_log);

        Self { resource_map }
    }

    pub fn add_resource<T: 'static>(&mut self, resource: T) -> u32 {
        let resource: Box<dyn Any> = Box::new(resource);
        self.resource_map.insert(resource)
    }

    pub fn drop_resource(&mut self, rid: u32) -> bool {
        match rid {
            0 | 1 => false, // TODO: logs can not be dropped
            _ => self.resource_map.remove(rid).is_some(),
        }
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
}

impl Data {
    pub fn output_log(&self) -> &Logs {
        self.get_resource::<Logs>(0).unwrap()
    }

    pub fn output_log_mut(&mut self) -> &mut Logs {
        self.get_resource_mut::<Logs>(0).unwrap()
    }

    pub fn error_log(&self) -> &Logs {
        self.get_resource::<Logs>(1).unwrap()
    }

    pub fn error_log_mut(&mut self) -> &mut Logs {
        self.get_resource_mut::<Logs>(1).unwrap()
    }
}
