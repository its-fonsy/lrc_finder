use std::env;
use std::io::prelude::*;
use std::os::unix::net::UnixStream;

use crate::error::LyricFinderError;
use crate::song::Song;

type Result<T> = std::result::Result<T, LyricFinderError>;

pub struct Cmus {
    status: String,
}

const XDG_RUNTIME_DIR: &str = "XDG_RUNTIME_DIR";

impl Cmus {
    pub fn new() -> Result<Cmus> {
        let mut socket_path: String = env::var(XDG_RUNTIME_DIR)?;
        socket_path.push_str("/cmus-socket");

        let mut response = [0; 2048];
        let mut stream = UnixStream::connect(socket_path.clone())?;

        stream.write(b"status\n")?;
        stream.read(&mut response)?;

        let mut i = 0;
        while response[i] != 0 {
            i = i + 1;
        }

        let status = String::from_utf8_lossy(&response[0..i]).to_string();

        Ok(Cmus { status })
    }

    pub fn get_playing_song(&self) -> Song {
        let title = self.parse_status("tag title");
        let artist = self.parse_status("tag artist");

        Song::new(artist.as_str(), title.as_str())
    }

    fn parse_status(&self, pattern: &str) -> String {
        let mut value = String::new();

        for line in self.status.lines() {
            match line.strip_prefix(pattern) {
                Some(stripped) => {
                    value = String::from(stripped);
                    break;
                }
                None => continue,
            }
        }

        value.trim().to_string()
    }
}
