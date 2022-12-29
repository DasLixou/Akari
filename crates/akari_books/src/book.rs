use std::{fs::File, io::BufReader};

use epub::doc::EpubDoc;

pub struct Book {
    pub title: String,
    pub creator: String,
    pub cover: String,
}

impl Book {
    pub fn new(mut doc: EpubDoc<BufReader<File>>) -> Self {
        let mut cover = "".to_string();

        match doc.get_cover() {
            Ok(cover_data) => {
                cover = format!("data:image/png;base64,{}", base64::encode(&cover_data));
            }
            _ => {}
        }

        Self {
            title: doc.mdata("title").unwrap_or_default(),
            creator: doc.mdata("creator").unwrap_or_default(),
            cover,
        }
    }
}
