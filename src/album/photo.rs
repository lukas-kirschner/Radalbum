use std::ffi::{OsStr, OsString};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::{fs, io};
use std::io::ErrorKind;
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
    pub(crate) fn get_html_escaped_caption(&self) -> String {
        self.description.clone().trim().to_string()
    }
}

impl Photo {
    pub(crate) fn get_html_escaped_title(&self) -> String {
        self.heading.clone().trim().to_string()
    }
}

impl Photo {
    pub(crate) fn get_relative_path(&self) -> PathBuf {
        let foldername = self.source.parent().map(|p| p.file_name()).unwrap().unwrap();
        let filename = self.source.file_name().unwrap();
        PathBuf::from(foldername).join(filename)
    }
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
    fn normalize_filename(&self, filename: &OsStr) -> OsString {
        filename.to_os_string() //TODO
    }

    /// Write this photo into an "img" subfolder of the given folder.
    /// If there already exists a photo with the same name, this will change the output name of
    /// the photo to a unique name.
    /// Special characters in the source images will be truncated to underscores in the target image.
    pub fn write_to_directory(&self, target: &PathBuf) -> io::Result<Self> {
        let filename = self.source.file_name().ok_or(io::Error::new(ErrorKind::Unsupported, "Directories are not supported as photos!"))?;
        let out_path = target.join("img").join(self.normalize_filename(filename));
        fs::create_dir_all(out_path.parent().unwrap())?;
        fs::copy(&self.source, &out_path)?;
        Ok(Photo {
            heading: self.heading.clone(),
            description: self.description.clone(),
            source: out_path,
        })
    }
}

impl Display for Photo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}\n{}\n{}", &self.source, &self.heading, &self.description)
    }
}