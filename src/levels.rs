//! ======================================================
//!  THE LEVEL BOOK — reading and writing levels.txt!
//! ======================================================
//! The levels themselves live in assets/levels.txt now —
//! open it up, it's easy to read! This file is the
//! LIBRARIAN: it reads that file into the game when we
//! start, and writes it back when the LEVEL EDITOR saves.

use crate::Piece;
use bevy::prelude::Resource;
#[cfg(not(target_arch = "wasm32"))]
use std::fs;

/// Where the level book lives on disk.
/// (On the web there is no disk, so this goes unused.)
#[cfg_attr(target_arch = "wasm32", allow(dead_code))]
pub const LEVELS_FILE: &str = "assets/levels.txt";

/// Everything we know about ONE stage of the game.
pub struct LevelData {
    pub name: String,
    pub speed: f32,
    /// Where the golden gate is (a minus number, like pieces).
    pub finish: f32,
    /// Which boss lives here: "tomato", "thorn", or "bat".
    /// Empty ("") means it's a normal level, not a boss!
    pub boss: String,
    /// The list of pieces, just like always!
    pub pieces: Vec<(Piece, f32)>,
}

/// The whole book of levels, shared with the whole game.
#[derive(Resource)]
pub struct LevelBook {
    pub levels: Vec<LevelData>,
}

impl LevelBook {
    /// Look up one level. Level 1 is the first page.
    /// (".min" keeps us from reading past the last page!)
    pub fn get(&self, level: usize) -> &LevelData {
        let page = (level - 1).min(self.levels.len() - 1);
        &self.levels[page]
    }

    /// Look up one level when we want to CHANGE it.
    pub fn get_mut(&mut self, level: usize) -> &mut LevelData {
        let page = (level - 1).min(self.levels.len() - 1);
        &mut self.levels[page]
    }

    /// Bosses don't count as levels! This counts only the
    /// REAL levels up to this stage, so stage 5 (which
    /// comes after the first boss) is shown as "Level 4".
    pub fn level_number(&self, stage: usize) -> usize {
        let mut count = 0;
        for page in 0..stage.min(self.levels.len()) {
            if self.levels[page].boss.is_empty() {
                count += 1;
            }
        }
        count
    }

    /// And this counts only the BOSSES up to this stage,
    /// so the Cursed Thorn (stage 8) is "Boss 2".
    pub fn boss_number(&self, stage: usize) -> usize {
        let mut count = 0;
        for page in 0..stage.min(self.levels.len()) {
            if !self.levels[page].boss.is_empty() {
                count += 1;
            }
        }
        count
    }

    /// THE ONE TRUE NAME TAG! Every screen in the game
    /// asks here, so a stage is always called the same
    /// thing everywhere: "Level 4: Sky Stairs" for real
    /// levels, "Boss 1: Rotten Tomato" for bosses.
    pub fn label(&self, stage: usize) -> String {
        let data = self.get(stage);
        if data.boss.is_empty() {
            format!("Level {}: {}", self.level_number(stage), data.name)
        } else {
            format!("Boss {}: {}", self.boss_number(stage), data.name)
        }
    }
}

/// Turn a piece into the word we write in the file.
#[cfg_attr(target_arch = "wasm32", allow(dead_code))]
pub fn piece_to_word(piece: Piece) -> &'static str {
    match piece {
        Piece::Spike => "spike",
        Piece::Cube => "cube",
        Piece::TallCube => "tall",
        Piece::TripleCube => "triple",
        Piece::CubeWithSpike => "cubespike",
        Piece::SkySpike => "skyspike",
        Piece::BadGuy => "badguy",
    }
}

/// Turn a word from the file back into a piece.
/// "None" means "that's not a piece word I know!"
pub fn word_to_piece(word: &str) -> Option<Piece> {
    match word {
        "spike" => Some(Piece::Spike),
        "cube" => Some(Piece::Cube),
        "tall" => Some(Piece::TallCube),
        "triple" => Some(Piece::TripleCube),
        "cubespike" => Some(Piece::CubeWithSpike),
        "skyspike" => Some(Piece::SkySpike),
        "badguy" => Some(Piece::BadGuy),
        _ => None,
    }
}

/// Read levels.txt and build the level book, one line
/// at a time. This is called PARSING — turning words
/// in a file into things the computer understands!
pub fn load_level_book() -> LevelBook {
    // In a web browser there are no files to read, so the
    // level book gets BAKED INTO the game when we build it.
    #[cfg(target_arch = "wasm32")]
    let text = include_str!("../assets/levels.txt").to_string();

    // On a real computer, read the file fresh each launch.
    #[cfg(not(target_arch = "wasm32"))]
    let text = fs::read_to_string(LEVELS_FILE)
        .expect("Oh no — I couldn't find assets/levels.txt!");

    let mut levels: Vec<LevelData> = Vec::new();

    for line in text.lines() {
        let line = line.trim();

        // Skip empty lines and "#" comment lines.
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Chop the line into words: "spike 18" → ["spike", "18"]
        let words: Vec<&str> = line.split_whitespace().collect();

        match words[0] {
            // "level 5 Sky Stairs" → start a fresh level.
            // Its name is all the words after the number.
            "level" => levels.push(LevelData {
                name: words[2..].join(" "),
                speed: 8.0,
                finish: 0.0,
                boss: String::new(),
                pieces: Vec::new(),
            }),
            // "boss tomato" → this stage is a boss fight!
            "boss" => {
                if let Some(level) = levels.last_mut() {
                    level.boss = words[1].to_string();
                }
            }
            // "speed 9" → how fast this level goes.
            "speed" => {
                if let Some(level) = levels.last_mut() {
                    level.speed = words[1].parse().unwrap_or(8.0);
                }
            }
            // "finish 106" → the gate is 106 down the road.
            // We flip it to -106 because "down the road"
            // is the minus-z direction in our world.
            "finish" => {
                if let Some(level) = levels.last_mut() {
                    let distance: f32 = words[1].parse().unwrap_or(0.0);
                    level.finish = -distance;
                }
            }
            // Any other word should be a piece, like "cube 27".
            word => {
                if let (Some(piece), Some(level)) = (word_to_piece(word), levels.last_mut()) {
                    let distance: f32 = words[1].parse().unwrap_or(0.0);
                    level.pieces.push((piece, -distance));
                }
            }
        }
    }

    LevelBook { levels }
}

/// Write the whole level book back into levels.txt.
/// This is what the level editor's SAVE button does!
/// Gives back true if it worked — in a web browser
/// there's no file to write, so it gives back false.
pub fn save_level_book(book: &LevelBook) -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        let _ = book; // (nothing to do on the web)
        return false;
    }
    #[cfg(not(target_arch = "wasm32"))]
    save_to_disk(book)
}

#[cfg(not(target_arch = "wasm32"))]
fn save_to_disk(book: &LevelBook) -> bool {
    let mut out = String::from(
        "# ======================================================\n\
         #  THE LEVEL BOOK for 3D Bunny Dash!\n\
         # ======================================================\n\
         # Pieces: spike cube tall triple cubespike skyspike badguy\n\
         # Distances are how far down the road. Edit me by hand,\n\
         # or use the LEVEL EDITOR in the game!\n",
    );

    for (i, level) in book.levels.iter().enumerate() {
        out += &format!("\nlevel {} {}\n", i + 1, level.name);
        // Boss stages remember which boss lives there.
        if !level.boss.is_empty() {
            out += &format!("boss {}\n", level.boss);
        }
        out += &format!("speed {}\nfinish {}\n", level.speed, -level.finish);
        for (piece, z) in &level.pieces {
            // Flip the minus numbers back to plus for the
            // file, and round to one decimal place so the
            // file never fills up with 22.800003-style
            // crumbs (× 10, round, ÷ 10 → 22.8 exactly).
            let distance = (-z * 10.0).round() / 10.0;
            out += &format!("{} {}\n", piece_to_word(*piece), distance);
        }
    }

    if fs::write(LEVELS_FILE, out).is_err() {
        println!("Oh no — I couldn't save the level book!");
        return false;
    }
    true
}
