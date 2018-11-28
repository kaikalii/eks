//! `World` iterators

use crate::{Entity, TryRef};

/// An iterator over one component in a `World`. Created by `World::iter1`.
pub struct Iter1<'a, T, A> {
    pub(crate) iter: std::slice::Iter<'a, Entity<T>>,
    pub(crate) a: A,
}

impl<'a, T, A> Iterator for Iter1<'a, T, A>
where
    A: Copy,
    Entity<T>: TryRef<A>,
    <Entity<T> as TryRef<A>>::Output: 'a,
{
    type Item = &'a <Entity<T> as TryRef<A>>::Output;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(entity) = self.iter.next() {
                if let Some(a) = entity.try_ref(self.a) {
                    break Some(a);
                }
            } else {
                break None;
            }
        }
    }
}

/// An iterator over two components in a `World`. Created by `World::iter2`.
pub struct Iter2<'a, T, A, B> {
    pub(crate) iter: std::slice::Iter<'a, Entity<T>>,
    pub(crate) a: A,
    pub(crate) b: B,
}

impl<'a, T, A, B> Iterator for Iter2<'a, T, A, B>
where
    A: Copy,
    B: Copy,
    Entity<T>: TryRef<A>,
    <Entity<T> as TryRef<A>>::Output: 'a,
    Entity<T>: TryRef<B>,
    <Entity<T> as TryRef<B>>::Output: 'a,
{
    type Item = (
        &'a <Entity<T> as TryRef<A>>::Output,
        &'a <Entity<T> as TryRef<B>>::Output,
    );
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(entity) = self.iter.next() {
                if let (Some(a), Some(b)) = (entity.try_ref(self.a), entity.try_ref(self.b)) {
                    break Some((a, b));
                }
            } else {
                break None;
            }
        }
    }
}
