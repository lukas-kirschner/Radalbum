mod photo;

use std::{env, fs, io};
use std::io::ErrorKind;
use std::path::PathBuf;
use crate::album::photo::Photo;
use itertools::Itertools;

pub struct Album {
    photos: Vec<Photo>,
}

impl Album {
    const ASSETS_DIR: &'static str = "assets";
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
    fn write_aux_files(&self, path: &PathBuf) -> io::Result<()> {
        let mut assets_path = env::current_exe()?.parent().ok_or(io::Error::new(ErrorKind::Unsupported, "Exe Path did not have a parent!"))?.join(Self::ASSETS_DIR);
        if !assets_path.is_dir() {
            assets_path = PathBuf::from(Self::ASSETS_DIR);
        }
        for asset in ["Makefile", "Test.css"] {
            fs::copy(assets_path.join(asset), path.join(asset))?;
        }
        Ok(())
    }
    pub fn write_to_disk(&mut self, path: &PathBuf) {
        if let Err(e) = self.write_aux_files(path) {
            eprintln!("{:?}", e);
        }
        // Copy all photos to the output directory.
        for photo in &mut self.photos {
            match photo.write_to_directory(path) {
                Ok(p) => { *photo = p; }
                Err(e) => { eprintln!("{:?}", e); }
            }
        }
        for photo in &self.photos {
            println!("{}", photo)
        }
    }
}