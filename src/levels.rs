//! ======================================================
//!  THE LEVEL BOOK — every level lives here!
//! ======================================================
//! Each level is just a LIST of (what piece, how far away).
//! That's all a level is: a list! One day, a LEVEL EDITOR
//! could write these lists for us by clicking — for now,
//! we write them by hand. Design your own!
//!
//! THE WHOLE ADVENTURE:
//!   1. Bunny Meadow      — a friendly start
//!   2. Cube Mountain     — platform hopping
//!   3. The Gauntlet      — fast and furious!
//!   4. THE BIG RED BOSS  — the first boss...
//!   5. Sky Stairs        — climb high above the ground!
//!   6. The Tricky Tower  — gap jumps at great heights!
//!   7. THE CURSED THORN  — the FINAL boss of the game!
//!
//! Bigger minus numbers = farther down the road.
//! Cubes 1.2 apart sit right next to each other and
//! make a longer platform you can run across!

use crate::Piece;

/// Level 4 is the first boss...
pub const FIRST_BOSS: usize = 4;

/// ...and level 7 is the FINAL boss: the Cursed Thorn!
pub const FINAL_BOSS: usize = 7;

/// How fast each level slides at you.
/// Levels 3 and 6 are the speedy ones!
pub fn level_speed(level: usize) -> f32 {
    match level {
        3 => 10.0,
        6 => 9.0,
        _ => 8.0,
    }
}

/// Where the golden finish gate waits in each level.
pub fn finish_line(level: usize) -> f32 {
    match level {
        1 => -106.0,
        2 => -110.0,
        3 => -122.0,
        5 => -100.0,
        _ => -108.0,
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
        // climb the big cube mountain in the middle.
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

        // ---------- LEVEL 5: Sky Stairs ----------
        // After the first boss! All about climbing.
        // Bridges with GAPS — hop from one to the next,
        // then climb a giant staircase into the sky!
        5 => vec![
            // Two low bridges with a gap between them.
            (Piece::Cube, -18.0),
            (Piece::Cube, -19.2),
            (Piece::Cube, -20.4),
            (Piece::Cube, -26.0),
            (Piece::Cube, -27.2),
            (Piece::Cube, -28.4),
            // ...climbing to a tall ridge at the end!
            (Piece::TallCube, -29.6),
            (Piece::TallCube, -30.8),
            (Piece::Spike, -39.0),
            // THE GIANT STAIRCASE: low, tall, TRIPLE!
            // Up here the bunny is really, really high!
            (Piece::Cube, -46.0),
            (Piece::Cube, -47.2),
            (Piece::TallCube, -48.4),
            (Piece::TallCube, -49.6),
            (Piece::TripleCube, -50.8),
            (Piece::TripleCube, -52.0),
            // The big drop! Then double spikes.
            (Piece::Spike, -61.0),
            (Piece::Spike, -62.2),
            (Piece::Cube, -69.0),
            (Piece::Cube, -70.2),
            (Piece::Cube, -71.4),
            (Piece::SkySpike, -79.0),
            (Piece::Spike, -86.0),
        ],

        // ---------- LEVEL 6: The Tricky Tower ----------
        // The hardest level! Gap jumps at every height:
        // bridge → tall bridge → TRIPLE bridge, each with
        // a gap. Miss a jump and you hit the SIDE. Careful!
        6 => vec![
            (Piece::Spike, -18.0),
            // Low bridge...
            (Piece::Cube, -25.0),
            (Piece::Cube, -26.2),
            (Piece::Cube, -27.4),
            // ...gap, then jump UP onto the tall bridge...
            (Piece::TallCube, -33.0),
            (Piece::TallCube, -34.2),
            (Piece::TallCube, -35.4),
            // ...gap, then UP again onto the TRIPLE bridge!
            (Piece::TripleCube, -41.0),
            (Piece::TripleCube, -42.2),
            (Piece::TripleCube, -43.4),
            // The huge drop, then double spikes!
            (Piece::Spike, -52.0),
            (Piece::Spike, -53.2),
            // Ride two cubes, jump the spiky third.
            (Piece::Cube, -60.0),
            (Piece::Cube, -61.2),
            (Piece::CubeWithSpike, -62.4),
            (Piece::SkySpike, -70.0),
            (Piece::BadGuy, -77.0),
            (Piece::Cube, -84.0),
            (Piece::Cube, -85.2),
            (Piece::Cube, -86.4),
            (Piece::Spike, -94.0),
        ],

        // ---------- BOSS LEVELS (4 and 7) ----------
        // No pieces! The bosses bring their own trouble...
        _ => vec![],
    }
}
