//! `World` iterators

use crate::{Entity, TryMut, TryRef};

/// An index wrapper for only immutable references
pub trait Index<C> {
    /// The type of the component's value
    type Output;
    /// Try to get a reference from an `Entity`
    fn try_entity(entity: &Entity<C>) -> Option<&Self::Output>;
}

impl<T, C> Index<C> for T
where
    T: Default,
    Entity<C>: TryRef<T>,
{
    type Output = <Entity<C> as TryRef<T>>::Output;
    fn try_entity(entity: &Entity<C>) -> Option<&Self::Output> {
        entity.get::<T>()
    }
}

/// An index wrapper that designates a reference type
pub trait IndexMut<'a, C> {
    /// The reference to the component's value
    type Reference;
    /// Try to get a reference from an `Entity`
    fn try_entity_mut(entity: &'a Entity<C>) -> Option<Self::Reference>;
}

impl<'a, T, C> IndexMut<'a, C> for T
where
    T: Default,
    Entity<C>: TryMut<T>,
    <Entity<C> as TryRef<T>>::Output: 'a,
{
    type Reference = &'a mut <Entity<C> as TryRef<T>>::Output;
    fn try_entity_mut(entity: &'a Entity<C>) -> Option<Self::Reference> {
        unsafe { (entity as *const Entity<C> as *mut Entity<C>).as_mut() }
            .unwrap()
            .get_mut::<T>()
    }
}

/**
Macro for immutably accessing components

This macro has two syntaxes:

# Iterator syntax `map!(C1, C2, ... in WORLD)`

Creates an iterator over the given `World` where the elements
are tuples of immutable references to the specified components
from `Entity`s that have them.

# Closure syntax `map!(C1, C2, ...)`

Creates a closure that takes an `&Entity` and returns an
optional tuple of immutable references to the specified components
if the `Entity` has all of them.

# Note

If only one component is specified, the iterator element / optional
return value will not be a tuple.
*/
#[macro_export]
macro_rules! map {
    ($($name:ident),* in $world:expr) => {
        $world.iter().filter_map(map!($($name),*))
    };
    ($($name:ident),*) => {
        |entity| if $($name::try_entity(entity).is_some() &&)* true {
            Some(($($name::try_entity(entity).unwrap()),*))
        } else {
            None
        }
    };
}

/**
Macro for mutably accessing components

This macro has two syntaxes:

# Iterator syntax `map_mut!(C1, C2, ... in WORLD)`

Creates an iterator over the given `World` where the elements
are tuples of mutable references to the specified components
from `Entity`s that have them.

# Closure syntax `map_mut!(C1, C2, ...)`

Creates a closure that takes an `&Entity` and returns an
optional tuple of mutable references to the specified components
if the `Entity` has all of them.

# Note

If only one component is specified, the iterator element / optional
return value will not be a tuple.

# Warning

It is considered undefined behavior to specify multiples
of the same component, i.e. `map_mut!(Foo, Foo)`.
While this violates Rust's borrowing rules, it will still
compile and run for reasons having to do with performance. If
you want runtime checks that no two components are the same, use
`map_mut_checked!`.
*/
#[macro_export]
macro_rules! map_mut {
    ($name:ident) => {
        |entity| $name::try_entity_mut(entity)
    };
    ($($name:ident),* in $world:expr) => {
        $world.iter_mut().filter_map(map_mut!($($name),*))
    };
    ($($name:ident),*) => {
        |entity| if $($name::try_entity_mut(entity).is_some() &&)* true {
            Some(($($name::try_entity_mut(entity).unwrap()),*))
        } else {
            None
        }
    };
}

/**
Macro for mutably accessing components

This macro has two syntaxes:

# Iterator syntax `map_mut_checked!(C1, C2, ... in WORLD)`

Creates an iterator over the given `World` where the elements
are tuples of mutable references to the specified components
from `Entity`s that have them.

# Closure syntax `map_mut_checked!(C1, C2, ...)`

Creates a closure that takes an `&Entity` and returns an
optional tuple of mutable references to the specified components
if the `Entity` has all of them.

# Notes

If only one component is specified, the iterator element / optional
return value will not be a tuple.

Because the generated closure performs a uniqueness check,
it will likely be considerably slower than one generated by `map_mut!`.

# Panics

Panics if any two specified components are the same,
i.e. `map_mut_checked!(Foo, Foo)`.
*/
#[macro_export]
macro_rules! map_mut_checked {
    ($($name:ident),* in $world:expr) => {
        $world.iter_mut().filter_map(map_mut_checked!($($name),*))
    };
    ($($name:ident),*) => {
        |entity| {
            use std::collections::HashSet;
            let mut used: HashSet<String> = HashSet::new();
            $(
                let s = format!("{:?}", $name::default());
                if !used.contains(&s) {
                    used.insert(s);
                } else {
                    panic!("{:?} is used twice in `map_mut_checked` in {} on line {}:{}", s, file!(), line!(), column!());
                }
            )*
            if $($name::try_entity_mut(entity).is_some() &&)* true {
                Some(($($name::try_entity_mut(entity).unwrap()),*))
            } else {
                None
            }
        }
    };
}

/**
Macro for filtering entities that have certain components

This macro has two syntaxes:

# Iterator syntax `tags!(C1, C2, ... in WORLD)`

Creates an iterator over the given `World` where the elements
are immutable references to `Entity`s that have all of the
specified components.

# Closure syntax `tags!(C1, C2, ...)`

Creates a closure that takes an `&Entity` and returns a `bool`
indicating whether or not it has all the specified components.
*/
#[macro_export]
macro_rules! tags {
    ($($name:ident),* in $world:expr) => {
        $world.iter().filter(tags!($($name),*))
    };
    ($($name:ident),*) => {
        |entity| $($name::try_entity(entity).is_some() &&)* true
    };
}
