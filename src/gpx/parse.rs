use input;
use std;
use std::path::{PathBuf, Path};
use std::io::Error;
use std::fs::ReadDir;
use std::ffi::OsStr;
use super::GpxTrackPoint;

///
/// Parse all the GPX files.
///
pub fn parse(settings: input::CmdLineSettings) {
    let gps_track_path : PathBuf = settings.get_path_to_tracks();

    if let Ok(paths_to_gpx_files) = get_all_paths_to_gpx_files(gps_track_path.as_ref()) {

    }
}

///
/// Get all the paths for all the GPX tracks for the provided directory.
///
fn get_all_paths_to_gpx_files(path_to_directory: &Path) -> Result<Vec<PathBuf>, Error> {
    let files : Result<ReadDir, std::io::Error> = std::fs::read_dir(path_to_directory);

    if let Ok(paths) = files {
        let foo : Vec<PathBuf> = paths
            .filter_map(Result::ok)
            .filter(|p| p.path().extension() == Some(OsStr::new("gpx")))
            .map(|p| p.path())
            .collect::<Vec<PathBuf>>();

        Ok(foo)
    } else {
        Err(files.unwrap_err())
    }
}

///
///
///
fn get_track_from_file(path_to_file: &Path) {

}
