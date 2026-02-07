# Lyric finder

This is a an application written in Rust that gets the current playing song in
cmus and then search for a matching lyric inside a folder.

It's a personal application used to move from an old version of lyric library
to a new one.

## Build and install

With the rust toolcahin installed

    cargo build --release

then install

	install -m 755 target/release/lrc_finder $HOME/.local/bin
