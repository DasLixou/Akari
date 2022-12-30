use akari_books::{book::Book, BookFiles};
use dioxus::prelude::*;

use crate::{hooks::use_sidebar, sidebar::SidebarData};

pub fn BooksScreen(cx: Scope) -> Element {
    let files = use_future(&cx, (), |_| async move { BookFiles::new() });

    use_sidebar(&cx, || SidebarData::Applications);

    cx.render(rsx! {
        ul {
            class: "w-full",
            match files.value() {
                None => rsx! { "loading.." },
                Some(books) => rsx! {
                    books.books.iter().map(|(id, book)| {
                        rsx! {
                            BookPresentation {
                                key: "{id}",
                                book: &book,
                                id: id.to_owned(),
                            }
                        }
                    })
                },
            }
        }
    })
}

#[inline_props]
pub fn BookPresentation<'a>(cx: Scope, book: &'a Book, id: String) -> Element {
    cx.render(rsx! {
        li {
            class: "flex px-10 py-3 h-56 w-full border-b border-gray-100",
            Link {
                to: "/books/{id}",
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
