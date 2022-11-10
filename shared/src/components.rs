use serde::{Serialize, Deserialize};
use slotmap::SlotMap;
use crate::{Id, Entity, CSDUnsafeCell};


#[derive(Default, Serialize, Clone)]
pub struct Components<T : Copy + Clone + Serialize + Deserialize<'static>> {
    inner:SlotMap<Id, CSDUnsafeCell<T>>
}

type E<T> = SlotMap<Id, CSDUnsafeCell<T>>;

impl<T : Copy + Clone + Serialize + Deserialize<'static>> Deserialize<'static> for Components<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'static> {
        match E::deserialize(deserializer) {
            Ok(inner) => {
                return Ok(Components {
                    inner
                });
            },
            Err(err) => {
                return Err(err);
            },
        }
    }
}

pub struct IterMut<'a, T : Serialize + Deserialize<'static> + Copy + Clone> {
    iter:slotmap::basic::Iter<'a, Id, CSDUnsafeCell<T>>
}

impl<'a, T : Serialize + Deserialize<'static> + Copy + Clone> Iterator for IterMut<'a, T> {
    type Item = (Id, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((id, e)) = self.iter.next() {
            let e = unsafe { &mut *e.get() };
            return Some((id, e));
        }

        None
    }
}

pub struct Iter<'a, T : Serialize + Deserialize<'static> + Copy + Clone> {
    iter:slotmap::basic::Iter<'a, Id, CSDUnsafeCell<T>>
}

impl<'a, T : Serialize + Deserialize<'static> + Copy + Clone> Iterator for Iter<'a, T> {
    type Item = (Id, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((id, e)) = self.iter.next() {
            let e = unsafe { & *e.get() };
            return Some((id, e));
        }

        None
    }
}


impl<T : Copy + Clone + Serialize + Deserialize<'static>> Components<T> {
    pub fn spawn_entity(&mut self, entity:T) -> Id {
        self.inner.insert(CSDUnsafeCell::new(entity))
    }

    pub fn despawn_entity(&mut self, id:Id) {
        self.inner.remove(id);
    }

    pub fn iter_mut(&self) -> IterMut<T> {
        IterMut {
            iter:self.inner.iter()
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            iter:self.inner.iter()
        }
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn get(&self, id:Id) -> Option<&T> {
        if let Some(e) = self.inner.get(id) {
            return Some(unsafe {& *e.get()});
        }

        None
    }

    pub fn get_mut(&self, id:Id) -> Option<&mut T> {
        if let Some(e) = self.inner.get(id) {
            return Some(unsafe {&mut *e.get()});
        }

        None
    } 

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}