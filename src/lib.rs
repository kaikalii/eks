#![deny(missing_docs)]

/*!
`eks` is an entity-component system crate with a focus on simplicity.
*/

pub mod iter;

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
                pub fn new(val: $ty) -> Component {
                    Component::$id(val)
                }
                pub fn of(entity: &Entity<Component>) -> &$ty {
                    &entity[$id {}]
                }
                pub fn of_mut(entity: &mut Entity<Component>) -> &mut $ty {
                    &mut entity[$id {}]
                }
                pub fn try_of(entity: &Entity<Component>) -> Option<&$ty> {
                    entity.try_ref($id {})
                }
                pub fn try_of_mut(entity: &mut Entity<Component>) -> Option<&mut $ty> {
                    entity.try_mut($id {})
                }
            }
            #[allow(non_snake_case)]
            pub fn $id(val: $ty) -> Component {
                $id::new(val)
            }
            impl TryRef<$id> for Entity<Component> {
                type Output = $ty;
                fn try_ref(&self, _index: $id) -> Option<&Self::Output> {
                    for comp in self.iter() {
                        match comp {
                            Component::$id(ref val) => return Some(val),
                            _ => ()
                        }
                    }
                    None
                }
            }
            impl TryMut<$id> for Entity<Component> {
                fn try_mut(&mut self, _index: $id) -> Option<&mut Self::Output> {
                    for comp in self.iter_mut() {
                        match comp {
                            Component::$id(ref mut val) => return Some(val),
                            _ => ()
                        }
                    }
                    None
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
            $($id($ty),)*
        }
    };
}

/// An entity in the ECS
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Entity<T> {
    components: Vec<T>,
}

impl<T> Entity<T> {
    /// Create a new `Entity`
    pub fn new() -> Entity<T> {
        Entity {
            components: Vec::new(),
        }
    }
    /// Add a component to the `Entity`
    pub fn with(mut self, component: T) -> Self {
        self.components.push(component);
        self
    }
}

impl<T> std::ops::Deref for Entity<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.components
    }
}

impl<T> std::ops::DerefMut for Entity<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.components
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

impl<T> World<T> {
    /// Iterate over all `Entity`s that have the given component with access to its value
    pub fn iter1<A>(&self, component: A) -> iter::Iter1<'_, T, A> {
        iter::Iter1 {
            iter: self.entities.iter(),
            a: component,
        }
    }
    /// Iterate over all `Entity`s that have both the given components with access to their values
    pub fn iter2<A, B>(&self, a: A, b: B) -> iter::Iter2<'_, T, A, B> {
        iter::Iter2 {
            iter: self.entities.iter(),
            a,
            b,
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
            Size: usize
        };
        let mut world = World::new();
        world.push(Entity::new().with(Size(5)));
        world.push(Entity::new().with(Size(3)).with(Position(-1)));
        let mut iter = world.iter2(Size {}, Position {});
        assert_eq!(Some((&3, &-1)), iter.next());
        assert_eq!(None, iter.next());
    }
}
