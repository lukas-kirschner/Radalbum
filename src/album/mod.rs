mod photo;

use std::{fs, io};
use std::path::PathBuf;
use crate::album::photo::Photo;
use itertools::Itertools;

pub struct Album {
    photos: Vec<Photo>,
}

impl Album {
    pub fn import_all_photos(path: &PathBuf) -> Result<Self, io::Error> {
        Ok(Album {
            photos: fs::read_dir(path)?
                .filter_map(|p| p.ok())
                .map(|p| p.path())
                .filter_map(|p| {
                    if p.extension().map_or(false, |ext| ext == "jpg" || ext == "png") {
                        Some(p)
                    } else {
                        None
                    }
                })
                .sorted()
                .map(|p| {
                    println!("Loading Photo {:?}", &p);
                    p
                })
                .map(|p| Photo::load_from_disk(p))
                .filter_map(|f| f.ok())
                .collect::<Vec<_>>()
        })
    }
    pub fn write_to_disk(&self, path: &PathBuf) {
        for photo in &self.photos {
            println!("{}", photo)
        }
    }
}