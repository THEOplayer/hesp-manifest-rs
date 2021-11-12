use std::collections::{hash_map, HashMap};
use std::hash::Hash;

use crate::*;

pub trait Entity {
    fn id(&self) -> &str;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EntityMap<E: Entity> {
    inner: HashMap<String, E>,
}

impl<E: Entity> EntityMap<E> {
    pub fn get(&self, id: &str) -> Option<&E> {
        self.inner.get(id)
    }
    pub fn get_mut(&mut self, id: &str) -> Option<&mut E> {
        self.inner.get_mut(id)
    }
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    pub fn iter(&self) -> EntityIter<E> {
        EntityIter::new(self.inner.values())
    }
    pub fn iter_mut(&mut self) -> EntityIterMut<E> {
        EntityIterMut::new(self.inner.values_mut())
    }
}

pub struct EntityIter<'a, E: Entity> {
    inner: hash_map::Values<'a, String, E>,
}

impl<'a, E: Entity> Iterator for EntityIter<'a, E> {
    type Item = &'a E;

    #[inline]
    fn next(&mut self) -> Option<&'a E> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, E: Entity> ExactSizeIterator for EntityIter<'a, E> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

pub struct EntityIterMut<'a, E: Entity> {
    inner: hash_map::ValuesMut<'a, String, E>,
}

impl<'a, E: Entity> Iterator for EntityIterMut<'a, E> {
    type Item = &'a mut E;

    #[inline]
    fn next(&mut self) -> Option<&'a mut E> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, E: Entity> ExactSizeIterator for EntityIterMut<'a, E> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

pub trait IntoEntities<E: Entity> {
    fn try_collect(self) -> Result<EntityMap<E>>;
}

impl<E: Entity, I: IntoIterator<Item = Result<E>>> IntoEntities<E> for I {
    fn try_collect(self) -> Result<EntityMap<E>> {
        let iter = self.into_iter();
        let mut map = HashMap::with_capacity(iter.size_hint().0);
        for entity in self {
            let entity = entity?;
            if let Some(duplicate) = map.insert(entity.id(), entity) {
                return Err(Error::DuplicateId(duplicate.id().to_string()));
            }
        }
        Ok(EntityMap { inner: map })
    }
}

impl<E: Entity> Default for EntityMap<E> {
    fn default() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
}
