mod outline;
pub use outline::*;

mod solid;
pub use solid::*;

#[derive(PartialEq, Clone)]
pub enum Icon {
    BookOpen,
    CalenderDays,
    Cog8Tooth,
    DocumentPlus,
    PencilSquare,
}
