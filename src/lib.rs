#![deny(missing_docs)]

/*!
`eks` is an entity-component system crate with a focus on simplicity and ergonomics.

# Example
```
use eks::*;

component! {
    Position: isize,
    Speed: isize,
    Special: (),
}

fn main() {
    // Create the world
    let mut world = World::new();

    // Add some entities
    world.push(entity! {
        Position: 0,
        Speed: -1,
    });
    world.push(entity! {
        Position: 2,
        Speed: 3,
        Special: (),
    });

    // Move the entites forward one step
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

use std::collections::HashMap;

pub use crate::map::*;

/// An indexing operation that may or may not be successful
pub trait TryRef<I> {
    /// The index output type
    type Output;
    /// Try to get a reference by index
    fn try_ref(&self, index: I) -> Option<&Self::Output>;
}

/// A mutable indexing operation that may or may not be successful
pub trait TryMut<I>: TryRef<I> {
    /// Try to get a mutable reference by index
    fn try_mut(&mut self, index: I) -> Option<&mut Self::Output>;
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
            }
            impl TryRef<$id> for Entity<Component> {
                type Output = $ty;
                fn try_ref(&self, index: $id) -> Option<&Self::Output> {
                    let s = format!("{:?}", index);
                    if let Some(index) = self.indices.get(&s).cloned() {
                        match &self.components[index] {
                            Component::$id(ref val) => Some(val),
                            _ => panic!("Entity::indices led to incorrect component")
                        }
                    } else {
                        None
                    }
                }
            }
            impl TryMut<$id> for Entity<Component> {
                fn try_mut(&mut self, index: $id) -> Option<&mut Self::Output> {
                    let s = format!("{:?}", index);
                    if let Some(index) = self.indices.get(&s).cloned() {
                        match &mut self.components[index] {
                            Component::$id(ref mut val) => Some(val),
                            _ => panic!("Entity::indices led to incorrect component")
                        }
                    } else {
                        None
                    }
                }
            }
            impl std::ops::Index<$id> for Entity<Component> {
                type Output = $ty;
                fn index(&self, index: $id) -> &Self::Output {
                    self.try_ref(index)
                        .unwrap_or_else(|| panic!("Unable to find component {:?}", stringify!($id)))
                }
            }
            impl std::ops::IndexMut<$id> for Entity<Component> {
                fn index_mut(&mut self, index: $id) -> &mut Self::Output {
                    self.try_mut(index)
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
#[derive(Debug, Clone, PartialEq)]
pub struct Entity<C> {
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
            components: Vec::new(),
            indices: HashMap::new(),
        }
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
Creates an `Entity` with `struct`-like syntax.

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
    assert_eq!(26, dan[Age {}])
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
    entities: Vec<Entity<C>>,
}

impl<C> World<C> {
    /// Create a new `World`
    pub fn new() -> World<C> {
        World {
            entities: Vec::new(),
        }
    }
}

impl<C> std::ops::Deref for World<C> {
    type Target = Vec<Entity<C>>;
    fn deref(&self) -> &Self::Target {
        &self.entities
    }
}

impl<C> std::ops::DerefMut for World<C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entities
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn compiles() {
        component! {
            Position: isize,
            Speed: isize,
        };

        let mut world = World::new();

        world.push(entity! {
            Position: 5,
        });
        world.push(entity! {
            Position: -1,
            Speed: 3,
        });

        for (position, speed) in map_mut_checked!(Position, Speed in world) {
            *position += *speed
        }

        assert_eq!(Some((&2, &3)), map!(Position, Speed in world).next());
        assert_eq!(1, tags!(Speed in world).count());

        *map_mut!(Position)(&mut world[0]).unwrap() = 10;
    }
}
