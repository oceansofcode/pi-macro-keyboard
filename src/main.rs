
use std::fs::{write, OpenOptions, File};
use std::path::Path;
use std::io::{Error, Write};

fn main() -> Result<(), Error> {
    //let path = Path::new("/dev/hidg0");
    let path = Path::new("test");

    // write!(keyboard_file, "\\x03\\x00\\x00\\x10\\x00\\x00\\x00\\x00\\x00");
    // write!(keyboard_file, "\\x00\\x00\\x00\\x00\\x00\\x00\\x00\\x00");
    let mut keyboard_file: File = OpenOptions::new().append(true).open(path)?;
    keyboard_file.write(b"\\x03\\x00\\x00\\x10\\x00\\x00\\x00\\x00\\x00")?;
    keyboard_file.write( b"\\x00\\x00\\x00\\x00\\x00\\x00\\x00\\x00")?;

    //write(path, b"\\x03\\x00\\x00\\x10\\x00\\x00\\x00\\x00\\x00")?;
    //write(path, b"\\x00\\x00\\x00\\x00\\x00\\x00\\x00\\x00")?;

    Ok(())
}