pub(crate) mod SinglePhoto;
pub(crate) mod TagMarker;
pub(crate) mod ThreePhotos;
pub(crate) mod TwoPhotos;

use crate::album::photo::PhotoLoadingError::{ExifParseError, IOError};
use rexiv2::Rexiv2Error;
use std::ffi::{OsStr, OsString};
use std::fmt::{Display, Formatter};
use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::{fs, io};

pub enum PhotoLoadingError {
    IOError { e: io::Error },
    ExifParseError { e: Rexiv2Error },
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

impl Display for PhotoLoadingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IOError { e } => write!(f, "IOError: {}", e),
            ExifParseError { e } => write!(f, "IOError: {}", e),
        }
    }
}
#[derive(Clone)]
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
        let foldername = self
            .source
            .parent()
            .map(|p| p.file_name())
            .unwrap()
            .unwrap();
        let filename = self.source.file_name().unwrap();
        PathBuf::from(foldername).join(filename)
    }
}

impl Photo {
    pub fn load_from_disk(source: PathBuf) -> Result<Self, PhotoLoadingError> {
        let exif = rexiv2::Metadata::new_from_path(&source)?;
        let heading = exif
            .get_tag_string("Iptc.Application2.ObjectName")
            .unwrap_or("".to_string());
        let description = exif
            .get_tag_string("Iptc.Application2.Caption")
            .unwrap_or("".to_string());
        Ok(Photo {
            heading,
            description,
            source,
        })
    }
    fn normalize_filename(&self, filename: &OsStr) -> OsString {
        //TODO OSString may lose invalid bytes in the process!
        OsString::from(
            filename
                .to_string_lossy()
                .chars()
                .map(|c| match c {
                    ' ' => '_',
                    'ä' => 'a',
                    'ö' => 'o',
                    'ü' => 'u',
                    'Ä' => 'A',
                    'Ö' => 'O',
                    'Ü' => 'U',
                    'ß' => 's',
                    'é' => 'e',
                    'É' => 'E',
                    '!' => '_',
                    'ł' => 'l',
                    x => x,
                })
                .collect::<String>(),
        )
    }

    /// Write this photo into an "img" subfolder of the given folder.
    /// If there already exists a photo with the same name, this will change the output name of
    /// the photo to a unique name.
    /// Special characters in the source images will be truncated to underscores in the target image.
    pub fn write_to_directory(&self, target: &Path) -> io::Result<Self> {
        let filename = self.source.file_name().ok_or(io::Error::new(
            ErrorKind::Unsupported,
            "Directories are not supported as photos!",
        ))?;
        let out_path = target.join("img").join(self.normalize_filename(filename));
        fs::create_dir_all(out_path.parent().unwrap())?;
        fs::copy(&self.source, &out_path)?;
        Ok(Photo {
            heading: self.heading.clone(),
            description: self.description.clone(),
            source: out_path,
        })
    }
    fn print_markdown(&self, f: &mut Box<dyn Write>) -> std::io::Result<()> {
        writeln!(
            f,
            "![Missing Image: {0}]({0})",
            self.get_relative_path()
                .into_os_string()
                .into_string()
                .map_err(|_| io::Error::new(
                    ErrorKind::InvalidData,
                    "Invalid Path in image detected!"
                ))?
        )?;
        Ok(())
    }
}

impl Display for Photo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}\n{}\n{}",
            &self.source, &self.heading, &self.description
        )
    }
}

pub trait PhotoContainer {
    /// Write everything out as Markdown file.
    /// The generated markdown file contains the paths stored inside this object.
    /// If photos need to be copied before generating the markdown file,
    /// call write_to_directory before calling this function!
    fn print_markdown(&self, f: &mut Box<dyn Write>) -> io::Result<()>;
    /// Write all photos to the given directory.
    /// Returns a copy of itself if successful, which contains the updated paths.
    /// All print calls must be done on the copy in order to make sure the paths match.
    fn write_to_directory(&self, target: &Path) -> io::Result<Box<dyn PhotoContainer>>;
    fn photos(&self) -> Box<dyn Iterator<Item = &Photo> + '_>;
}
