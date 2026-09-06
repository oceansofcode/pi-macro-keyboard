use std::fs::{OpenOptions, File};
use std::path::Path;
use std::io::{Error, Write};

use tiny_http::Server;

fn main() -> Result<(), Error> {
    let tiny_http = Server::http("0.0.0.0:7878").unwrap();

    for request in tiny_http.incoming_requests() {
        let headers = request.headers();
        
        for header in headers {

            if header.field.as_str().eq("X-Auth") && header.value == "test" {
                trigger_mute_unmute()?;
            }
        }
    }


    Ok(())
}

fn trigger_mute_unmute () -> Result<(), Error> {

    let path: &Path = Path::new("/dev/hidg0");
    let mut keyboard_file: File = OpenOptions::new().append(true).open(path)?;

    keyboard_file.write(b"\x03\x00\x00\x10\x00\x00\x00\x00")?;
    keyboard_file.write(b"\x00\x00\x00\x00\x00\x00\x00\x00")?;

    Ok(())
}