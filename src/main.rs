use std::env::home_dir;
use std::fs::{self, File, OpenOptions};
use std::path::Path;
use std::io::{Error, Write};

use tiny_http::Server;

fn main() -> Result<(), Error> {

    let home_dir = home_dir().expect("No home?!");

    const AUTH_FILE: &str = "auth.txt";

    let home_dir = home_dir.display().to_string() + "/" + AUTH_FILE;

    let home_dir = Path::new(&home_dir);

    let auth = fs::read_to_string(home_dir).expect("No Auth File");

    let tiny_http = Server::http("0.0.0.0:7878").unwrap();

    for request in tiny_http.incoming_requests() {
        let headers = request.headers();

        println!("request {request:#?}");
        
        for header in headers {

            if header.field.as_str().eq("X-Auth") {
                let value = header.value.to_string();
                let auth_check = value.trim() == auth.trim();

                if auth_check {
                    println!("request!");
                    trigger_mute_unmute()?;
                }
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