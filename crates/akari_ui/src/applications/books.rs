pub struct Books;

// Doesnt work for some reason :c
/*impl Application for Books {
    fn routes(cx: Scope) -> Element
    where
        Self: Sized,
    {
        cx.render(rsx! {
            Route { to: "/books/:book", BookScreen {} }
            Route { to: "/books",
                BooksScreen {},
                // Route { to: "/:book", BookScreen {} } // this currently doesnt work :c
            }
        })
    }
}
*/
