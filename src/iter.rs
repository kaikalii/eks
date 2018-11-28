//! `World` iterators

use crate::{Entity, TryMut, TryRef};

/// An index wrapper that designates an immutable reference
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Ref<T>(pub T);

/// An index wrapper that designates a mutable reference
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Mut<T>(pub T);

/// An index wrapper for only immutable references
pub trait Index<C> {
    /// The type of the component's value
    type Output;
    /// Try to get a reference from an `Entity`
    fn try_entity(self, entity: &Entity<C>) -> Option<&Self::Output>;
}

impl<T, C> Index<C> for Ref<T>
where
    Entity<C>: TryRef<T>,
{
    type Output = <Entity<C> as TryRef<T>>::Output;
    fn try_entity(self, entity: &Entity<C>) -> Option<&Self::Output> {
        entity.try_ref(self.0)
    }
}

/// An index wrapper that designates a reference type
pub trait IndexMut<'a, C> {
    /// The reference to the component's value
    type Reference;
    /// Try to get a reference from an `Entity`
    fn try_entity_mut(self, entity: &'a Entity<C>) -> Option<Self::Reference>;
}

impl<'a, T, C> IndexMut<'a, C> for Ref<T>
where
    Entity<C>: TryRef<T>,
    <Entity<C> as TryRef<T>>::Output: 'a,
{
    type Reference = &'a <Entity<C> as TryRef<T>>::Output;
    fn try_entity_mut(self, entity: &'a Entity<C>) -> Option<Self::Reference> {
        entity.try_ref(self.0)
    }
}

impl<'a, T, C> IndexMut<'a, C> for Mut<T>
where
    Entity<C>: TryMut<T>,
    <Entity<C> as TryRef<T>>::Output: 'a,
{
    type Reference = &'a mut <Entity<C> as TryRef<T>>::Output;
    fn try_entity_mut(self, entity: &'a Entity<C>) -> Option<Self::Reference> {
        unsafe {
            (entity as *const Entity<C> as *mut Entity<C>)
                .as_mut()
                .unwrap()
        }
        .try_mut(self.0)
    }
}

#[macro_export]
macro_rules! map {
    ($index:expr) => {
        |entity: & Entity<_>| $index.try_entity(entity)
    };
    ($($index:expr),*) => {
        |entity: & Entity<_>| if $($index.try_entity(entity).is_some() &&)* true {
            Some(($($index.try_entity(entity).unwrap()),*))
        } else {
            None
        }
    };
}

#[macro_export]
macro_rules! map_mut {
    ($index:expr) => {
        |entity: &mut Entity<_>| $index.try_entity_mut(entity)
    };
    ($($index:expr),*) => {
        |entity: &mut Entity<_>| if $($index.try_entity_mut(entity).is_some() &&)* true {
            Some(($($index.try_entity_mut(entity).unwrap()),*))
        } else {
            None
        }
    };
}
