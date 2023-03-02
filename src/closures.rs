use vizia::{
    prelude::{Context, Data, EventContext},
    state::Res,
    style::PseudoClass,
};

macro_rules! closure {
    ($name:ident, $cx:ty) => {
        #[derive(Clone)]
        pub struct $name(pub fn($cx));

        impl $name {
            #[inline]
            pub fn run(&self, cx: $cx) {
                (self.0)(cx)
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }

        impl Data for $name {
            fn same(&self, other: &Self) -> bool {
                self.eq(other)
            }
        }
    };
}

closure!(BuildClosure, &mut Context);
closure!(EventClosure, &mut EventContext);

pub struct InitContext<'a> {
    pub(crate) cx: &'a mut Context,
    pub index: usize,
}

impl InitContext<'_> {
    pub fn checked(&mut self, checked: impl Res<bool>) {
        let entity = self.cx.current();
        checked.set_or_bind(self.cx, entity, |cx, entity, val| {
            let style = EventContext::new(cx).style;
            if let Some(pseudo_classes) = style.pseudo_classes.get_mut(entity) {
                pseudo_classes.set(PseudoClass::CHECKED, val.into());
            } else {
                let mut pseudoclass = PseudoClass::empty();
                pseudoclass.set(PseudoClass::CHECKED, val.into());
                style.pseudo_classes.insert(entity, pseudoclass).unwrap();
            }

            cx.needs_restyle();
        });
    }
}

closure!(InitClosure, &mut InitContext);
