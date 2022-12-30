use std::collections::HashMap;

use book::{Book, BookID};
use epub::doc::EpubDoc;
use log::error;

pub mod book;

pub struct BookFiles {
    pub books: HashMap<BookID, Book>,
}

impl BookFiles {
    pub fn new() -> Self {
        let paths = match std::fs::read_dir(format!("{}/Books", std::env::var("HOME").unwrap())) {
            Ok(e) => e,
            Err(err) => {
                error!("Couldn't read in `~/Books`: {}", err);
                return BookFiles {
                    books: HashMap::new(),
                };
            }
        };
        let collected = paths.collect::<Vec<_>>();

        let mut books = HashMap::with_capacity(collected.len());

        for path in collected {
            let path_buf = path.unwrap().path();
            let doc = match EpubDoc::new(path_buf.clone()) {
                Ok(doc) => doc,
                Err(e) => {
                    error!("Couldn't read epub file `{:?}`: {e}", path_buf);
                    return BookFiles {
                        books: HashMap::new(),
                    };
                }
            };
            let book = Book::new(doc);
            books.insert(book.id.clone(), book);
        }
        Self { books }
    }
}
