use serde::{Serialize, Deserialize};
use slotmap::SlotMap;
use crate::{Id, Sprite, CSDUnsafeCell};


#[derive(Default, Serialize, Clone)]
pub struct Sprites {
    inner:SlotMap<Id, CSDUnsafeCell<Sprite>>
}

type E = SlotMap<Id, CSDUnsafeCell<Sprite>>;

impl<'de> Deserialize<'de> for Sprites {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        match E::deserialize(deserializer) {
            Ok(inner) => {
                Ok(Sprites {
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
    iter:slotmap::basic::Iter<'a, Id, CSDUnsafeCell<Sprite>>
}

impl<'a> Iterator for IterMut<'a> {
    type Item = (Id, &'a mut Sprite);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((id, e)) = self.iter.next() {
            let e = unsafe { &mut *e.get() };
            return Some((id, e));
        }

        None
    }
}

pub struct Iter<'a> {
    iter:slotmap::basic::Iter<'a, Id, CSDUnsafeCell<Sprite>>
}

impl<'a> Iterator for Iter<'a> {
    type Item = (Id, &'a Sprite);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((id, e)) = self.iter.next() {
            let e = unsafe { & *e.get() };
            return Some((id, e));
        }

        None
    }
}


impl Sprites {
    pub fn spawn(&mut self, sprite:Sprite) -> Id {
        self.inner.insert(CSDUnsafeCell::new(sprite))
    }

    pub fn despawn(&mut self, id:Id) {
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

    pub fn get(&self, id:Id) -> Option<&Sprite> {
        if let Some(e) = self.inner.get(id) {
            return Some(unsafe {& *e.get()});
        }

        None
    }

    pub fn get_mut(&self, id:Id) -> Option<&mut Sprite> {
        if let Some(e) = self.inner.get(id) {
            return Some(unsafe {&mut *e.get()});
        }

        None
    } 

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}