use crate::song::Song;

use crate::error::LyricFinderError;

type Result<T> = std::result::Result<T, LyricFinderError>;

const RANK_LIST_SIZE: usize = 5;

#[derive(Clone)]
pub struct SongRank {
    song: Song,
    score: usize,
    pub filename: String,
}

impl Default for SongRank {
    fn default() -> Self {
        SongRank {
            song: Default::default(),
            score: usize::MAX,
            filename: String::new(),
        }
    }
}

pub struct SongRanker<'a> {
    ranking: [SongRank; RANK_LIST_SIZE],
    song_to_match: &'a Song,
}

impl<'a> SongRanker<'a> {
    pub fn new(song: &'a Song) -> SongRanker<'a> {
        let ranking: [SongRank; RANK_LIST_SIZE] = Default::default();

        SongRanker {
            ranking,
            song_to_match: song,
        }
    }

    pub fn rank(&mut self, new_song: Song, filename: String) {
        /* Create song rank */

        let base_token = self.song_to_match.generate_token();
        let new_song_token = new_song.generate_token();
        let score = SongRanker::token_diff(base_token.as_str(), new_song_token.as_str());
        let song_rank = SongRank {
            song: new_song,
            score,
            filename,
        };

        /* Check score with previous entries */

        for rank_position in 0..RANK_LIST_SIZE {
            let ranked_item = &self.ranking[rank_position];

            if song_rank.score < ranked_item.score {
                self.add_to_ranklist(song_rank, rank_position);
                return;
            }
        }
    }

    fn add_to_ranklist(&mut self, item: SongRank, pos: usize) {
        for i in (RANK_LIST_SIZE - 1)..(pos + 1) {
            self.ranking[i] = self.ranking[i - 1].clone();
        }
        self.ranking[pos] = item.clone();
    }

    pub fn print_rank_list(&self) {
        println!(
            "Ranking for '{}' by '{}':",
            self.song_to_match.title, self.song_to_match.artist
        );

        for i in 0..RANK_LIST_SIZE {
            let artist = &self.ranking[i].song.artist;
            let title = &self.ranking[i].song.title;
            let score = self.ranking[i].score;
            let filename = &self.ranking[i].filename;
            println!(
                "  {:>2}. ({:>2}) {} - {} `{}`",
                i + 1,
                score,
                artist,
                title,
                filename
            );
        }
    }

    pub fn get_pos(&self, pos: usize) -> Result<SongRank> {
        if pos >= RANK_LIST_SIZE || pos < 1 {
            return Err(LyricFinderError::InvalidRankListPositionError);
        }

        Ok(self.ranking[pos - 1].clone())
    }

    fn token_diff(base: &str, test: &str) -> usize {
        let base_len = base.len();
        let test_len = test.len();
        let min_len = std::cmp::min(base_len, test_len);

        /* If string dimension differs add the difference to the score */

        let mut diff = if base_len > test_len {
            base_len - test_len
        } else {
            test_len - base_len
        };

        /* For each character check if base-test differs */

        for c in 0..min_len as usize {
            let base_char = base.as_bytes()[c];
            let test_char = test.as_bytes()[c];

            if base_char != test_char {
                diff += 1;
            }
        }

        diff
    }
}
