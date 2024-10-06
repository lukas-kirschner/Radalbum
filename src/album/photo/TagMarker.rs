use crate::album::photo::{Photo, PhotoContainer};
use itertools::Itertools;
use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::{io, iter};

/// A chapter (Day X) for a Rad-Album, marking a day of travel, together with special data fields.
#[derive(Clone)]
pub struct TagMarker {
    photo: Option<Photo>,
    gpxfile: Option<PathBuf>,
    distance: Option<String>,
    ascent: Option<String>,
    descent: Option<String>,
    moving_time: Option<String>,
    avg_speed: Option<String>,
    dest_from: Option<String>,
    dest_to: Option<String>,
    title: String,
}
impl TagMarker {
    fn strip_unit(s: &str, u: &str) -> String {
        s.to_lowercase()
            .strip_suffix(u)
            .unwrap_or(s)
            .trim()
            .to_string()
    }
    pub fn new(photo: Photo) -> Self {
        let mut ret = Self {
            photo: None,
            gpxfile: None,
            distance: None,
            ascent: None,
            descent: None,
            moving_time: None,
            avg_speed: None,
            dest_from: None,
            dest_to: None,
            title: photo.get_html_escaped_title(),
        };
        for line in photo
            .get_html_escaped_caption()
            .split(&['\r', '\n'][..])
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
        {
            let parts = line
                .split(':')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect_vec();
            if parts.len() != 2 || parts[1].is_empty() {
                continue;
            }
            match parts[0].to_lowercase().as_str() {
                "gpx" => ret.gpxfile = Some(PathBuf::from(parts[1])),
                "distance" => ret.distance = Some(Self::strip_unit(parts[1], "km").to_string()),
                "ascent" => ret.ascent = Some(Self::strip_unit(parts[1], "m").to_string()),
                "descent" => ret.descent = Some(Self::strip_unit(parts[1], "m").to_string()),
                "moving time" => ret.moving_time = Some(parts[1].to_string()),
                "avg" => ret.avg_speed = Some(Self::strip_unit(parts[1], "km/h").to_string()),
                "from" => ret.dest_from = Some(parts[1].to_string()),
                "to" => ret.dest_to = Some(parts[1].to_string()),
                _ => {
                    eprintln!("Unmatched Part in day header: {}", parts[0])
                },
            };
        }
        ret.photo = Some(photo); // TODO Read GPX File instead and show track using JavaScript?
        ret
    }
}

impl PhotoContainer for TagMarker {
    fn print_markdown(&self, f: &mut Box<dyn Write>) -> std::io::Result<()> {
        writeln!(f, "<div class=\"dayheader\">")?;
        writeln!(f)?;
        writeln!(f, "## {}", self.title)?;
        writeln!(f)?;
        if let Some(dest_from) = &self.dest_from {
            writeln!(f, "<div class=\"destfrom\">{}</div>", dest_from)?;
        }
        if let Some(dest_to) = &self.dest_to {
            writeln!(f, "<div class=\"destto\">{}</div>", dest_to)?;
        }
        if let Some(ascent) = &self.ascent {
            writeln!(f, "<div class=\"ascent\">{}</div>", ascent)?;
        }
        if let Some(descent) = &self.descent {
            writeln!(f, "<div class=\"descent\">{}</div>", descent)?;
        }
        if let Some(distance) = &self.distance {
            writeln!(f, "<div class=\"distance\">{}</div>", distance)?;
        }
        if let Some(moving_time) = &self.moving_time {
            writeln!(f, "<div class=\"time\">{}</div>", moving_time)?;
        }
        if let Some(avg_speed) = &self.avg_speed {
            writeln!(f, "<div class=\"speed\">{}</div>", avg_speed)?;
        }
        if let Some(photo) = &self.photo {
            writeln!(f)?;
            writeln!(f, "<div class=\"image\">")?;
            writeln!(f)?;
            photo.print_markdown(f)?;
            writeln!(f)?;
            writeln!(f, "</div>")?; //image
        }
        writeln!(f)?;
        writeln!(f, "</div>")?; //dayheader
        writeln!(f)?;
        Ok(())
    }

    fn write_to_directory(&self, target: &Path) -> io::Result<Box<dyn PhotoContainer>> {
        if let Some(photo) = &self.photo {
            match photo.write_to_directory(target) {
                Ok(photo) => Ok(Box::new(TagMarker::new(photo))),
                Err(e) => Err(e),
            }
        } else {
            Ok(Box::new(self.clone()))
        }
    }

    fn photos(&self) -> Box<dyn Iterator<Item = &Photo> + '_> {
        if let Some(photo) = &self.photo {
            Box::new(iter::once(photo))
        } else {
            Box::new(iter::empty())
        }
    }
}
