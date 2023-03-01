use vizia::prelude::{Context, Data, EventContext};

#[derive(Clone)]
pub struct BuildClosure(pub fn(&mut Context));

impl BuildClosure {
    #[inline]
    pub fn build(&self, cx: &mut Context) {
        (self.0)(cx)
    }
}

impl PartialEq for BuildClosure {
    fn eq(&self, other: &Self) -> bool {
        self.0 as usize == other.0 as usize
    }
}

impl Data for BuildClosure {
    fn same(&self, other: &Self) -> bool {
        self.eq(other)
    }
}

#[derive(Clone)]
pub struct EventClosure(pub fn(&mut EventContext));

impl EventClosure {
    #[inline]
    pub fn emit(&self, cx: &mut EventContext) {
        (self.0)(cx)
    }
}

impl PartialEq for EventClosure {
    fn eq(&self, other: &Self) -> bool {
        self.0 as usize == other.0 as usize
    }
}

impl Data for EventClosure {
    fn same(&self, other: &Self) -> bool {
        self.eq(other)
    }
}
