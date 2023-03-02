use std::{hash::Hash, marker::PhantomData};

#[derive(Clone, Copy)]
pub struct Atom<T: ?Sized> {
    pub(crate) key: u64,
    phantom: PhantomData<T>,
}

impl<T: ?Sized> Atom<T> {
    pub const fn new(key: u64) -> Self {
        Self {
            key,
            phantom: PhantomData,
        }
    }
}

impl<T: ?Sized> Hash for Atom<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}

impl<T: ?Sized> Eq for Atom<T> {}

impl<T: ?Sized> PartialEq for Atom<T> {
    fn eq(&self, other: &Self) -> bool {
        self.key.eq(&other.key)
    }
}
