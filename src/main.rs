use std::env;
use std::fs;
use std::io::Write;

mod cmus;
mod error;
mod ranker;
mod song;

use crate::cmus::Cmus;
use crate::error::LyricFinderError;
use crate::ranker::{SongRank, SongRanker};
use crate::song::Song;

type Result<T> = std::result::Result<T, LyricFinderError>;

const OLD_LYRIC_DIRECTORY: &str = "/media/music/archive/old_lyrics";
const ENV_LYRIC_DIR: &str = "LYRICS_DIR";

macro_rules! user_confirm_or_abort {
    ($ans:expr) => {
        user_confirm_or_abort!($ans, "Aborted.")
    };
    ($ans:expr, $msg:expr) => {
        if $ans != "y" && $ans != "yes" {
            println!("{}", $msg);
            return Ok(());
        }
    };
}

fn inline_dialog(question: &str) -> Result<String> {
    let mut answer = String::new();

    print!("{}", question);
    std::io::stdout().flush()?;

    answer.clear();
    std::io::stdin().read_line(&mut answer)?;

    Ok(answer.trim().to_string())
}

fn run() -> Result<()> {
    /* Get playing songw from music player */

    let player: Cmus = Cmus::new()?;
    let song_to_match: Song = player.get_playing_song();

    /* Initialize the rank list */

    let mut ranker: SongRanker = SongRanker::new(&song_to_match);

    /* Check every song from lyric directory */

    let path: fs::ReadDir = fs::read_dir(OLD_LYRIC_DIRECTORY)?;

    for entry in path {
        let filename: String = entry?.file_name().into_string()?;
        let (artist, title): (&str, &str) = match filename.split_once("-") {
            Some(res) => res,
            None => continue,
        };

        /* Remove extension from filename */

        let title: &str = match title.rsplit_once('.') {
            Some(res) => res.0,
            None => continue,
        };

        let song: Song = Song::new(artist, title);
        ranker.rank(song, filename);
    }

    /* Ask user to add lyric to the repository */

    ranker.print_rank_list();
    let mut user_answer: String = inline_dialog("Add a lyric to repository (y/N)? ")?;
    user_confirm_or_abort!(user_answer);

    /* Ask user which from the rank list */

    user_answer = inline_dialog("Select position: ")?;
    let position: usize = user_answer.parse()?;
    let song_to_copy: SongRank = ranker.get_pos(position)?;

    /* Set source and destination file names */

    let dst_lrc_folder: String = env::var(ENV_LYRIC_DIR)?.trim_end_matches('/').to_string();
    let src_lrc_full_path: String = OLD_LYRIC_DIRECTORY.to_owned() + "/" + &song_to_copy.filename;
    let dst_lrc_full_path: String = dst_lrc_folder + "/" + song_to_match.hased_filename().as_str();

    /* Ask user to confirm copy */

    println!("Copy");
    println!("  source: {}", src_lrc_full_path);
    println!("    dest: {}", dst_lrc_full_path);

    user_answer = inline_dialog("Confirm (y/N)? ")?.to_lowercase();
    user_confirm_or_abort!(user_answer);

    /* User confirmed, copy execute the copy */

    match fs::copy(&src_lrc_full_path, &dst_lrc_full_path) {
        Ok(_) => println!("Copied successfully"),
        Err(e) => eprintln!("Error copying file: {}", e),
    }

    Ok(())
}

fn main() {
    if let Err(error_message) = run() {
        eprintln!("Error: {}", error_message);
        std::process::exit(1);
    }
}
