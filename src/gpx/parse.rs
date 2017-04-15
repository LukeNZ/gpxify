use input;
use std;
use super::GpxData;

pub fn parse(settings: input::CmdLineSettings) {
    println!("{}", std::env::current_dir().unwrap().display());
}
