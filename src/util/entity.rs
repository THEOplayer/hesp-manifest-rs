use std::borrow::{Borrow, BorrowMut};
use std::convert::TryFrom;
use std::fmt::Display;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};
use std::vec::IntoIter;

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::*;
use std::slice;

pub trait Entity {
    type Id: Hash + Eq + Display + ?Sized;
    fn id(&self) -> &Self::Id;
}

//TODO use HashMap instead
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "Vec<E>")]
pub struct EntityVec<E: Entity>(pub(crate) Vec<E>);

impl<E: Entity> EntityVec<E> {
    pub fn get(&self, id: &E::Id) -> Option<&E> {
        self.iter().find(|entity| entity.id() == id)
    }
}

impl<E: Entity> TryFrom<Vec<E>> for EntityVec<E> {
    type Error = Error;
    fn try_from(vec: Vec<E>) -> Result<Self> {
        let duplicates = vec.iter().map(E::id).duplicates().join(", ");
        if !duplicates.is_empty() {
            Err(Error::DuplicateIds(duplicates))
        } else {
            Ok(Self(vec))
        }
    }
}

impl<E: Entity> Deref for EntityVec<E> {
    type Target = [E];
    fn deref(&self) -> &[E] {
        &self.0
    }
}

impl<E: Entity> DerefMut for EntityVec<E> {
    fn deref_mut(&mut self) -> &mut [E] {
        &mut self.0
    }
}

impl<E: Entity> Borrow<[E]> for EntityVec<E> {
    fn borrow(&self) -> &[E] {
        &self[..]
    }
}

impl<E: Entity> BorrowMut<[E]> for EntityVec<E> {
    fn borrow_mut(&mut self) -> &mut [E] {
        &mut self[..]
    }
}

impl<E: Entity> Default for EntityVec<E> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<E: Entity> IntoIterator for EntityVec<E> {
    type Item = E;
    type IntoIter = IntoIter<E>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, E: Entity> IntoIterator for &'a EntityVec<E> {
    type Item = &'a E;
    type IntoIter = slice::Iter<'a, E>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, E: Entity> IntoIterator for &'a mut EntityVec<E> {
    type Item = &'a mut E;
    type IntoIter = slice::IterMut<'a, E>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<E: Entity> AsMut<[E]> for EntityVec<E> {
    fn as_mut(&mut self) -> &mut [E] {
        self
    }
}

impl<E: Entity> AsRef<[E]> for EntityVec<E> {
    fn as_ref(&self) -> &[E] {
        self
    }
}
