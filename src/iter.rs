//! `World` iterators

use crate::{Entity, TryMut, TryRef};

/// An index wrapper that designates an immutable reference
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Ref<T>(pub T);

/// An index wrapper that designates a mutable reference
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Mut<T>(pub T);

/// An index wrapper that designates a reference type
pub trait Index<'a, C> {
    /// The reference to the component's value
    type Reference;
    /// Try to get a reference from an `Entity`
    fn try_entity(self, entity: &'a Entity<C>) -> Option<Self::Reference>;
}

impl<'a, T, C> Index<'a, C> for Ref<T>
where
    Entity<C>: TryRef<T>,
    <Entity<C> as TryRef<T>>::Output: 'a,
{
    type Reference = &'a <Entity<C> as TryRef<T>>::Output;
    fn try_entity(self, entity: &'a Entity<C>) -> Option<Self::Reference> {
        entity.try_ref(self.0)
    }
}

impl<'a, T, C> Index<'a, C> for Mut<T>
where
    Entity<C>: TryMut<T>,
    <Entity<C> as TryRef<T>>::Output: 'a,
{
    type Reference = &'a mut <Entity<C> as TryRef<T>>::Output;
    fn try_entity(self, entity: &'a Entity<C>) -> Option<Self::Reference> {
        unsafe {
            (entity as *const Entity<C> as *mut Entity<C>)
                .as_mut()
                .unwrap()
        }
        .try_mut(self.0)
    }
}

#[macro_export]
macro_rules! entity_as {
    ($entity:expr, $index:expr) => {
        $index.try_entity(&$entity)
    };
    ($entity:expr, $($index:expr),*) => {
        if $($index.try_entity(&$entity).is_some() &&)* true {
            Some(($($index.try_entity(&$entity).unwrap()),*))
        } else {
            None
        }
    };
}

#[macro_export]
macro_rules! map {
    ($index:expr) => {
        |entity| entity_as!(entity, $index)
    };
    ($($index:expr),*) => {
        |entity| entity_as!(entity, $($index),*)
    };
}
