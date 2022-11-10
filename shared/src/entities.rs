use serde::{Serialize, Deserialize};
use slotmap::SlotMap;
use crate::{Id, Entity, CSDUnsafeCell};


#[derive(Default, Serialize, Clone)]
pub struct Entities {
    inner:SlotMap<Id, CSDUnsafeCell<Entity>>
}

type E = SlotMap<Id, CSDUnsafeCell<Entity>>;

impl<'de> Deserialize<'de> for Entities {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        match E::deserialize(deserializer) {
            Ok(inner) => {
                Ok(Entities {
                    inner
                })
            },
            Err(err) => {
                Err(err)
            },
        }
    }
}

pub struct IterMut<'a> {
    iter:slotmap::basic::Iter<'a, Id, CSDUnsafeCell<Entity>>
}

impl<'a> Iterator for IterMut<'a> {
    type Item = (Id, &'a mut Entity);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((id, e)) = self.iter.next() {
            let e = unsafe { &mut *e.get() };
            return Some((id, e));
        }

        None
    }
}

pub struct Iter<'a> {
    iter:slotmap::basic::Iter<'a, Id, CSDUnsafeCell<Entity>>
}

impl<'a> Iterator for Iter<'a> {
    type Item = (Id, &'a Entity);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((id, e)) = self.iter.next() {
            let e = unsafe { & *e.get() };
            return Some((id, e));
        }

        None
    }
}


impl Entities {
    pub fn spawn_entity(&mut self, entity:Entity) -> Id {
        self.inner.insert(CSDUnsafeCell::new(entity))
    }

    pub fn despawn_entity(&mut self, id:Id) {
        self.inner.remove(id);
    }

    pub fn iter_mut(&self) -> IterMut {
        IterMut {
            iter:self.inner.iter()
        }
    }

    pub fn iter(&self) -> Iter {
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