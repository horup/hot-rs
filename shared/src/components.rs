use serde::{Serialize, Deserialize, de::DeserializeOwned};
use slotmap::{SlotMap, SecondaryMap};
use crate::{Id, Entity, CSDUnsafeCell};


#[derive(Default, Serialize, Clone)]
pub struct Components<T : Copy + Clone> {
    inner:SecondaryMap<Id, CSDUnsafeCell<T>>
}

type E<T> = SecondaryMap<Id, CSDUnsafeCell<T>>;

impl<'de, T : Copy + Clone + Serialize + Deserialize<'de>> Deserialize<'de> for Components<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
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

pub struct IterMut<'a, T : Copy + Clone> {
    iter:slotmap::secondary::Iter<'a, Id, CSDUnsafeCell<T>>
}

impl<'a, T : Copy + Clone> Iterator for IterMut<'a, T> {
    type Item = (Id, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((id, e)) = self.iter.next() {
            let e = unsafe { &mut *e.get() };
            return Some((id, e));
        }

        None
    }
}

pub struct Iter<'a, T : Copy + Clone> {
    iter:slotmap::secondary::Iter<'a, Id, CSDUnsafeCell<T>>
}

impl<'a, T : Serialize + DeserializeOwned + Copy + Clone> Iterator for Iter<'a, T> {
    type Item = (Id, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((id, e)) = self.iter.next() {
            let e = unsafe { & *e.get() };
            return Some((id, e));
        }

        None
    }
}


impl<T : Copy + Clone> Components<T> {
    pub fn attach(&mut self, id:Id, cmp:T) {
        self.inner.insert(id, CSDUnsafeCell::new(cmp));
    }

    pub fn detach(&mut self, id:Id) {
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