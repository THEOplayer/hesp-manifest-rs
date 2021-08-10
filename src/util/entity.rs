use std::borrow::Borrow;
use std::fmt::Display;
use std::hash::Hash;
use std::ops::Deref;

use itertools::Itertools;
use serde::{Deserialize, Deserializer, Serialize};
use std::vec::IntoIter;
use crate::*;

pub trait Entity {
    type Id: Hash + Eq + Display + ?Sized;
    fn id(&self) -> &Self::Id;
}

#[derive(Debug, Clone, Serialize)]
pub struct EntityVec<E: Entity>(Vec<E>);

impl<E: Entity> EntityVec<E> {
    pub fn new(vec: Vec<E>) -> Result<Self> {
        let duplicates = vec.iter().map(E::id).duplicates().join(", ");
        if !duplicates.is_empty() {
            Err(Error::DuplicateIds(duplicates))
        } else {
            Ok(Self(vec))
        }
    }
}

impl<'de, E> Deserialize<'de> for EntityVec<E>
    where E: Entity + Deserialize<'de>
{
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let vec = Vec::deserialize(deserializer)?;
        Self::new(vec).map_err(serde::de::Error::custom)
    }
}

impl<E: Entity> Deref for EntityVec<E> {
    type Target = [E];
    fn deref(&self) -> &[E] { &self.0 }
}

impl<E: Entity> Borrow<[E]> for EntityVec<E> {
    fn borrow(&self) -> &[E] { &self.0 }
}

impl<E: Entity> Default for EntityVec<E> {
    fn default() -> Self { Self(Vec::new()) }
}

impl<E: Entity> IntoIterator for EntityVec<E> {
    type Item = E;
    type IntoIter = IntoIter<E>;
    fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}