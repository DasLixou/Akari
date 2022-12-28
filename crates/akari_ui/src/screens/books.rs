use akari_books::BookFiles;
use dioxus::prelude::*;

pub fn BooksScreen(cx: Scope) -> Element {
    let files = use_future(&cx, (), |_| async move { BookFiles::new() });

    let content = match files.value() {
        None => rsx! { "loading.." },
        Some(books) => rsx! {
            books.books.iter().enumerate().map(|(i, book)| {
                let title = book.mdata("title").unwrap();
                rsx! { h1 {
                    key: "{i}",
                    "{title}"
                }}
            })
        },
    };

    cx.render(rsx! {
        main {
            class: "p-3",
            content
        }
    })
}
