mod photo;

use crate::album::photo::Photo;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufWriter, ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::{env, fs, io};

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

    fn print_markdown<W: Write>(&self, f: &mut W) -> io::Result<()> {
        write!(f, "# Test-Album\n\n")?;
        for photo in &self.photos {
            writeln!(f, "<div class=\"imageblock fullsize\">")?;
            writeln!(f, "<div class=\"image\">")?;
            writeln!(f)?;
            writeln!(
                f,
                "![Missing Image: {0}]({0})",
                photo
                    .get_relative_path()
                    .into_os_string()
                    .into_string()
                    .map_err(|_| io::Error::new(
                        ErrorKind::InvalidData,
                        "Invalid Path in image detected!"
                    ))?
            )?;
            writeln!(f)?;
            writeln!(
                f,
                "<div class=\"imagetext\">{}</div>",
                photo.get_html_escaped_title()
            )?;
            writeln!(f, "</div>")?;
            writeln!(f, "</div>")?;
            writeln!(f)?;
            let caption = photo.get_html_escaped_caption();
            if !caption.is_empty() {
                writeln!(f, "<div class=\"textblock fullsizetext\">")?;
                writeln!(f)?;
                writeln!(f, "{}", caption)?;
                writeln!(f)?;
                writeln!(f, "</div>")?;
                writeln!(f)?;
            }
        }
        Ok(())
    }
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
        let out = File::create(path.join("Album.md")).unwrap();
        let mut out = BufWriter::new(out);
        self.print_markdown(&mut out).unwrap();
        for photo in &self.photos {
            println!("{}", photo)
        }
    }
}
