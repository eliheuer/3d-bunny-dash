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

/// How fast each level slides at you.
/// Level 3 is the speedy one!
pub fn level_speed(level: usize) -> f32 {
    if level == 3 { 10.0 } else { 8.0 }
}

/// Where the golden finish gate waits in each level.
pub fn finish_line(level: usize) -> f32 {
    match level {
        1 => -106.0,
        2 => -110.0,
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
            // Land on the bridge, ride two cubes, then
            // jump the spiky third one!
            (Piece::Cube, -74.0),
            (Piece::Cube, -75.2),
            (Piece::CubeWithSpike, -76.4),
            (Piece::Spike, -85.0),
            (Piece::SkySpike, -91.0),
            (Piece::Spike, -97.0),
        ],

        // ---------- LEVEL 2: Cube Mountain ----------
        // A platform level! Hop from cube to cube, and
        // climb the big cube mountain in the middle —
        // regular cubes are the stairs, TALL cubes are
        // the top. Up, up, across, and back down!
        2 => vec![
            (Piece::Cube, -18.0),
            (Piece::Spike, -26.0),
            // A little bridge to warm up.
            (Piece::Cube, -33.0),
            (Piece::Cube, -34.2),
            (Piece::SkySpike, -43.0),
            // THE CUBE MOUNTAIN! Three stairs up...
            (Piece::Cube, -50.0),
            (Piece::Cube, -51.2),
            (Piece::Cube, -52.4),
            // ...the tall peak (jump up from the stairs!)...
            (Piece::TallCube, -53.6),
            (Piece::TallCube, -54.8),
            // ...and back down the other side.
            (Piece::Cube, -56.0),
            (Piece::Cube, -57.2),
            (Piece::Spike, -65.0),
            // Ride two cubes, jump the spiky third!
            (Piece::Cube, -72.0),
            (Piece::Cube, -73.2),
            (Piece::CubeWithSpike, -74.4),
            (Piece::SkySpike, -83.0),
            (Piece::Spike, -90.0),
            // One last little bridge before the gate.
            (Piece::Cube, -97.0),
            (Piece::Cube, -98.2),
        ],

        // ---------- LEVEL 3: The Gauntlet ----------
        // The fastest level! Everything you've learned!
        3 => vec![
            (Piece::BadGuy, -18.0),
            (Piece::Spike, -27.0),
            (Piece::Spike, -28.2),
            (Piece::SkySpike, -37.0),
            // Ride two cubes, jump the spiky third!
            (Piece::Cube, -45.0),
            (Piece::Cube, -46.2),
            (Piece::CubeWithSpike, -47.4),
            (Piece::Spike, -58.0),
            (Piece::SkySpike, -65.0),
            (Piece::BadGuy, -72.0),
            // A long bridge...
            (Piece::Cube, -80.0),
            (Piece::Cube, -81.2),
            (Piece::Cube, -82.4),
            // ...then double spikes after you land!
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
