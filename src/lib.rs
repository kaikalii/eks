#![deny(missing_docs)]

/*!
`eks` is an entity-component system crate with a focus on simplicity and ergonomics.

Features:
    * `f_rayon` Use rayon parallel iterators

# Example
```
use eks::*;

// Set up the components
component! {
    Position: isize,
    Speed: isize,
    Special: (),
}

fn main() {
    // Create the world
    let mut world = World::new();

    // Add some entities
    let a = world.insert(entity! {
        Position: 0,
        Speed: -1,
    });
    let b = world.insert(entity! {
        Position: 2,
        Speed: 3,
        Special: (),
    });

    // Move the entities forward one step
    for (position, speed) in map_mut!(Position, Speed in world) {
        *position += *speed;
    }

    // Check that it worked
    assert_eq!(-1, world[a][Position]);
    assert_eq!( 5, world[b][Position]);
    assert_eq!(1, tags!(Special in world).count())
}
```
*/

pub mod example_component;
mod map;

use std::{
    collections::HashMap,
    fmt,
    ops::{Index, IndexMut},
};

#[cfg(feature = "f_rayon")]
use rayon::prelude::*;
use uuid::Uuid;

pub use crate::map::*;

/**
Trait for components

You do not need to impliment this trait manually.
The `component!` macro will do it for you.
*/
pub trait Component {
    /// The component's type
    type Type;
    /// The component's associated enum
    type Enum;
    /// Create a new component enum from the value
    fn new(val: Self::Type) -> Self::Enum;
    #[doc(hidden)]
    const AS_STR: &'static str;
    /// Try to get a reference to this component from an `Entity`
    #[doc(hidden)]
    fn try_entity(entity: &Entity<Self::Enum>) -> Option<&Self::Type>;
    /// Try to get a mutable reference to this component from an `Entity`
    #[doc(hidden)]
    fn try_entity_mut(entity: &Entity<Self::Enum>) -> Option<&mut Self::Type>;
    #[doc(hidden)]
    fn enum_as_val(enm: &Self::Enum) -> &Self::Type;
    #[doc(hidden)]
    fn enum_as_val_mut(enm: &mut Self::Enum) -> &mut Self::Type;
    #[doc(hidden)]
    fn enum_to_val(enm: Self::Enum) -> Self::Type;
}

/**
Sets up components for the ECS

For a usage example, check out [the component example module](example_component/index.html)
*/
#[macro_export]
macro_rules! component {
    ($(#[$top_attr:meta])* $name:ident { $($(#unit #[$unit_attr:meta])* $(#variant #[$variant_attr:meta])* $id:ident: $ty:ty),* $(,)* }) => {
        $(
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
            $(#[$unit_attr])*
            pub struct $id {}
            impl eks::Component for $id {
                const AS_STR: &'static str = stringify!($id);
                type Type = $ty;
                type Enum = $name;
                fn enum_as_val(enm: &Self::Enum) -> &$ty {
                    if let $name::$id(val) = enm {
                        val
                    } else {
                        panic!(concat!("Component is not ", stringify!($id)))
                    }
                }
                fn enum_as_val_mut(enm: &mut Self::Enum) -> &mut $ty {
                    if let $name::$id(val) = enm {
                        val
                    } else {
                        panic!(concat!("Component is not ", stringify!($id)))
                    }
                }
                fn enum_to_val(enm: Self::Enum) -> $ty {
                    if let $name::$id(val) = enm {
                        val
                    } else {
                        panic!(concat!("Component is not ", stringify!($id)))
                    }
                }
                /// Create a new component
                fn new(val: $ty) -> $name {
                    $name::$id(val)
                }
                /// Try to get a reference to this component from an `Entity`
                #[doc(hidden)]
                fn try_entity(entity: &eks::Entity<$name>) -> Option<&$ty> {
                    entity.get::<$id>()
                }
                /// Try to get a mutable reference to this component from an `Entity`
                #[doc(hidden)]
                fn try_entity_mut(entity: &eks::Entity<$name>) -> Option<&mut $ty> {
                    unsafe {
                        (entity
                            as *const eks::Entity<$name>
                            as *mut eks::Entity<$name>
                        ).as_mut()
                    }
                        .unwrap()
                        .get_mut::<$id>()
                }
            }
            impl std::fmt::Display for $id {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, stringify!($id))
                }
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals)]
            #[allow(dead_code)]
            pub const $id: $id = $id {};
            impl std::ops::Index<$id> for eks::Entity<$name> {
                type Output = $ty;
                fn index(&self, _: $id) -> &Self::Output {
                    self.get::<$id>()
                        .unwrap_or_else(|| panic!("Unable to find component {:?}", stringify!($id)))
                }
            }
            impl std::ops::IndexMut<$id> for eks::Entity<$name> {
                fn index_mut(&mut self, _: $id) -> &mut Self::Output {
                    self.get_mut::<$id>()
                        .unwrap_or_else(|| panic!("Unable to find component {:?}", stringify!($id)))
                }
            }
        )*
        $(#[$top_attr])*
        pub enum $name {
            $($(#[$variant_attr])* $id($ty)),*
        }

        impl AsRef<&'static str> for $name {
             fn as_ref(&self) -> &&'static str {
                match self {
                    $($name::$id(_) => &stringify!($id)),*
                }
            }
        }
    };
    ($(#[$top_attr:meta])* $($(#unit #[$unit_attr:meta])* $(#variant #[$variant_attr:meta])* $id:ident: $ty:ty),* $(,)*) => {
        eks::component!{ $(#[$top_attr])* Comp { $( $(#unit #[$unit_attr])* $(#variant #[$variant_attr])* $id: $ty),* } }
    };
}

/// An `Entity` id
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id(Uuid);

impl Id {
    fn new() -> Id {
        Id(Uuid::new_v4())
    }
}

impl fmt::Debug for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Uuid as fmt::Debug>::fmt(&self.0, f)
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Uuid as fmt::Display>::fmt(&self.0, f)
    }
}

/**
An entity in the ECS
*/
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entity<C> {
    /// The id of the `Entity`
    id: Id,
    /// A map of formatted component names to indices in
    /// the `components`
    #[doc(hidden)]
    pub components: HashMap<&'static str, C>,
}

impl<C> Entity<C> {
    /// Create a new `Entity`
    pub fn new() -> Entity<C> {
        Entity {
            id: Id::new(),
            components: HashMap::new(),
        }
    }
    /// Gets the `Entity`'s id
    pub fn id(&self) -> Id {
        self.id
    }
    /// Get an optional reference to a component's value
    pub fn get<T>(&self) -> Option<&T::Type>
    where
        T: Component<Enum = C>,
    {
        if let Some(component) = self.components.get(T::AS_STR) {
            Some(T::enum_as_val(component))
        } else {
            None
        }
    }
    /// Get an optional mutable reference to a component's value
    pub fn get_mut<T>(&mut self) -> Option<&mut T::Type>
    where
        T: Component<Enum = C>,
    {
        if let Some(component) = self.components.get_mut(T::AS_STR) {
            Some(T::enum_as_val_mut(component))
        } else {
            None
        }
    }
    /// Check if the `Entity` has the `Component`
    pub fn has<T>(&self) -> bool
    where
        T: Component<Enum = C>,
    {
        self.get::<T>().is_some()
    }
    /// Add a `Component` to the `Entity`
    pub fn add<T>(&mut self, value: T::Type) -> Option<T::Type>
    where
        T: Component<Enum = C>,
    {
        if let Some(component) = self.components.insert(T::AS_STR, T::new(value)) {
            Some(T::enum_to_val(component))
        } else {
            None
        }
    }
    /// Add a `Component` to the `Entity`
    pub fn with<T>(mut self, value: T::Type) -> Self
    where
        T: Component<Enum = C>,
    {
        self.add::<T>(value);
        self
    }
    /// Remove a `Component` from the `Entity`
    pub fn remove<T>(&mut self) -> Option<T::Type>
    where
        T: Component<Enum = C>,
    {
        if let Some(component) = self.components.remove(T::AS_STR) {
            Some(T::enum_to_val(component))
        } else {
            None
        }
    }
}

/**
Creates an `Entity` with `struct`-like syntax

# Example
```
use eks::{component, entity};

component! {
    Name: String,
    Age: u8
}

fn main() {
    let dan = entity! {
        Name: "Dan".to_string(),
        Age: 26
    };
    assert_eq!(Some(&26), dan.get::<Age>())
}
```
*/
#[macro_export]
macro_rules! entity {
    ($($id:ident: $value:expr),*) => {{
        let mut entity = eks::Entity::new();
        $(entity.add::<$id>($value);)*
        entity
    }};
    ($($id:ident: $value:expr,)*) => {
        eks::entity!{$($id: $value),*}
    };
}

/// The world of the ECS
pub struct World<C> {
    entities: HashMap<Id, Entity<C>>,
}

impl<C> World<C> {
    /// Create a new `World`
    pub fn new() -> World<C> {
        World {
            entities: HashMap::new(),
        }
    }
    /// Add an `Entity` to the `World`
    pub fn insert(&mut self, entity: Entity<C>) -> Id {
        let id = entity.id();
        self.entities.insert(id, entity);
        id
    }
    /// Removes the `Entity` with the given id
    pub fn remove(&mut self, id: Id) -> Option<Entity<C>> {
        self.entities.remove(&id)
    }
    /// Iterates through all `Entities` in the `World`
    pub fn iter(&self) -> std::collections::hash_map::Values<Id, Entity<C>> {
        self.entities.values()
    }
    /// Mutable iterates through all `Entities` in the `World`
    pub fn iter_mut(&mut self) -> std::collections::hash_map::ValuesMut<Id, Entity<C>> {
        self.entities.values_mut()
    }
    /// Get a reference to the `Entity` with the given `Id`
    pub fn get(&self, id: Id) -> Option<&Entity<C>> {
        self.entities.get(&id)
    }
    /// Get a mutable reference to the `Entity` with the given `Id`
    pub fn get_mut(&mut self, id: Id) -> Option<&mut Entity<C>> {
        self.entities.get_mut(&id)
    }
}

impl<C> Index<Id> for World<C> {
    type Output = Entity<C>;
    fn index(&self, id: Id) -> &Self::Output {
        self.get(id)
            .unwrap_or_else(|| panic!("Unable to find entity with id: {}", id))
    }
}

impl<C> IndexMut<Id> for World<C> {
    fn index_mut(&mut self, id: Id) -> &mut Self::Output {
        self.get_mut(id)
            .unwrap_or_else(|| panic!("Unable to find entity with id: {}", id))
    }
}

#[cfg(feature = "f_rayon")]
impl<'a, C> IntoParallelIterator for &'a World<C>
where
    C: Sync,
{
    type Item = <&'a HashMap<Id, Entity<C>> as IntoIterator>::Item;
    type Iter = rayon::collections::hash_map::Iter<'a, Id, Entity<C>>;
    fn into_par_iter(self) -> Self::Iter {
        self.entities.into_par_iter()
    }
}

#[cfg(feature = "f_rayon")]
impl<'a, C> IntoParallelIterator for &'a mut World<C>
where
    C: Send,
{
    type Item = <&'a mut HashMap<Id, Entity<C>> as IntoIterator>::Item;
    type Iter = rayon::collections::hash_map::IterMut<'a, Id, Entity<C>>;
    fn into_par_iter(self) -> Self::Iter {
        (&mut self.entities).into_par_iter()
    }
}

/// An iterator adapter that converts and `Entity` iterator to
/// an iterator over the `Entity`s' ids
pub struct Ids<C, E, I>
where
    E: std::borrow::Borrow<Entity<C>>,
    I: Iterator<Item = E>,
{
    iter: I,
    pd: std::marker::PhantomData<C>,
}

impl<C, E, I> Iterator for Ids<C, E, I>
where
    E: std::borrow::Borrow<Entity<C>>,
    I: Iterator<Item = E>,
{
    type Item = Id;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|e| e.borrow().id())
    }
}

/// Adds adapter functions for `Entity` iterators
pub trait EntityIterator<C, E>: Iterator<Item = E> + Sized
where
    E: std::borrow::Borrow<Entity<C>>,
{
    /// Converts and `Entity` iterator to
    /// an iterator over the `Entity`'s ids
    fn ids(self) -> Ids<C, E, Self> {
        Ids {
            iter: self,
            pd: std::marker::PhantomData,
        }
    }
}

impl<C, E, I> EntityIterator<C, E> for I
where
    I: Iterator<Item = E> + Sized,
    E: std::borrow::Borrow<Entity<C>>,
{
}

#[cfg(test)]
mod test {
    mod eks {
        pub use crate::*;
    }
    use eks::*;
    #[test]
    fn works() {
        component! {
            Position: isize,
            Speed: isize,
        };

        let mut world = World::new();

        world.insert(entity! {
            Position: 5,
        });
        world.insert(entity! {
            Position: -1,
            Speed: 3,
        });

        for (position, speed) in map_mut_checked!(Position, Speed in world) {
            *position += *speed
        }

        assert_eq!(Some((&2, &3)), map!(Position, Speed in world).next());
        assert_eq!(1, tags!(Speed in world).count());
    }
    #[test]
    #[cfg(feature = "f_rayon")]
    fn rayon() {
        component! { Foo: (), Bar: () }
        let mut world = World::new();
        for _ in 0..100 {
            world.insert(entity!(Foo: ()));
        }
        assert_eq!(100, tags!(Foo in par world).count());
    }
}
