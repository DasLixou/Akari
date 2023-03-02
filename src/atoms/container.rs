use std::{any::Any, borrow::Borrow};

use hashbrown::HashMap;
use once_cell::sync::Lazy;
use vizia::{prelude::*, state::Lens};

use super::atom::Atom;

#[derive(Lens)]
pub struct AtomContainer {
    map: HashMap<u64, Box<dyn Any>>,
}

pub enum ContainerEvent {
    SetAtom(u64, Box<dyn Any + Send>),
}

impl AtomContainer {
    pub fn new() -> Self {
        AtomContainer {
            map: HashMap::new(),
        }
    }

    pub fn lens<T: Sized + Clone + 'static>(atom: &Atom<T>) -> impl Lens<Target = T>
    where
        Atom<T>: Borrow<Atom<T>>,
    {
        let key = Lazy::force(&atom.key).to_owned();
        AtomContainer::map.map(move |map| {
            let b = map.get(&key).expect("Atom wasn't set");
            unsafe { b.downcast_ref::<T>().unwrap_unchecked().clone() }
        })
    }

    pub fn set<T: Sized + Send + 'static>(atom: &Atom<T>, value: T) -> ContainerEvent {
        ContainerEvent::SetAtom(Lazy::force(&atom.key).to_owned(), Box::new(value))
    }
}

impl Model for AtomContainer {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        if let Some(event) = event.take() {
            match event {
                ContainerEvent::SetAtom(id, data) => {
                    self.map.entry(id).insert(data);
                }
            }
        }
    }
}
