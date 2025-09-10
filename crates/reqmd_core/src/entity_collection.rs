use crate::{Entity, Id};
use std::collections::HashMap;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct EntityCollection<T: Entity>(HashMap<Id, T>);

impl<T: Entity> EntityCollection<T> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert(&mut self, entity: T) {
        self.0.insert(entity.id(), entity);
    }

    pub fn get(&self, id: &Id) -> Option<&T> {
        self.0.get(id)
    }

    pub fn remove(&mut self, id: &Id) -> Option<T> {
        self.0.remove(id)
    }

    pub fn contains(&self, id: &Id) -> bool {
        self.0.contains_key(id)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.values()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.0.values_mut()
    }
}

impl<I, T> FromIterator<I> for EntityCollection<T>
where
    I: IntoIterator<Item = T>,
    T: Entity,
{
    fn from_iter<U: IntoIterator<Item = I>>(iter: U) -> Self {
        let mut collection = EntityCollection::new();
        for item in iter {
            for entity in item {
                collection.insert(entity);
            }
        }
        collection
    }
}

impl<T: Entity> IntoIterator for EntityCollection<T> {
    type Item = T;
    type IntoIter = std::collections::hash_map::IntoValues<Id, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_values()
    }
}

impl<'a, T: Entity> IntoIterator for &'a EntityCollection<T> {
    type Item = &'a T;
    type IntoIter = std::collections::hash_map::Values<'a, Id, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.values()
    }
}

impl<'a, T: Entity> IntoIterator for &'a mut EntityCollection<T> {
    type Item = &'a mut T;
    type IntoIter = std::collections::hash_map::ValuesMut<'a, Id, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.values_mut()
    }
}

impl<T: Entity> Default for EntityCollection<T> {
    fn default() -> Self {
        Self::new()
    }
}
