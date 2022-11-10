use std::cell::UnsafeCell;
use slotmap::{SecondaryMap};
use crate::{Id, Entity};

#[derive(Default)]
pub struct Components<T> where T:Copy {
    inner:SecondaryMap<Id, UnsafeCell<T>>
}

pub struct IterMut<'a> {
    iter:slotmap::basic::Iter<'a, Id, UnsafeCell<Entity>>
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
    iter:slotmap::secondary::Iter<'a, Id, UnsafeCell<Entity>>
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


impl<T> Components<T> where T:Copy {
    pub fn insert(&mut self, id:Id, t:T) {
        self.inner.insert(id, UnsafeCell::new(t));
    }

    pub fn remove(&mut self, id:Id) {
        self.inner.remove(id);
    }

   /* pub fn iter_mut(&self) -> IterMut {
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
    }*/
}