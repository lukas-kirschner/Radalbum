pub mod photo;

use crate::album::photo::FourPhotosTwoByTwo::FourPhotosTwoByTwo;
use crate::album::photo::SinglePhoto::SinglePhoto;
use crate::album::photo::TagMarker::TagMarker;
use crate::album::photo::ThreePhotos::ThreePhotos;
use crate::album::photo::TwoPhotos::TwoPhotos;
use crate::album::photo::{Photo, PhotoContainer};
use itertools::Itertools;
use std::fs::File;
use std::io::{BufWriter, ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::{env, fs, io, mem};

pub struct Album {
    photos: Vec<Photo>,
    collected_photos: Option<Vec<Box<dyn PhotoContainer>>>,
}

impl Album {
    const ASSETS_DIR: &'static str = "assets";
    pub fn import_all_photos(path: &PathBuf) -> Result<Self, io::Error> {
        Ok(Album {
            collected_photos: None,
            photos: fs::read_dir(path)?
                .filter_map(|p| p.ok())
                .map(|p| p.path())
                .filter_map(|p| {
                    if p.extension()
                        .map_or(false, |ext| ext == "jpg" || ext == "png")
                    {
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
                .map(Photo::load_from_disk)
                .filter_map(|f| f.ok())
                .collect::<Vec<_>>(),
        })
    }
    fn is_tag_marker(photo: &Photo) -> bool {
        if photo
            .get_html_escaped_caption()
            .trim()
            .split('\n')
            .map(|line| line.split(':').next())
            .filter(|l| l.is_some())
            .map(|l| l.unwrap().trim())
            .map(|s| s.to_lowercase())
            .any(|l| vec!["gpx", "distance", "time"].iter().any(|w| l == *w))
        {
            return true;
        }
        if let Some(dayWord) = photo
            .get_html_escaped_title()
            .trim()
            .split(' ')
            .next()
            .map(|s| s.to_lowercase())
        {
            if !vec!["tag", "day", "chapter"].iter().any(|w| dayWord == *w) {
                return false;
            }
            return true; //TODO more things like GPX: or Distance: ?
        }
        false
    }

    /// Collect all photos into their appropriate containers.
    /// This will move all photos and empty the photos vector
    pub fn collect_photos(&mut self) -> () {
        if self.collected_photos.is_none() {
            self.collected_photos = Some(vec![]);
        }
        let mut stack: Vec<Photo> = vec![];
        let photos = mem::replace(&mut self.photos, vec![]);
        for photo in photos.into_iter() {
            if photo.get_html_escaped_title().trim() == "/" {
                stack.push(photo);
            } else {
                match stack.len() {
                    // Single Full-Size Photo or tag marker
                    0 => self.collected_photos.as_mut().unwrap().push(
                        if Self::is_tag_marker(&photo) {
                            Box::new(TagMarker::new(photo))
                        } else {
                            Box::new(SinglePhoto::new(photo))
                        },
                    ),
                    1 => self
                        .collected_photos
                        .as_mut()
                        .unwrap()
                        .push(Box::new(TwoPhotos::new(stack.pop().unwrap(), photo))),
                    2 => self
                        .collected_photos
                        .as_mut()
                        .unwrap()
                        .push(Box::new(ThreePhotos::new(
                            stack.pop().unwrap(),
                            stack.pop().unwrap(),
                            photo,
                        ))),
                    3 => self.collected_photos.as_mut().unwrap().push(Box::new(
                        FourPhotosTwoByTwo::new(
                            stack.pop().unwrap(),
                            stack.pop().unwrap(),
                            stack.pop().unwrap(),
                            photo,
                        ),
                    )),
                    _ => eprintln!("Unsupported Image Count: {}", stack.len()),
                }
            }
        }
    }
    fn write_aux_files(&self, path: &Path) -> io::Result<()> {
        let mut assets_path = env::current_exe()?
            .parent()
            .ok_or(io::Error::new(
                ErrorKind::Unsupported,
                "Exe Path did not have a parent!",
            ))?
            .join(Self::ASSETS_DIR);
        if !assets_path.is_dir() {
            assets_path = PathBuf::from(Self::ASSETS_DIR);
        }
        for asset in ["Makefile", "Album.css"] {
            fs::copy(assets_path.join(asset), path.join(asset))?;
        }
        Ok(())
    }

    fn print_markdown(&self, f: &mut Box<dyn Write>) -> io::Result<()> {
        write!(f, "# Test-Album\n\n")?;
        for container in self
            .collected_photos
            .as_ref()
            .expect("Please call collect_photos before calling this function!")
        {
            container.print_markdown(f)?;
        }
        Ok(())
    }
    /// Write the complete album to disk as Markdown file and copy all photos
    pub fn write_to_disk(&mut self, path: &Path) {
        if let Err(e) = self.write_aux_files(path) {
            eprintln!("{:?}", e);
        }
        // Copy all photos to the output directory.
        for photo in &mut self.photos {
            match photo.write_to_directory(path) {
                Ok(p) => {
                    *photo = p;
                },
                Err(e) => {
                    eprintln!("{:?}", e);
                },
            }
        }
        self.collect_photos();
        let out = File::create(path.join("Album.md")).unwrap();
        let mut out = BufWriter::new(out);
        let mut outBoxed: Box<dyn Write> = Box::new(out);
        self.print_markdown(&mut outBoxed).unwrap();
        for photo in &self.photos {
            println!("{}", photo)
        }
    }
}
