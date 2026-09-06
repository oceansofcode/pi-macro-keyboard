use std::env::home_dir;
use std::fs::{self, File, OpenOptions};
use std::path::Path;
use std::io::{Error, Write};

use tiny_http::Server;

const PORT: &str = "59285";

fn main() -> Result<(), Error> {
    let home_dir = home_dir().expect("No home");

    let home_dir = home_dir.display().to_string() + "/auth.txt";

    let auth = fs::read_to_string(home_dir).expect("No Auth File");

    let tiny_http = Server::http("0.0.0.0:".to_string() + PORT).unwrap();

    for request in tiny_http.incoming_requests() {
        for header in request.headers() {
            if header.field.as_str().eq("X-Auth") {
                if header.value.to_string().trim() == auth.trim() {
                    if request.url().trim() == "/teams-toggle" {
                        trigger_mute_unmute()?;
                        continue;
                    }
                }
            }
        }
        continue;
    }

    Ok(())
}

fn trigger_mute_unmute () -> Result<(), Error> {
    let mut keyboard_file: File = OpenOptions::new().append(true).open(Path::new("/dev/hidg0"))?;

    keyboard_file.write(b"\x03\x00\x00\x10\x00\x00\x00\x00")?;
    keyboard_file.write(b"\x00\x00\x00\x00\x00\x00\x00\x00")?;

    Ok(())
}