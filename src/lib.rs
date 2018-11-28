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
            impl TryRef<$id> for Vec<Component> {
                type Output = $ty;
                fn try_ref(&self, _index: $id) -> Option<&Self::Output> {
                    for comp in self {
                        match comp {
                            Component::$id(val) => return Some(val),
                            _ => ()
                        }
                    }
                    None
                }
            }
            impl TryMut<$id> for Vec<Component> {
                fn try_mut(&mut self, _index: $id) -> Option<&mut Self::Output> {
                    for comp in self {
                        match comp {
                            Component::$id(val) => return Some(val),
                            _ => ()
                        }
                    }
                    None
                }
            }
            impl std::ops::Index<$id> for Vec<Component> {
                type Output = $ty;
                fn index(&self, index: $id) -> &Self::Output {
                    self.try_ref(index)
                        .unwrap_or_else(|| panic!("Unable to find component {:?}", stringify!($id)))
                }
            }
            impl std::ops::IndexMut<$id> for Vec<Component> {
                fn index_mut(&mut self, index: $id) -> &mut Self::Output {
                    self.try_mut(index)
                        .unwrap_or_else(|| panic!("Unable to find component {:?}", stringify!($id)))
                }
            }
        )*
        pub enum Component {
            $($id($ty),)*
        }
    };
}

pub struct Entity<C> {
    components: Vec<C>,
}

impl<C> Entity<C> {
    pub fn new() -> Entity<C> {
        Entity {
            components: Vec::new(),
        }
    }
    pub fn with(mut self, component: C) -> Self {
        self.components.push(component);
        self
    }
}

impl<C> std::ops::Deref for Entity<C> {
    type Target = Vec<C>;
    fn deref(&self) -> &Self::Target {
        &self.components
    }
}

impl<C> std::ops::DerefMut for Entity<C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.components
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn compiles() {
        component! {Size: usize};
        let entity = Entity::new().with(Size::new(5));
        assert_eq!(Some(5), Size::try_of(&entity).cloned())
    }
}
