use crate::album::photo::{Photo, PhotoContainer};
use std::io::Write;
use std::path::Path;
use std::{io, iter};

pub struct FourPhotosTwoByTwo {
    photo1: Photo,
    photo2: Photo,
    photo3: Photo,
    photo4: Photo,
}
impl FourPhotosTwoByTwo {
    pub fn new(photo1: Photo, photo2: Photo, photo3: Photo, photo4: Photo) -> Self {
        Self {
            photo1,
            photo2,
            photo3,
            photo4,
        }
    }
}

impl PhotoContainer for FourPhotosTwoByTwo {
    fn print_markdown(&self, f: &mut Box<dyn Write>) -> std::io::Result<()> {
        writeln!(f, "<div class=\"imageblock multirow twoimages\">")?;
        writeln!(f, "<div class=\"imagerow\">")?;

        // Image 1
        writeln!(f, "<div class=\"image\">")?;
        writeln!(f)?;
        self.photo1.print_markdown(f)?;
        writeln!(f)?;
        writeln!(f, "</div>")?; // image

        // Image 2
        writeln!(f, "<div class=\"image\">")?;
        writeln!(f)?;
        self.photo2.print_markdown(f)?;
        writeln!(f)?;
        writeln!(f, "</div>")?; // image

        writeln!(f, "</div>")?; // imagerow
        writeln!(f, "<div class=\"imagerow\">")?;

        // Image 3
        writeln!(f, "<div class=\"image\">")?;
        writeln!(f)?;
        self.photo3.print_markdown(f)?;
        writeln!(f)?;
        writeln!(f, "</div>")?; // image

        // Image 4
        writeln!(f, "<div class=\"image\">")?;
        writeln!(f)?;
        self.photo4.print_markdown(f)?;
        writeln!(f)?;
        writeln!(f, "</div>")?; // image

        writeln!(f, "</div>")?; // imagerow

        writeln!(
            f,
            "<div class=\"imagetext\">{}</div>",
            self.photo4.get_html_escaped_title()
        )?;
        writeln!(f, "</div>")?; //imageblock
        writeln!(f)?;
        let caption = self.photo4.get_html_escaped_caption();
        if !caption.is_empty() {
            writeln!(f, "<div class=\"textblock fullsizetext forimage\">")?;
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
                Ok(photo2) => match self.photo3.write_to_directory(target) {
                    Ok(photo3) => match self.photo4.write_to_directory(target) {
                        Ok(photo4) => Ok(Box::new(FourPhotosTwoByTwo::new(
                            photo1, photo2, photo3, photo4,
                        ))),
                        Err(e) => Err(e),
                    },
                    Err(e) => Err(e),
                },
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }

    fn photos(&self) -> Box<dyn Iterator<Item = &Photo> + '_> {
        Box::new(
            iter::once(&self.photo1)
                .chain(iter::once(&self.photo2))
                .chain(iter::once(&self.photo3))
                .chain(iter::once(&self.photo4)),
        )
    }
}
