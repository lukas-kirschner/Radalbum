use std::path::PathBuf;

pub struct Album {}

impl Album {
    pub fn import_all_photos(&mut self, path: &PathBuf) {}
    pub fn write_to_disk(&self, path: &PathBuf) {}
}