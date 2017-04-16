extern crate chrono;

mod input;
mod gpx;

fn main() {
    // Grab the command line options and parse them into settings for this
    // run of the application.
    let settings : input::CmdLineSettings = input::get_cmd_line_opts();

    // Open & read the GPX tracklog, producing a BTreeMap, ordered by time.
    gpx::parse(settings);
    //let gpx : gpx::GpxData = gpx::parse(settings);

    // Geotag all the photos we have.
    //photos::geotag(gpx);
}
