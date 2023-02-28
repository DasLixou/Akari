use vizia::state::Lens;

pub struct Toggle<A, B, T> {
    a: A,
    b: B,
    toggle: T,
}

impl<A, B, T> Toggle<A, B, T> {
    pub const fn new(a: A, b: B, toggle: T) -> Self
    where
        A: Lens,
        B: Lens,
        T: Lens<Target = bool>,
    {
        Self { a, b, toggle }
    }
}

impl<A, B, T> Lens for Toggle<A, B, T>
where
    A: Lens,
    B: Lens<Source = A::Source, Target = A::Target>,
    T: Lens<Source = A::Source, Target = bool>,
{
    type Source = A::Source;
    type Target = B::Target;

    fn view<O, F: FnOnce(Option<&Self::Target>) -> O>(&self, source: &Self::Source, map: F) -> O {
        self.toggle.view(source, |t| match t {
            Some(false) => self.a.view(source, map),
            Some(true) => self.b.view(source, map),
            None => map(None),
        })
    }

    fn name(&self) -> Option<&'static str> {
        self.toggle.name()
    }
}

impl<A: Clone, B: Clone, T: Clone> Clone for Toggle<A, B, T> {
    fn clone(&self) -> Self {
        Self {
            a: self.a.clone(),
            b: self.b.clone(),
            toggle: self.toggle.clone(),
        }
    }
}

impl<A: Copy, B: Copy, T: Copy> Copy for Toggle<A, B, T> {}
