use crate::album::photo::{Photo, PhotoContainer};
use std::io::{ErrorKind, Write};
use std::path::Path;
use std::{io, iter};

pub struct SinglePhoto {
    photo: Photo,
}
impl SinglePhoto {
    pub fn new(photo: Photo) -> Self {
        Self { photo }
    }
}

impl PhotoContainer for SinglePhoto {
    fn print_markdown(&self, f: &mut Box<dyn Write>) -> std::io::Result<()> {
        writeln!(f, "<div class=\"imageblock fullsize\">")?;
        writeln!(f, "<div class=\"image\">")?;
        writeln!(f)?;
        self.photo.print_markdown(f)?;
        writeln!(f)?;
        writeln!(
            f,
            "<div class=\"imagetext\">{}</div>",
            self.photo.get_html_escaped_title()
        )?;
        writeln!(f, "</div>")?;
        writeln!(f, "</div>")?;
        writeln!(f)?;
        let caption = self.photo.get_html_escaped_caption();
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
        match self.photo.write_to_directory(target) {
            Ok(photo) => Ok(Box::new(SinglePhoto::new(photo))),
            Err(e) => Err(e),
        }
    }

    fn photos(&self) -> Box<dyn Iterator<Item = &Photo> + '_> {
        Box::new(iter::once(&self.photo))
    }
}
