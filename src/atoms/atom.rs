use std::{hash::Hash, marker::PhantomData};

use once_cell::sync::Lazy;

pub struct Atom<T: ?Sized> {
    pub(crate) key: Lazy<u64>,
    phantom: PhantomData<T>,
}

impl<T: ?Sized> Atom<T> {
    pub const fn new() -> Self {
        static mut ATOM_COUNTER: u64 = 0;
        Self {
            key: Lazy::new(|| unsafe {
                let val = ATOM_COUNTER;
                ATOM_COUNTER += 1;
                val
            }),
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
