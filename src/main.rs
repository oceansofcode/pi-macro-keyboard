use std::env::home_dir;
use std::fs::{self, File, OpenOptions};
use std::path::Path;
use std::io::{Error, Write};
use std::{thread, time};

use tiny_http::Server;

const AUTH_HEADER: &str = "X-Auth";
const AUTH_FILE: &str = "/auth.txt";

const PORT: &str = "59285";

const TOGGLE_MUTE_ENDPOINT: &str = "/teams-toggle";
const TOGGLE_END_CALL_ENDPOINT: &str = "/end-call";

fn main() -> Result<(), Error> {
    let home_dir = home_dir().expect("No home");

    let home_dir = home_dir.display().to_string() + AUTH_FILE;

    let auth = fs::read_to_string(home_dir).expect("No Auth File");

    let tiny_http = Server::http(format!("[::]:{}", PORT)).unwrap();

    for request in tiny_http.incoming_requests() {
        for header in request.headers() {
            if header.field.as_str().eq(AUTH_HEADER) {
                if header.value.to_string().trim() == auth.trim() {
                    if request.url().trim() == TOGGLE_MUTE_ENDPOINT {
                        trigger_keyboard(b"\x03\x00\x10\x00\x00\x00\x00\x00")?;
                        continue;
                    } else if request.url().trim() == TOGGLE_END_CALL_ENDPOINT {
                        trigger_keyboard(b"\x03\x00\x0b\x00\x00\x00\x00\x00")?;
                        continue;
                    }
                }
            }
        }
        thread::sleep(time::Duration::from_secs(1));
    }

    Ok(())
}

fn trigger_keyboard (bytes: &[u8]) -> Result<(), Error> {
     let mut keyboard_file: File = OpenOptions::new().append(true).open(Path::new("/dev/hidg0"))?;

    keyboard_file.write(bytes)?;
    keyboard_file.write(b"\x00\x00\x00\x00\x00\x00\x00\x00")?;

    Ok(())   
}