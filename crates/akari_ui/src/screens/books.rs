use akari_books::{book::Book, BookFiles};
use dioxus::prelude::*;

use crate::components::sidebar::MainSidebar;

pub fn BooksScreen(cx: Scope) -> Element {
    let files = use_future(&cx, (), |_| async move { BookFiles::new() });

    cx.render(rsx! {
        main {
            class: "flex",
            MainSidebar {},
            ul {
                class: "w-full",
                match files.value() {
                    None => rsx! { "loading.." },
                    Some(books) => rsx! {
                        books.books.iter().enumerate().map(|(i, book)| {
                            rsx! {
                                BookPresentation {
                                    key: "{i}",
                                    book: &book,
                                }
                            }
                        })
                    },
                }
            }
        }
    })
}

#[inline_props]
pub fn BookPresentation<'a>(cx: Scope, book: &'a Book) -> Element {
    cx.render(rsx! {
        li {
            class: "flex px-10 py-3 h-56 w-full border-b border-gray-100",
            button {
                class: "flex w-full h-full",
                img {
                    class: "w-36 h-auto max-h-full object-contain",
                    src: "{book.cover}"
                },
                div {
                    class: "flex flex-col pl-3 justify-center h-full items-start",
                    h1 { class: "text-xl", "{book.title}" }
                    h2 { class: "text-md italic text-gray-700", "{book.creator}" }
                }
            }
        }
    })
}
