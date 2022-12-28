use std::{
    fs::{self, File},
    io::{BufReader, Write},
};

use epub::doc::EpubDoc;

pub struct Book {
    pub title: String,
    pub creator: String,
    pub cover_src: String,
}

impl Book {
    pub fn new(mut doc: EpubDoc<BufReader<File>>) -> Self {
        let identifier = doc.mdata("identifier").unwrap();
        let cover_src = format!("/tmp/book_cover-{}.png", identifier);

        match doc.get_cover() {
            Ok(cover_data) => {
                let mut f = fs::File::create(cover_src.clone()).unwrap();
                f.write_all(&cover_data).unwrap();
            }
            _ => {}
        }

        Self {
            title: doc.mdata("title").unwrap_or_default(),
            creator: doc.mdata("creator").unwrap_or_default(),
            cover_src,
        }
    }
}
