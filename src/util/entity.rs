use std::collections::{hash_map, HashMap};

use crate::{Error, Result};

pub trait Entity {
    fn id(&self) -> &str;
}

#[derive(Debug, Clone, PartialEq)]
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
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    pub fn iter(&self) -> EntityIter<E> {
        EntityIter {
            inner: self.inner.values(),
        }
    }
    pub fn iter_mut(&mut self) -> EntityIterMut<E> {
        EntityIterMut {
            inner: self.inner.values_mut(),
        }
    }
}

impl<E: Entity> IntoIterator for EntityMap<E> {
    type Item = E;
    type IntoIter = hash_map::IntoValues<String, E>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_values()
    }
}

impl<'a, E: Entity> IntoIterator for &'a EntityMap<E> {
    type Item = &'a E;
    type IntoIter = EntityIter<'a, E>;

    #[inline]
    fn into_iter(self) -> EntityIter<'a, E> {
        self.iter()
    }
}

impl<'a, E: Entity> IntoIterator for &'a mut EntityMap<E> {
    type Item = &'a mut E;
    type IntoIter = EntityIterMut<'a, E>;

    #[inline]
    fn into_iter(self) -> EntityIterMut<'a, E> {
        self.iter_mut()
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

pub trait FromEntities<E: Entity> {
    fn into_entities(self) -> Result<EntityMap<E>>;
}

impl<E: Entity, I: IntoIterator<Item = Result<E>>> FromEntities<E> for I {
    fn into_entities(self) -> Result<EntityMap<E>> {
        let iter = self.into_iter();
        let mut map = HashMap::with_capacity(iter.size_hint().0);
        for entity in iter {
            let entity = entity?;
            if let Some(duplicate) = map.insert(entity.id().to_owned(), entity) {
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
