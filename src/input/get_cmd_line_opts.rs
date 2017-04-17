use std::env;
use super::CmdLineSettings;

///
/// Gets the command line options that the user provided to run the application with.
///
/// These command line options can include the path to a .gpx tracklog file (or a directory of
/// them), a path to a directory of images to be geotagged, and a note to append to each geotagged
/// image.
///
/// Returns `CmdLineSettings` - A struct representing the options passed in to the application.
///
pub fn get_cmd_line_opts() -> CmdLineSettings {
    let mut settings : CmdLineSettings = CmdLineSettings::new();

    let mut args = env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_ref() {
            "-t" | "--tracks" => match args.next() {
                Some(a) => settings.set_path_to_tracks(&a),
                None => break
            },
            "-p" | "--photos" => match args.next() {
                Some(a) => settings.set_path_to_photos(&a),
                None => break
            },
            "-n" | "--note" => match args.next() {
                Some(a) => settings.set_note(&a),
                None => break
            },
            _ => break
        }
    }

    settings
}
