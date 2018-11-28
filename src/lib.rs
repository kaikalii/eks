mod iter;

pub trait TryRef<I> {
    type Output;
    fn try_ref(&self, index: I) -> Option<&Self::Output>;
}

pub trait TryMut<I>: TryRef<I> {
    fn try_mut(&mut self, index: I) -> Option<&mut Self::Output>;
}

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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Entity<T> {
    components: Vec<T>,
}

impl<T> Entity<T> {
    pub fn new() -> Entity<T> {
        Entity {
            components: Vec::new(),
        }
    }
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

pub struct World<T> {
    entities: Vec<Entity<T>>,
}

impl<T> World<T> {
    pub fn new() -> World<T> {
        World {
            entities: Vec::new(),
        }
    }
}

impl<T> World<T> {
    pub fn iter1<A>(&self, component: A) -> iter::Iter<'_, T, A> {
        iter::Iter {
            iter: self.entities.iter(),
            a: component,
        }
    }
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
