use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io;
use std::path::PathBuf;
use rexiv2::Rexiv2Error;
use crate::album::photo::PhotoLoadingError::{ExifParseError, IOError, MissingExifError};

pub enum PhotoLoadingError {
    IOError { e: io::Error },
    ExifParseError { e: Rexiv2Error },
    MissingExifError,
}
impl From<io::Error> for PhotoLoadingError {
    fn from(value: io::Error) -> Self {
        IOError { e: value }
    }
}
impl From<Rexiv2Error> for PhotoLoadingError {
    fn from(value: Rexiv2Error) -> Self {
        ExifParseError { e: value }
    }
}

pub struct Photo {
    heading: String,
    description: String,
    source: PathBuf,
}

impl Photo {
    pub fn load_from_disk(source: PathBuf) -> Result<Self, PhotoLoadingError> {
        let exif = rexiv2::Metadata::new_from_path(&source)?;
        let heading = exif.get_tag_string("Iptc.Application2.ObjectName").unwrap_or("".to_string());
        let description = exif.get_tag_string("Iptc.Application2.Caption").unwrap_or("".to_string());
        return Ok(Photo {
            heading,
            description,
            source,
        });
    }
}

impl Display for Photo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}\n{}\n{}", &self.source, &self.heading, &self.description)
    }
}