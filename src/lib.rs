#![deny(missing_docs)]

/*!
`eks` is an entity-component system crate with a focus on simplicity and ergonomics.

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
    world.insert(entity! {
        Position: 0,
        Speed: -1,
    });
    world.insert(entity! {
        Position: 2,
        Speed: 3,
        Special: (),
    });

    // Move the entities forward one step
    for (position, speed) in map_mut!(Position, Speed in world) {
        *position += *speed;
    }

    // Check that it worked
    let mut position_iter = map!(Position in world);
    assert_eq!(Some(&-1), position_iter.next());
    assert_eq!(Some(& 5), position_iter.next());
    assert_eq!(1, tags!(Special in world).count())
}
```
*/

mod map;

use std::collections::{BTreeMap, HashMap};

pub use crate::map::*;

/**
An indexing operation that may or may not be successful

You do not need to implement this trait for anything directly.
The `component!` macro does it for you.
*/
pub trait Get<I> {
    /// The index output type
    type Output;
    /// Try to get a reference by index
    fn get(&self, index: I) -> Option<&Self::Output>;
}

/**
A mutable indexing operation that may or may not be successful

You do not need to implement this trait for anything directly.
The `component!` macro does it for you.
*/
pub trait GetMut<I>: Get<I> {
    /// Try to get a mutable reference by index
    fn get_mut(&mut self, index: I) -> Option<&mut Self::Output>;
}

/**
Sets up components for the ECS

Syntax is similar to a `struct`. For each component, a unit `struct` is created,
and a variant is added to a `Component` `enum`.

# Example
```
use eks::*;

component! {
    Position: isize,
    Size: usize,
    Speed: isize,
    IsAlive: bool,
    Name: String
}
```
*/
#[macro_export]
macro_rules! component {
    ($($id:ident: $ty:ty),*) => {
        $(
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
            pub struct $id {}
            impl $id {
                /// Create a new `Component`
                pub fn new(val: $ty) -> Component {
                    Component::$id(val)
                }
                pub fn try_entity(entity: &Entity<Component>) -> Option<&$ty> {
                    entity.get::<$id>()
                }
                pub fn try_entity_mut(entity: &Entity<Component>) -> Option<&mut $ty> {
                    unsafe {
                        (entity
                            as *const Entity<Component>
                            as *mut Entity<Component>
                        ).as_mut()
                    }
                        .unwrap()
                        .get_mut::<$id>()
                }
            }
            impl Get<$id> for Entity<Component> {
                type Output = $ty;
                fn get(&self, index: $id) -> Option<&Self::Output> {
                    let s = format!("{:?}", index);
                    if let Some(index) = self.indices.get(&s).cloned() {
                        match &self.components[index] {
                            Component::$id(ref val) => Some(val),
                            _ => unreachable!()
                        }
                    } else {
                        None
                    }
                }
            }
            impl GetMut<$id> for Entity<Component> {
                fn get_mut(&mut self, index: $id) -> Option<&mut Self::Output> {
                    let s = format!("{:?}", index);
                    if let Some(index) = self.indices.get(&s).cloned() {
                        match &mut self.components[index] {
                            Component::$id(ref mut val) => Some(val),
                            _ => unreachable!()
                        }
                    } else {
                        None
                    }
                }
            }
            impl std::ops::Index<$id> for Entity<Component> {
                type Output = $ty;
                fn index(&self, index: $id) -> &Self::Output {
                    Get::get(self, index)
                        .unwrap_or_else(|| panic!("Unable to find component {:?}", stringify!($id)))
                }
            }
            impl std::ops::IndexMut<$id> for Entity<Component> {
                fn index_mut(&mut self, index: $id) -> &mut Self::Output {
                    GetMut::get_mut(self, index)
                        .unwrap_or_else(|| panic!("Unable to find component {:?}", stringify!($id)))
                }
            }
        )*
        #[derive(Debug, Clone, PartialEq, PartialOrd)]
        pub enum Component {
            $($id($ty)),*
        }

        impl ToString for Component {
            fn to_string(&self) -> String {
                match self {
                    $(Component::$id(_) => format!("{:?}", $id::default())),*
                }
            }
        }
    };
    ($($id:ident: $ty:ty,)*) => {
        component!{$($id: $ty),*}
    };
}

/**
An entity in the ECS

The fields of this `struct` are public so that the `component!`
macro works correctly. They should not be modified directly.
*/
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Entity<C> {
    /// The id of the `Entity`
    id: usize,
    /// The actual list of components
    pub components: Vec<C>,
    /// A map of formatted component names to indices in
    /// the `components`
    pub indices: HashMap<String, usize>,
}

impl<C> Entity<C> {
    /// Create a new `Entity`
    pub fn new() -> Entity<C> {
        Entity {
            id: 0,
            components: Vec::new(),
            indices: HashMap::new(),
        }
    }

    /// Gets the `Entity`'s id
    pub fn id(&self) -> usize {
        self.id
    }

    /// Get an optional reference to a component's value
    pub fn get<T>(&self) -> Option<&<Self as Get<T>>::Output>
    where
        T: Default,
        Self: Get<T>,
    {
        Get::get(self, T::default())
    }

    /// Get an optional mutable reference to a component's value
    pub fn get_mut<T>(&mut self) -> Option<&mut <Self as Get<T>>::Output>
    where
        T: Default,
        Self: GetMut<T>,
    {
        GetMut::get_mut(self, T::default())
    }
}

impl<C: ToString> Entity<C> {
    /// Add a component to the `Entity`
    pub fn with(mut self, component: C) -> Self {
        self.add(component);
        self
    }

    /// Add a component to the `Entity`
    pub fn add(&mut self, component: C) {
        let s = component.to_string();
        if !self.indices.contains_key(&s) {
            self.indices
                .insert(component.to_string(), self.components.len());
            self.components.push(component);
        }
    }
}

/**
Creates an `Entity` with `struct`-like syntax

# Example
```
use eks::*;

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
        let mut entity = Entity::new();
        $(entity.add($id::new($value));)*
        entity
    }};
    ($($id:ident: $value:expr,)*) => {
        entity!{$($id: $value),*}
    };
}

/// The world of the ECS
pub struct World<C> {
    entities: BTreeMap<usize, Entity<C>>,
    next_id: usize,
}

impl<C> World<C> {
    /// Create a new `World`
    pub fn new() -> World<C> {
        World {
            entities: BTreeMap::new(),
            next_id: 1,
        }
    }

    /// Add an `Entity` to the `World`
    pub fn insert(&mut self, mut entity: Entity<C>) {
        entity.id = self.next_id;
        self.entities.insert(self.next_id, entity);
        self.next_id += 1;
    }

    /// Removes the `Entity` with the given id
    pub fn remove(&mut self, id: usize) -> Option<Entity<C>> {
        self.entities.remove(&id)
    }

    /// Iterates through all `Entities` in the `World`
    pub fn iter(&self) -> std::collections::btree_map::Values<usize, Entity<C>> {
        self.entities.values()
    }

    /// Mutable iterates through all `Entities` in the `World`
    pub fn iter_mut(&mut self) -> std::collections::btree_map::ValuesMut<usize, Entity<C>> {
        self.entities.values_mut()
    }
}

/// An iterator adapter that converts and `Entity` iterator to
/// an iterator over the `Entity`'s ids
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
    type Item = usize;

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
    use super::*;
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
    fn ids() {
        component! {
            X: (),
            Y: ()
        }

        let mut world = World::new();

        world.insert(entity! {X: ()});
        world.insert(entity! {X: ()});
        world.insert(entity! {X: ()});

        world.remove(2);

        let mut ids = world.iter().ids();
        assert_eq!(Some(1), ids.next());
        assert_eq!(Some(3), ids.next());
        assert_eq!(None, ids.next());
    }
}
