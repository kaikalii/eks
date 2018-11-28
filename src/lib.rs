#![deny(missing_docs)]

/*!
`eks` is an entity-component system crate with a focus on simplicity.
*/

mod iter;

use std::collections::HashMap;

pub use crate::iter::*;

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
    IsAlive: bool
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
                /// Create a new `Ref` indexer
                pub fn as_ref() -> Ref<Self> {
                    Ref($id {})
                }
                /// Create a new `Mut` indexer
                pub fn as_mut() -> Mut<Self> {
                    Mut($id {})
                }
            }
            #[allow(non_snake_case)]
            pub fn $id(val: $ty) -> Component {
                $id::new(val)
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
}

/**
An entity in the ECS

The fields of this `struct` are public so that the `component!`
macro works correctly. They should not be modified directly.
*/
#[derive(Debug, Clone, PartialEq)]
pub struct Entity<T> {
    /// The actual list of components
    pub components: Vec<T>,
    /// A map of formatted component names to indices in
    /// the `components`
    pub indices: HashMap<String, usize>,
}

impl<T> Entity<T> {
    /// Create a new `Entity`
    pub fn new() -> Entity<T> {
        Entity {
            components: Vec::new(),
            indices: HashMap::new(),
        }
    }
}

impl<T: ToString> Entity<T> {
    /// Add a component to the `Entity`
    pub fn with(mut self, component: T) -> Self {
        self.indices
            .insert(component.to_string(), self.components.len());
        self.components.push(component);
        self
    }
}

/// The world of the ECS
pub struct World<T> {
    entities: Vec<Entity<T>>,
}

impl<T> World<T> {
    /// Create a new `World`
    pub fn new() -> World<T> {
        World {
            entities: Vec::new(),
        }
    }
}

impl<T> std::ops::Deref for World<T> {
    type Target = Vec<Entity<T>>;
    fn deref(&self) -> &Self::Target {
        &self.entities
    }
}

impl<T> std::ops::DerefMut for World<T> {
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
            Speed: isize
        };
        let mut world = World::new();
        world.push(Entity::new().with(Position(5)));
        world.push(Entity::new().with(Position(-1)).with(Speed(3)));
        for (position, speed) in world
            .iter_mut()
            .filter_map(map_mut!(Position::as_mut(), Speed {}))
        {
            *position += *speed
        }
        assert_eq!(
            Some((&2, &3)),
            world.iter().filter_map(map!(Position {}, Speed {})).next()
        );
    }
}
