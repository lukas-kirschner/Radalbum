use crate::album::photo::{Photo, PhotoContainer};
use std::io::{ErrorKind, Write};
use std::path::Path;
use std::{io, iter};

pub struct TwoPhotos {
    photo1: Photo,
    photo2: Photo,
}
impl TwoPhotos {
    pub fn new(photo1: Photo, photo2: Photo) -> Self {
        Self { photo1, photo2 }
    }
}

impl PhotoContainer for TwoPhotos {
    fn print_markdown(&self, f: &mut Box<dyn Write>) -> std::io::Result<()> {
        writeln!(f, "<div class=\"imageblock twoimages\">")?;
        // Image 1
        writeln!(f, "<div class=\"image\">")?;
        writeln!(f)?;
        writeln!(
            f,
            "![Missing Image: {0}]({0})",
            self.photo1
                .get_relative_path()
                .into_os_string()
                .into_string()
                .map_err(|_| io::Error::new(
                    ErrorKind::InvalidData,
                    "Invalid Path in image detected!"
                ))?
        )?;
        writeln!(f)?;
        writeln!(f, "</div>")?;
        // Image 2
        writeln!(f, "<div class=\"image\">")?;
        writeln!(f)?;
        writeln!(
            f,
            "![Missing Image: {0}]({0})",
            self.photo2
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
            self.photo2.get_html_escaped_title()
        )?;
        writeln!(f, "</div>")?;

        writeln!(f, "</div>")?;
        writeln!(f)?;
        let caption = self.photo2.get_html_escaped_caption();
        if !caption.is_empty() {
            writeln!(f, "<div class=\"textblock fullsizetext\">")?;
            writeln!(f)?;
            writeln!(f, "{}", caption)?;
            writeln!(f)?;
            writeln!(f, "</div>")?;
            writeln!(f)?;
        }
        Ok(())
    }

    fn write_to_directory(&self, target: &Path) -> io::Result<Box<dyn PhotoContainer>> {
        match self.photo1.write_to_directory(target) {
            Ok(photo1) => match self.photo2.write_to_directory(target) {
                Ok(photo2) => Ok(Box::new(TwoPhotos::new(photo1, photo2))),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }

    fn photos(&self) -> Box<dyn Iterator<Item = &Photo> + '_> {
        Box::new(iter::once(&self.photo1).chain(iter::once(&self.photo2)))
    }
}
