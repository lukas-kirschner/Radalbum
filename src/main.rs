mod album;

use std::fs;
use std::path::PathBuf;

use clap::{Parser};
use crate::album::Album;

/// Radalbum - Create a photo album from a set of images automatically using metadata stored inside the image files.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct RadalbumArgs {
    /// Input Directory, containing the source images that should be used in the generated album.
    /// Source images will always be sorted alphabetically by their file path.
    input_directory: PathBuf,

    /// Sets the output directory that will be used to store the generated photo album.
    /// If the directory does not exist yet, it will be created.
    #[arg(short, long, value_name = "DIRECTORY")]
    out: Option<PathBuf>,
}

fn main() {
    let cli = RadalbumArgs::parse();

    let input_directory = cli.input_directory;
    println!("Value for input directory: {input_directory:?}");

    let mut album = Album::import_all_photos(&input_directory).unwrap();

    if let Some(config_path) = cli.out {
        println!("Value for output directory: {}", config_path.display());
        if !config_path.is_dir() {
            if let Err(e) = fs::create_dir(&config_path) {
                eprintln!("{:?}", e);
            }
        }
        album.write_to_disk(&config_path);
    }
}