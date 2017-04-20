extern crate chrono;

mod input;
mod gpx;
mod photos;

fn main() {
    // Grab the command line options and parse them into settings for this
    // run of the application.
    let settings : input::CmdLineSettings = input::get_cmd_line_opts();

    // Open & read the GPX tracklog, producing a vector of BTreeMaps, each containing a
    // track segment.
    if let Err(gpx_data_err) = gpx::get(settings) {
        // panic?
    }

    // Geotag all the photos we have with the GPS data we've extracted.
    //photos::geotag(gpx);
}
