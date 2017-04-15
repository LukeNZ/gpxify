use std::env::current_dir;
use std::path::PathBuf;

#[derive(Debug)]
pub struct CmdLineSettings {
    path_to_tracks: Option<PathBuf>,
    path_to_photos: Option<PathBuf>,
    note: Option<String>
}

impl CmdLineSettings {

    ///
    /// Creates a new CmdLineSettings struct.
    ///
    pub fn new() -> CmdLineSettings {
        CmdLineSettings { path_to_tracks: None, path_to_photos: None, note: None }
    }

    ///
    /// Sets the path to a .gpx tracklog file, or a directory containing .gpx tracklog files, that
    /// should be read from.
    ///
    /// `path_to_tracks` &str - The path to the .gpx tracklog or tracklog directory.
    ///
    pub fn set_path_to_tracks(&mut self, path_to_tracks: &str) {
        self.path_to_tracks = Some(PathBuf::from(path_to_tracks));
    }

    ///
    /// Sets the path to a directory of photos to be geotagged.
    ///
    /// `path_to_photos` &str - The path to the directory of photos to geotag.
    ///
    pub fn set_path_to_photos(&mut self, path_to_photos: &str) {
        self.path_to_photos = Some(PathBuf::from(path_to_photos));
    }

    ///
    /// Sets the note that should be appeneded to each geotagged image.
    ///
    /// `note` &str - The note to be appended.
    ///
    pub fn set_note(&mut self, note: &str) {
        self.note = Some(note.to_string());
    }

    ///
    /// Gets the path to a .gpx tracklog file, or a directory containing .gpx tracklog files, that
    /// should be read from. If the tracklog file or directory was not specified by the user as an
    /// argument, use the current directory of script execution.
    ///
    /// Returns a path buffer representing the path to a .gpx tracklog file or a directory containing
    /// .gpx tracklog files.
    ///
    pub fn get_path_to_tracks(self) -> PathBuf {
        self.path_to_tracks.unwrap_or(current_dir().unwrap())
    }

    ///
    /// Gets the path to a directory of photos to be geotagged. If the photo directory was not specified
    /// by the user as an argument, use the current directory of script execution.
    ///
    /// Returns a path buffer representing the path to a directory of photos to be geotagged.
    ///
    pub fn get_path_to_photos(self) -> PathBuf {
        self.path_to_tracks.unwrap_or(current_dir().unwrap())
    }

    ///
    /// Gets the note that should be appended to each geotagged image.
    ///
    /// Returns a string representing the note to be appended to each geotagged image.
    ///
    pub fn get_note(self) -> String {
        self.note.unwrap_or(String::new())
    }
}
