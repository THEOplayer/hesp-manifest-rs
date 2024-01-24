use std::collections::{hash_map, HashMap};
use std::vec;

use crate::{Error, Result};

pub trait Entity {
    fn id(&self) -> &str;
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EntityMap<E: Entity> {
    inner: HashMap<String, E>,
    keys: Vec<String>,
}

impl<E: Entity> EntityMap<E> {
    #[must_use]
    pub fn get(&self, id: &str) -> Option<&E> {
        self.inner.get(id)
    }

    #[must_use]
    pub fn get_mut(&mut self, id: &str) -> Option<&mut E> {
        self.inner.get_mut(id)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    #[must_use]
    pub fn iter(&self) -> EntityIter<E> {
        EntityIter {
            inner: self.inner.values(),
        }
    }

    #[must_use]
    pub fn iter_mut(&mut self) -> EntityIterMut<E> {
        EntityIterMut {
            inner: self.inner.values_mut(),
        }
    }
}

pub struct OrderedEntityIter<E: Entity> {
    map: HashMap<String, E>,
    key_iter: vec::IntoIter<String>,
}

impl<E: Entity> Iterator for OrderedEntityIter<E> {
    type Item = E;

    #[inline]
    fn next(&mut self) -> Option<E> {
        self.key_iter
            .next()
            .map(|key| self.map.remove_entry(&key).unwrap())
            .map(|(_, entity)| entity)
    }
}

impl<E: Entity> IntoIterator for EntityMap<E> {
    type Item = E;
    type IntoIter = OrderedEntityIter<E>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        OrderedEntityIter {
            map: self.inner,
            key_iter: self.keys.into_iter(),
        }
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

// this could be replaced by `impl<E: Entity, I: IntoIterator<Item = Result<E>>> TryFrom<I> for EntityMap<E>`
// but this does not work because of https://github.com/rust-lang/rust/issues/50133
pub trait FromEntities<E: Entity> {
    fn into_entities(self) -> Result<EntityMap<E>>;
}

impl<E: Entity, I: IntoIterator<Item = Result<E>>> FromEntities<E> for I {
    fn into_entities(self) -> Result<EntityMap<E>> {
        let iter = self.into_iter();
        let size_hint = iter.size_hint().0;
        let mut inner = HashMap::with_capacity(size_hint);
        let mut keys = Vec::with_capacity(size_hint);
        for entity in iter {
            let entity = entity?;
            keys.push(entity.id().to_owned());
            if let Some(duplicate) = inner.insert(entity.id().to_owned(), entity) {
                return Err(Error::DuplicateId(duplicate.id().to_string()));
            }
        }
        Ok(EntityMap { inner, keys })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn entity_map_retains_order() -> Result<()> {
        let vec = vec!["t", "h", "e", "o", " ", "r", "u", "l", "z"]
            .into_iter()
            .map(DummyEntity);
        let map: EntityMap<DummyEntity> = vec.into_iter().map(Result::Ok).into_entities()?;

        let mut iter = map.into_iter().map(|e| e.0);
        assert_eq!(iter.next(), Some("t"));
        assert_eq!(iter.next(), Some("h"));
        assert_eq!(iter.next(), Some("e"));
        assert_eq!(iter.next(), Some("o"));
        assert_eq!(iter.next(), Some(" "));
        assert_eq!(iter.next(), Some("r"));
        assert_eq!(iter.next(), Some("u"));
        assert_eq!(iter.next(), Some("l"));
        assert_eq!(iter.next(), Some("z"));
        assert_eq!(iter.next(), None);
        Ok(())
    }

    struct DummyEntity(&'static str);
    impl Entity for DummyEntity {
        fn id(&self) -> &str {
            self.0
        }
    }
}
