use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use serde;

use crate::*;

pub trait Entity {
    type Id: Hash + Eq + Display + ?Sized;
    fn id(&self) -> &Self::Id;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[serde(try_from = "Vec<E>")]
pub struct EntityMap<E: Entity>(HashMap<E::Id, E>);

impl<E: Entity> EntityMap<E> {
    pub fn get(&self, id: &E::Id) -> Option<&E> {
        self.0.get(id)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

}

trait IntoEntities<E: Entity> {
    fn try_collect(self) -> Result<EntityMap<E>>;
}

impl <E:Entity, I:IntoIterator<Item=E>> IntoEntities<E> for I {
    fn try_collect(self) -> Result<EntityMap<E>> {
        let iter = self.into_iter();
        let mut map = HashMap::with_capacity(iter.size_hint().0);
        for entity in self {
            if let Some(duplicate) = map.insert(entity.id(), entity) {
                return Err(Error::DuplicateId(duplicate.id().to_string()))
            }
        }
        Ok(EntityMap(map))
    }
}

impl<E: Entity> Default for EntityMap<E> {
    fn default() -> Self {
        Self(HashMap::new())
    }
}