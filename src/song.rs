use md5;

#[derive(Clone, Default)]
pub struct Song {
    pub artist: String,
    pub title: String,
}

impl Song {
    pub fn new(artist: &str, title: &str) -> Song {
        let artist: String = String::from(artist);
        let title: String = String::from(title);

        Song { artist, title }
    }

    pub fn generate_token(&self) -> String {
        let artist: String = Song::sanitize(self.artist.as_str());
        let title: String = Song::sanitize(self.title.as_str());

        format!("{}{}", artist, title).to_string()
    }

    fn sanitize(input: &str) -> String {
        let mut sanitized = String::new();

        for char in input.chars() {
            if char.is_alphanumeric() {
                sanitized.push_str(&char.to_lowercase().to_string());
            }
        }

        sanitized
    }

    pub fn hased_filename(&self) -> String {
        let digest = md5::compute(format!("{}{}", self.artist, self.title).as_bytes());
        format!("{:x}.lrc", digest).to_string()
    }
}
