//! # random-unicode-emoji
//!
//! A simple Rust crate that returns random Unicode emojis. ❤️

use include_dir::{include_dir, Dir};
use rand::seq::SliceRandom;
use include_dir::File;
use std::u32;

pub fn random_emoji(count: usize, version: &str) -> Vec<String> {

    static PROJECT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR");
    let file: &File = PROJECT_DIR.get_file("emoji/".to_owned() + &version + "/emoji-test.txt").expect(&("Unicode version \"".to_owned() + version + "\" not supported."));
    let lines: &str = file.contents_utf8().unwrap();

    let mut lines: Vec<String> = lines.split('\n').map(|line| line.replace("#", "#")).collect();
    lines.retain(|value: &String| !value.starts_with("#") & !value.is_empty());

    let mut unicode: Vec<String> = Vec::new();

    for mut line in lines{
        line = (&line[0..line.find(";").unwrap_or(line.len())]).trim().to_string();
        let mut full_emoji: String = "".to_string();

        for word in line.split_whitespace(){
            let hex: u32 = u32::from_str_radix(&word, 16).unwrap();
            let emoji: char = char::from_u32(hex).unwrap();
            full_emoji.push(emoji);
        }
        unicode.push(full_emoji);
    }

    let sample = unicode.choose_multiple(&mut rand::thread_rng(), count).cloned().collect();

    return sample;
}

// Add tests
