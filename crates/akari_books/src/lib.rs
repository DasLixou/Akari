use book::Book;
use epub::doc::EpubDoc;
use log::error;

pub mod book;

pub struct BookFiles {
    pub books: Vec<Book>,
}

impl BookFiles {
    pub fn new() -> Self {
        let paths = match std::fs::read_dir(format!("{}/Books", std::env::var("HOME").unwrap())) {
            Ok(e) => e,
            Err(err) => {
                error!("Couldn't read in `~/Books`: {}", err);
                return BookFiles { books: vec![] };
            }
        };
        let collected = paths.collect::<Vec<_>>();

        let mut books = Vec::with_capacity(collected.len());

        for path in collected {
            let path_buf = path.unwrap().path();
            let doc = match EpubDoc::new(path_buf.clone()) {
                Ok(doc) => doc,
                Err(e) => {
                    error!("Couldn't read epub file `{:?}`: {e}", path_buf);
                    return BookFiles { books: vec![] };
                }
            };
            books.push(Book::new(doc));
        }
        Self { books }
    }
}
