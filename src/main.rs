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

    let best_match: SongRank = ranker.get_pos(1)?;

    println!(
        "The best match for '{}' by '{}' is:",
        song_to_match.title, song_to_match.artist
    );
    println!();
    println!("       Artist: {}", best_match.song.artist);
    println!("        Title: {}", best_match.song.title);
    println!("     Filename: {}", best_match.filename);
    println!("   Diff score: {}", best_match.score);
    println!();

    /* Set source and destination file names */

    let dst_lrc_folder: String = env::var(ENV_LYRIC_DIR)?.trim_end_matches('/').to_string();
    let src_lrc_full_path: String = OLD_LYRIC_DIRECTORY.to_owned() + "/" + &best_match.filename;
    let dst_lrc_full_path: String = dst_lrc_folder + "/" + song_to_match.hased_filename().as_str();

    /* Ask user to confirm copy */

    println!("Copy the lyric from:");
    println!("       Source: {}", src_lrc_full_path);
    println!("  Destination: {}", dst_lrc_full_path);
    println!();

    let user_answer: String = inline_dialog("Confirm (y/N)? ")?.to_lowercase();
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
