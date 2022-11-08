use std::cell::UnsafeCell;
use slotmap::SlotMap;
use crate::{Id, Entity};

#[derive(Default)]
pub struct Entities {
    inner:SlotMap<Id, UnsafeCell<Entity>>
}

pub struct Iter<'a> {
    iter:slotmap::basic::Iter<'a, Id, UnsafeCell<Entity>>
}

impl<'a> Iterator for Iter<'a> {
    type Item = (Id, &'a mut Entity);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((id, e)) = self.iter.next() {
            let e = unsafe { &mut *e.get() };
            return Some((id, e));
        }

        None
    }
}

impl Entities {
    pub fn spawn_entity(&mut self, entity:Entity) -> Id {
        self.inner.insert(UnsafeCell::new(entity))
    }

    pub fn despawn_entity(&mut self, id:Id) {
        self.inner.remove(id);
    }

    pub fn iter_mut(&self) -> Iter {
        Iter {
            iter:self.inner.iter()
        }
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn get(&self, id:Id) -> Option<&Entity> {
        if let Some(e) = self.inner.get(id) {
            return Some(unsafe {& *e.get()});
        }

        None
    }

    pub fn get_mut(&self, id:Id) -> Option<&mut Entity> {
        if let Some(e) = self.inner.get(id) {
            return Some(unsafe {&mut *e.get()});
        }

        None
    } 

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}