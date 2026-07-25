//! ======================================================
//!  THE LEVEL BOOK — every level lives here!
//! ======================================================
//! Each level is just a LIST of (what piece, how far away).
//! That's all a level is: a list! One day, a LEVEL EDITOR
//! could write these lists for us by clicking — for now,
//! we write them by hand. Design your own!
//!
//! Bigger minus numbers = farther down the road.
//! Cubes 1.2 apart sit right next to each other and
//! make a longer platform you can run across!

use crate::Piece;

/// How many levels there are. The LAST one is the BOSS!
pub const LAST_LEVEL: usize = 4;

/// Each level is a little faster than the one before!
/// Level 1 → 8, Level 2 → 9, Level 3 → 10, Boss → 8.
pub fn level_speed(level: usize) -> f32 {
    if level == LAST_LEVEL {
        8.0 // the boss level doesn't scroll, anyway!
    } else {
        7.0 + level as f32
    }
}

/// Where the golden finish gate waits in each level.
pub fn finish_line(level: usize) -> f32 {
    match level {
        1 => -106.0,
        2 => -112.0,
        _ => -122.0,
    }
}

/// The list of pieces for each level!
pub fn level_pieces(level: usize) -> Vec<(Piece, f32)> {
    match level {
        // ---------- LEVEL 1: Bunny Meadow ----------
        // A friendly start. One of everything!
        1 => vec![
            (Piece::Spike, -18.0),
            (Piece::Cube, -27.0),
            (Piece::Spike, -36.0),
            // Three cubes in a row make a bridge!
            (Piece::Cube, -45.0),
            (Piece::Cube, -46.2),
            (Piece::Cube, -47.4),
            (Piece::SkySpike, -56.0),
            (Piece::BadGuy, -65.0),
            // Hop on the plain cube, then jump the spiky one!
            (Piece::Cube, -74.0),
            (Piece::CubeWithSpike, -75.2),
            (Piece::Spike, -85.0),
            (Piece::SkySpike, -91.0),
            (Piece::Spike, -97.0),
        ],

        // ---------- LEVEL 2: Spiky Canyon ----------
        // Faster! And DOUBLE spikes — jump a little early
        // so you sail over both!
        2 => vec![
            (Piece::Spike, -18.0),
            // Two spikes side by side. One big jump!
            (Piece::Spike, -27.0),
            (Piece::Spike, -28.2),
            (Piece::Cube, -37.0),
            (Piece::Cube, -38.2),
            (Piece::SkySpike, -48.0),
            (Piece::Spike, -55.0),
            (Piece::Cube, -63.0),
            (Piece::CubeWithSpike, -64.2),
            (Piece::BadGuy, -74.0),
            (Piece::Spike, -82.0),
            (Piece::Spike, -83.2),
            (Piece::SkySpike, -92.0),
            (Piece::Spike, -99.0),
        ],

        // ---------- LEVEL 3: The Gauntlet ----------
        // The fastest level! Everything you've learned!
        3 => vec![
            (Piece::BadGuy, -18.0),
            (Piece::Spike, -27.0),
            (Piece::Spike, -28.2),
            (Piece::SkySpike, -37.0),
            (Piece::Cube, -45.0),
            (Piece::CubeWithSpike, -46.2),
            (Piece::Spike, -56.0),
            (Piece::SkySpike, -63.0),
            (Piece::BadGuy, -72.0),
            // A long bridge...
            (Piece::Cube, -80.0),
            (Piece::Cube, -81.2),
            (Piece::Cube, -82.4),
            // ...then double spikes right after landing!
            (Piece::Spike, -92.0),
            (Piece::Spike, -93.2),
            (Piece::SkySpike, -101.0),
            (Piece::Spike, -108.0),
        ],

        // ---------- LEVEL 4: THE BOSS ----------
        // No pieces! The boss builds his own trouble...
        _ => vec![],
    }
}
