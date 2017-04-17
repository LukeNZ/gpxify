use input;
use std;
use std::path::{PathBuf, Path};
use std::io::{BufReader, Bytes, Error, Read};
use std::fs::{File, ReadDir};
use std::ffi::OsStr;
use super::{GpxTrackSegment};

///
/// Get the contents of all the GPX files.
///
pub fn get(settings: input::CmdLineSettings) /*-> Result<Vec<GpxTrackSegment>, Error> */{
    let gps_track_path : PathBuf = settings.get_path_to_tracks();

    if let Ok(paths_to_gpx_files) = get_all_paths_to_gpx_files(gps_track_path.as_ref()) {
        let gpx_files : Vec<String> = paths_to_gpx_files
            .iter()
            .map(|p| get_track_from_file(&p))
            .filter_map(Result::ok)
            .collect::<Vec<String>>();

    } else {
        // Indicate gpx file(s) could not be read.
        Err(paths_to_gpx_files)
    }
}

///
/// Get all the paths for all the GPX tracks for the provided directory.
///
/// For the directory provided, opens and reads through all the files, filtering by files which are
/// listed as having a `.gpx` file extension. These are then returned to the callee as a vector of
/// `PathBuf`'s wrapped in a `Result`. If the directory provided by the callee encountered an error,
/// return that `Err`.
///
/// `path_to_directory` &Path - The directory to iterate over to find .gpx files.
///
/// Returns `Result<Vec<PathBuf>, Error>` - a collection of PathBuf's if successful, otherwise returns
///     and error.
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
/// For a given path to a GPX file, return the contents of that file as a string.
///
/// Opens a file and reads it in into a buffered reader, before parsing it into a string and returning
/// the result with the wrapped string, or an error if there was a problem opening the file.
///
/// `path_to_file` &Path - The path of the file to read into a string.
///
/// Returns `Result<String, Error>`
///
fn get_track_from_file(path_to_file: &Path) -> Result<String, Error> {
    let file = File::open(path_to_file);
    if let Ok(foo) = file {
        let mut contents = String::new();
        let reader = BufReader::new(foo);
        parse_gpx_file(reader);
        Ok(contents)
    } else {
        Err(file.unwrap_err())
    }
}
