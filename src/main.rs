//! ======================================================
//!        3-D  BUNNY  GEOMETRY  DASH
//! ======================================================
//! A game about a pink bunny who runs forever and
//! jumps over obstacles. Press ANY key (or SPACE) to jump!
//!
//! THE RULES (just like real Geometry Dash!):
//!   * SPIKES kill you if you touch them at all.
//!   * CUBES are PLATFORMS: you can LAND ON TOP of them,
//!     but if you smack into the SIDE... you die!
//!   * If you die, the level starts over. Instantly.
//!   * Reach the GOLDEN FINISH LINE to beat the level —
//!     fireworks! — and move on to the NEXT level.
//!   * Level 4 is the BIG RED BOSS... but that's only
//!     the FIRST boss. After more levels, level 7 is
//!     the FINAL boss: THE CURSED THORN — a spiky rose
//!     in a flower pot that shoots thorns at you!
//!
//! BOSS FIGHTS:
//!   Bosses shoot things at you — jump and dodge! Every
//!   3 shots you dodge, the boss loses a heart. Take all
//!   3 hearts to win the fight!
//!
//! THE MATH YOU WILL LEARN:
//!   * ADDING     : position = position + speed
//!   * GRAVITY    : a number that pulls you DOWN every frame
//!   * DISTANCE   : how far apart two things are
//!   * COMPARING  : is 2 smaller than 5?  (2 < 5 is true!)
//!   * WAVES      : sine makes numbers wiggle up and down!
//!   * REMAINDERS : 7 balls dodged, groups of 3 → 1 left over

use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;

// The level designs live in their own file: src/levels.rs
mod levels;

// ======================================================
//  NUMBERS THAT CONTROL THE GAME  (try changing these!)
// ======================================================

/// ~~~ SECRET CHEAT CODE ~~~
/// Which level the game starts on! Change this to jump
/// straight to any level (1 to 7) — great for practicing
/// a tricky level or visiting a boss. Put it back to 1
/// when you want to play the whole adventure!
const STARTING_LEVEL: usize = 1;

/// How strong the bunny's jump is.
const JUMP_POWER: f32 = 9.0;

/// Gravity pulls the bunny down. On Earth gravity is 9.8!
const GRAVITY: f32 = 22.0;

/// How close a spike or bad guy must be to hurt the bunny.
const CRASH_DISTANCE: f32 = 1.0;

/// How big the platform cubes are (1.2 on every side).
const CUBE_SIZE: f32 = 1.2;

/// How long the fireworks party lasts between levels.
const PARTY_SECONDS: f32 = 5.0;

/// How many hearts the boss has.
const BOSS_HEARTS: i32 = 3;

/// Dodge this many boss balls to knock off one heart.
const DODGES_PER_HEART: i32 = 3;

// ======================================================
//  KINDS OF LEVEL PIECES — a menu to choose from!
//  (The level lists in levels.rs are made of these.)
// ======================================================

#[derive(Clone, Copy)]
pub enum Piece {
    Spike,         // orange cone on the ground — JUMP!
    Cube,          // purple platform cube — land on top, or jump over!
    TallCube,      // TWO cubes high! Climb up from a normal cube!
    TripleCube,    // THREE cubes high! Climb up from a tall cube!
    CubeWithSpike, // a cube with a spike hat — do NOT land here!
    SkySpike,      // blue upside-down cone in the air — DON'T jump!
    BadGuy,        // red bouncing ball — jump over him!
}

// ======================================================
//  TAGS — little name stickers we put on things
//  so the computer knows which thing is which.
// ======================================================

/// The sticker that says "I am the bunny!"
#[derive(Component)]
struct Bunny {
    /// How fast the bunny is moving UP right now.
    /// A minus number means moving DOWN.
    up_speed: f32,
}

/// EVERYTHING that belongs to the current level wears
/// this sticker, so we can sweep it all away in one go
/// when the level restarts or changes.
#[derive(Component)]
struct LevelStuff;

/// Things that slide along as the level scrolls.
#[derive(Component)]
struct Scrolls;

/// Things that kill the bunny when touched.
#[derive(Component)]
struct Deadly;

/// Platform cubes: safe on TOP, deadly on the SIDE!
/// Each one remembers how tall its top is, so the bunny
/// knows how high to stand (normal cube: 1.2, tall: 2.4).
#[derive(Component)]
struct Platform {
    top: f32,
}

/// The golden gate at the end of the level.
#[derive(Component)]
struct FinishLine;

/// An extra sticker for things that do little hops.
#[derive(Component)]
struct Bouncing;

/// The sticker for the bunny's ears, so they can flop!
#[derive(Component)]
struct Ear;

/// Which boss is this?
#[derive(Clone, Copy, PartialEq)]
enum BossKind {
    BigRed,      // the first boss: a giant angry ball
    CursedThorn, // the FINAL boss: a spiky rose in a pot!
}

/// A BOSS! It remembers its hearts and its throwing.
#[derive(Component)]
struct Boss {
    kind: BossKind,
    hearts: i32,
    throw_timer: f32,
    shots_dodged: i32,
}

/// Something a boss threw at you — a ball or a thorn!
/// It remembers which way it is flying.
#[derive(Component)]
struct BossShot {
    velocity: Vec3,
}

/// One little glowing ball of firework spark!
#[derive(Component)]
struct Firework {
    /// Which way (and how fast) this spark is flying.
    velocity: Vec3,
    /// How many seconds of sparkle are left.
    life: f32,
}

/// The big words in the middle of the screen.
#[derive(Component)]
struct BigMessage;

/// The score words in the corner.
#[derive(Component)]
struct ScoreText;

/// The "Level 2  Boss: <3 <3 <3" words in the corner.
#[derive(Component)]
struct LevelText;

// ======================================================
//  RESOURCES — facts the whole game shares.
// ======================================================

/// The score: how long you have survived!
#[derive(Resource)]
struct Score {
    points: f32,
}

/// Which level we are on, and whether we need to build
/// a new one. "Some(3)" means "please switch to level 3!"
#[derive(Resource)]
struct Game {
    level: usize,
    switch_to: Option<usize>,
}

/// Is the fireworks party happening?
#[derive(Resource)]
struct Party {
    happening: bool,
    timer: f32,
    /// The level to go to when the party ends.
    next_level: usize,
}

// ======================================================
//  THE LAVA LAMP BACKGROUND — a custom shader material!
//  The real magic is in assets/lava_lamp.wgsl, a tiny
//  program that runs on the GRAPHICS CARD and paints
//  every dot of the background with wavy rainbow math.
// ======================================================

#[derive(Asset, TypePath, AsBindGroup, Clone)]
struct LavaLampMaterial {}

impl Material for LavaLampMaterial {
    fn fragment_shader() -> ShaderRef {
        "lava_lamp.wgsl".into()
    }
}

// ======================================================
//  MAIN — where the program starts, like page 1 of a book
// ======================================================

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialPlugin::<LavaLampMaterial>::default())
        .insert_resource(Score { points: 0.0 })
        .insert_resource(Game {
            level: STARTING_LEVEL,
            // Build the starting level right away!
            switch_to: Some(STARTING_LEVEL),
        })
        .insert_resource(Party {
            happening: false,
            timer: 0.0,
            next_level: 1,
        })
        // Run ONCE when the game starts:
        .add_systems(Startup, build_the_world)
        // Run EVERY FRAME (about 60 times each second!).
        // ".chain()" means: run them in exactly this order.
        .add_systems(
            Update,
            (
                switch_level,
                bunny_jump,
                flop_ears,
                move_level,
                spin_finish_line,
                bounce_bad_guys,
                boss_fight,
                move_boss_shots,
                check_for_crash,
                check_for_finish,
                sparkle_fireworks,
                end_the_party,
                update_words,
            )
                .chain(),
        )
        .run();
}

// ======================================================
//  BUILD THE WORLD — the things that are ALWAYS there:
//  ground, bunny, sun, camera, background, and words.
//  (The level pieces get built by switch_level.)
// ======================================================

fn build_the_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut lava: ResMut<Assets<LavaLampMaterial>>,
) {
    // ---------- COLORS (mixed from red, green, blue) ----------
    // Each number goes from 0.0 (none) to 1.0 (lots).
    // PINK = lots of red + some green + some blue.
    let pink = materials.add(Color::srgb(1.0, 0.4, 0.7));
    let light_pink = materials.add(Color::srgb(1.0, 0.7, 0.85));
    let green = materials.add(Color::srgb(0.3, 0.8, 0.4));
    let white = materials.add(Color::srgb(1.0, 1.0, 1.0));

    // ---------- THE GROUND ----------
    // A big flat box: 8 wide, very long, and thin like a pancake.
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(8.0, 0.2, 300.0))),
        MeshMaterial3d(green.clone()),
        // Position is (x, y, z):  x = left/right,
        // y = up/down,  z = toward you / away from you.
        Transform::from_xyz(0.0, -0.1, -50.0),
    ));

    // ---------- THE BUNNY ----------
    // We build the bunny out of simple shapes,
    // like snapping building blocks together!
    commands
        .spawn((
            Bunny { up_speed: 0.0 },
            // The bunny starts at height y = 0.5 (sitting on the ground).
            Transform::from_xyz(0.0, 0.5, 0.0),
            Visibility::default(),
        ))
        .with_children(|bunny| {
            // Body: a round ball
            bunny.spawn((
                Mesh3d(meshes.add(Sphere::new(0.5))),
                MeshMaterial3d(pink.clone()),
                Transform::from_xyz(0.0, 0.0, 0.0),
            ));
            // Head: a smaller ball on top and a little forward
            bunny.spawn((
                Mesh3d(meshes.add(Sphere::new(0.35))),
                MeshMaterial3d(pink.clone()),
                Transform::from_xyz(0.0, 0.55, -0.3),
            ));
            // Two tall ears! They wear the Ear sticker
            // so they can FLOP when we jump!
            bunny.spawn((
                Ear,
                Mesh3d(meshes.add(Capsule3d::new(0.08, 0.5))),
                MeshMaterial3d(light_pink.clone()),
                Transform::from_xyz(-0.15, 1.1, -0.3),
            ));
            bunny.spawn((
                Ear,
                Mesh3d(meshes.add(Capsule3d::new(0.08, 0.5))),
                MeshMaterial3d(light_pink.clone()),
                Transform::from_xyz(0.15, 1.1, -0.3),
            ));
            // A fluffy white tail on the back
            bunny.spawn((
                Mesh3d(meshes.add(Sphere::new(0.18))),
                MeshMaterial3d(white.clone()),
                Transform::from_xyz(0.0, 0.0, 0.55),
            ));
        });

    // ---------- THE SUN ----------
    commands.spawn((
        DirectionalLight {
            illuminance: 8_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // ---------- THE CAMERA (with the background stuck on!) ----------
    commands
        .spawn((
            Camera3d::default(),
            Transform::from_xyz(6.0, 5.0, 9.0).looking_at(Vec3::new(0.0, 1.0, -3.0), Vec3::Y),
        ))
        .with_children(|camera| {
            // ---------- THE LAVA LAMP BACKGROUND ----------
            // A giant wall painted by our shader program
            // (see assets/lava_lamp.wgsl). It is a CHILD of
            // the camera — like a poster taped WAY out in
            // front of the lens — so no matter where the
            // camera looks, the background fills the screen!
            camera.spawn((
                Mesh3d(meshes.add(Rectangle::new(2400.0, 1000.0))),
                MeshMaterial3d(lava.add(LavaLampMaterial {})),
                // 600 away, deeper than the whole level,
                // so everything else draws in front of it.
                Transform::from_xyz(0.0, 0.0, -600.0),
            ));
        });

    // ---------- THE CORNER WORDS ----------
    commands.spawn((
        ScoreText,
        Text::new("Score: 0"),
        TextFont {
            font_size: 40.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        },
    ));
    commands.spawn((
        LevelText,
        Text::new("Level 1"),
        TextFont {
            font_size: 40.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            right: Val::Px(20.0),
            ..default()
        },
    ));
}

// ======================================================
//  SWITCH LEVEL — sweep away the old level and build
//  the new one from its list in levels.rs!
// ======================================================

fn switch_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut game: ResMut<Game>,
    old_stuff: Query<Entity, With<LevelStuff>>,
) {
    // ".take()" grabs the switch order and empties it,
    // so we only do this once per order.
    let Some(new_level) = game.switch_to.take() else {
        return; // no order? nothing to do.
    };
    game.level = new_level;

    // Sweep away every piece of the old level.
    for old_thing in &old_stuff {
        commands.entity(old_thing).despawn();
    }

    // Colors and shapes for the level pieces:
    let orange = materials.add(Color::srgb(1.0, 0.5, 0.1));
    let purple = materials.add(Color::srgb(0.6, 0.2, 0.9));
    let blue = materials.add(Color::srgb(0.2, 0.5, 1.0));
    let red = materials.add(Color::srgb(0.9, 0.1, 0.1));
    let gold = materials.add(Color::srgb(1.0, 0.85, 0.2));
    let white = materials.add(Color::srgb(1.0, 1.0, 1.0));
    let spike_shape = meshes.add(Cone::new(0.6, 1.4));
    let cube_shape = meshes.add(Cuboid::new(CUBE_SIZE, CUBE_SIZE, CUBE_SIZE));

    // Ask the level book for this level's list of pieces.
    for (piece, start_z) in levels::level_pieces(new_level) {
        match piece {
            // Orange spike sitting on the ground. JUMP!
            Piece::Spike => {
                commands.spawn((
                    LevelStuff,
                    Scrolls,
                    Deadly,
                    Mesh3d(spike_shape.clone()),
                    MeshMaterial3d(orange.clone()),
                    Transform::from_xyz(0.0, 0.7, start_z),
                ));
            }
            // Purple platform cube. Land on TOP — not the side!
            Piece::Cube => {
                commands.spawn((
                    LevelStuff,
                    Scrolls,
                    Platform { top: CUBE_SIZE },
                    Mesh3d(cube_shape.clone()),
                    MeshMaterial3d(purple.clone()),
                    // The cube's middle is at half its height.
                    Transform::from_xyz(0.0, CUBE_SIZE / 2.0, start_z),
                ));
            }
            // A DOUBLE-TALL cube! Too high to reach from the
            // ground — climb up using a normal cube first,
            // like stairs!
            Piece::TallCube => {
                commands.spawn((
                    LevelStuff,
                    Scrolls,
                    Platform {
                        top: CUBE_SIZE * 2.0,
                    },
                    Mesh3d(meshes.add(Cuboid::new(CUBE_SIZE, CUBE_SIZE * 2.0, CUBE_SIZE))),
                    MeshMaterial3d(purple.clone()),
                    Transform::from_xyz(0.0, CUBE_SIZE, start_z),
                ));
            }
            // A TRIPLE-TALL cube! Only reachable from a
            // tall cube. Up here you can see everything!
            Piece::TripleCube => {
                commands.spawn((
                    LevelStuff,
                    Scrolls,
                    Platform {
                        top: CUBE_SIZE * 3.0,
                    },
                    Mesh3d(meshes.add(Cuboid::new(CUBE_SIZE, CUBE_SIZE * 3.0, CUBE_SIZE))),
                    MeshMaterial3d(purple.clone()),
                    Transform::from_xyz(0.0, CUBE_SIZE * 1.5, start_z),
                ));
            }
            // A cube wearing a spike hat. Do NOT land on this one!
            Piece::CubeWithSpike => {
                commands.spawn((
                    LevelStuff,
                    Scrolls,
                    Platform { top: CUBE_SIZE },
                    Mesh3d(cube_shape.clone()),
                    MeshMaterial3d(purple.clone()),
                    Transform::from_xyz(0.0, CUBE_SIZE / 2.0, start_z),
                ));
                commands.spawn((
                    LevelStuff,
                    Scrolls,
                    Deadly,
                    Mesh3d(spike_shape.clone()),
                    MeshMaterial3d(orange.clone()),
                    // The spike sits on top: cube height + half the spike.
                    Transform::from_xyz(0.0, CUBE_SIZE + 0.7, start_z),
                ));
            }
            // Blue upside-down spike FLOATING IN THE AIR.
            // DON'T jump — run under it!
            Piece::SkySpike => {
                commands.spawn((
                    LevelStuff,
                    Scrolls,
                    Deadly,
                    Mesh3d(spike_shape.clone()),
                    MeshMaterial3d(blue.clone()),
                    // rotate_x by PI (half a full turn) flips
                    // the cone upside down!
                    Transform::from_xyz(0.0, 2.2, start_z)
                        .with_rotation(Quat::from_rotation_x(std::f32::consts::PI)),
                ));
            }
            // The RED BAD GUY! A ball that does little hops.
            Piece::BadGuy => {
                commands
                    .spawn((
                        LevelStuff,
                        Scrolls,
                        Deadly,
                        Bouncing,
                        Mesh3d(meshes.add(Sphere::new(0.55))),
                        MeshMaterial3d(red.clone()),
                        Transform::from_xyz(0.0, 0.55, start_z),
                        Visibility::default(),
                    ))
                    .with_children(|bad_guy| {
                        bad_guy.spawn((
                            Mesh3d(meshes.add(Sphere::new(0.12))),
                            MeshMaterial3d(white.clone()),
                            Transform::from_xyz(-0.2, 0.2, 0.45),
                        ));
                        bad_guy.spawn((
                            Mesh3d(meshes.add(Sphere::new(0.12))),
                            MeshMaterial3d(white.clone()),
                            Transform::from_xyz(0.2, 0.2, 0.45),
                        ));
                    });
            }
        }
    }

    if new_level == levels::FIRST_BOSS {
        // ---------- THE BIG RED BOSS! ----------
        // A GIANT angry ball floats ahead and throws
        // bouncing balls at you. He does not scroll —
        // he just hangs there, being enormous and rude.
        commands
            .spawn((
                LevelStuff,
                Boss {
                    kind: BossKind::BigRed,
                    hearts: BOSS_HEARTS,
                    throw_timer: 0.0,
                    shots_dodged: 0,
                },
                Mesh3d(meshes.add(Sphere::new(1.4))),
                MeshMaterial3d(red.clone()),
                Transform::from_xyz(0.0, 2.2, -16.0),
                Visibility::default(),
            ))
            .with_children(|boss| {
                // Giant angry eyes!
                boss.spawn((
                    Mesh3d(meshes.add(Sphere::new(0.3))),
                    MeshMaterial3d(white.clone()),
                    Transform::from_xyz(-0.5, 0.4, 1.15),
                ));
                boss.spawn((
                    Mesh3d(meshes.add(Sphere::new(0.3))),
                    MeshMaterial3d(white.clone()),
                    Transform::from_xyz(0.5, 0.4, 1.15),
                ));
                // A golden crown, because he is the boss.
                boss.spawn((
                    Mesh3d(meshes.add(Cone::new(0.5, 0.7))),
                    MeshMaterial3d(gold.clone()),
                    Transform::from_xyz(0.0, 1.5, 0.0),
                ));
            });
    } else if new_level == levels::FINAL_BOSS {
        // ---------- THE CURSED THORN! ----------
        // The FINAL boss: a spooky rose growing out of a
        // flower pot. It slides side to side and shoots
        // THORNS at you. Dodge them all to win the game!
        let brown = materials.add(Color::srgb(0.5, 0.3, 0.15));
        let stem_green = materials.add(Color::srgb(0.2, 0.55, 0.2));
        let rose_pink = materials.add(Color::srgb(1.0, 0.2, 0.5));
        let thorn_gray = materials.add(Color::srgb(0.45, 0.45, 0.4));

        commands
            .spawn((
                LevelStuff,
                Boss {
                    kind: BossKind::CursedThorn,
                    hearts: BOSS_HEARTS,
                    throw_timer: 0.0,
                    shots_dodged: 0,
                },
                Transform::from_xyz(0.0, 0.0, -16.0),
                Visibility::default(),
            ))
            .with_children(|plant| {
                // The flower pot.
                plant.spawn((
                    Mesh3d(meshes.add(Cuboid::new(1.6, 1.1, 1.6))),
                    MeshMaterial3d(brown.clone()),
                    Transform::from_xyz(0.0, 0.55, 0.0),
                ));
                // The tall green stem growing out of it.
                plant.spawn((
                    Mesh3d(meshes.add(Capsule3d::new(0.16, 2.4))),
                    MeshMaterial3d(stem_green.clone()),
                    Transform::from_xyz(0.0, 2.3, 0.0),
                ));
                // Sharp thorns sticking out of the stem,
                // pointing left and right, one per height.
                for i in 0..4 {
                    // Thorn 0 points left, 1 right, 2 left...
                    // (i % 2 tells us: even or odd?)
                    let side = if i % 2 == 0 { -1.0 } else { 1.0 };
                    let height = 1.4 + i as f32 * 0.55;
                    plant.spawn((
                        Mesh3d(meshes.add(Cone::new(0.14, 0.5))),
                        MeshMaterial3d(thorn_gray.clone()),
                        Transform::from_xyz(side * 0.35, height, 0.0)
                            // Tip the thorn sideways (a quarter
                            // turn is about 1.57 radians).
                            .with_rotation(Quat::from_rotation_z(side * -1.57)),
                    ));
                }
                // The evil rose on top: a yellow middle...
                plant.spawn((
                    Mesh3d(meshes.add(Sphere::new(0.3))),
                    MeshMaterial3d(materials.add(Color::srgb(1.0, 0.85, 0.2))),
                    Transform::from_xyz(0.0, 3.7, 0.3),
                ));
                // ...with 6 pink petals in a circle around it.
                // cos & sin place them around the circle,
                // just like the fireworks!
                for i in 0..6 {
                    let angle = i as f32 * 6.28 / 6.0;
                    plant.spawn((
                        Mesh3d(meshes.add(Sphere::new(0.28))),
                        MeshMaterial3d(rose_pink.clone()),
                        Transform::from_xyz(angle.cos() * 0.5, 3.7 + angle.sin() * 0.5, 0.15),
                    ));
                }
                // Two angry eyes on the flower, of course.
                plant.spawn((
                    Mesh3d(meshes.add(Sphere::new(0.09))),
                    MeshMaterial3d(white.clone()),
                    Transform::from_xyz(-0.12, 3.75, 0.56),
                ));
                plant.spawn((
                    Mesh3d(meshes.add(Sphere::new(0.09))),
                    MeshMaterial3d(white.clone()),
                    Transform::from_xyz(0.12, 3.75, 0.56),
                ));
            });
    } else {
        // ---------- THE GOLDEN FINISH LINE ----------
        // Normal levels end at a golden gate. Reach it to win!
        commands
            .spawn((
                LevelStuff,
                Scrolls,
                FinishLine,
                Transform::from_xyz(0.0, 0.0, levels::finish_line(new_level)),
                Visibility::default(),
            ))
            .with_children(|gate| {
                gate.spawn((
                    Mesh3d(meshes.add(Cuboid::new(0.4, 4.0, 0.4))),
                    MeshMaterial3d(gold.clone()),
                    Transform::from_xyz(-2.0, 2.0, 0.0),
                ));
                gate.spawn((
                    Mesh3d(meshes.add(Cuboid::new(0.4, 4.0, 0.4))),
                    MeshMaterial3d(gold.clone()),
                    Transform::from_xyz(2.0, 2.0, 0.0),
                ));
                gate.spawn((
                    Mesh3d(meshes.add(Cuboid::new(4.4, 0.4, 0.4))),
                    MeshMaterial3d(gold.clone()),
                    Transform::from_xyz(0.0, 4.0, 0.0),
                ));
            });
    }
}

// ======================================================
//  A LITTLE HELPER — how high is the floor under the
//  bunny right now? Usually 0 (the ground), but if a
//  platform cube is under us, the floor is the cube top!
// ======================================================

fn floor_height_under_bunny(
    platforms: &Query<(&Transform, &Platform), Without<Bunny>>,
) -> f32 {
    let mut floor = 0.0;

    for (position, platform) in platforms {
        // How far away is this cube from the bunny (at z = 0)?
        // ".abs()" makes minus numbers plus: -3 becomes 3.
        let how_far = position.translation.z.abs();

        // If the cube is under our feet (closer than 1 away)
        // AND its top is the highest one so far...
        if how_far < 1.0 && platform.top > floor {
            // ...the floor is the TOP of that cube!
            floor = platform.top;
        }
    }

    floor
}

// ======================================================
//  JUMPING — press any key to jump!
//  Also: landing on platform cubes, like Geometry Dash.
// ======================================================

fn bunny_jump(
    mut commands: Commands,
    sounds: Res<AssetServer>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    party: Res<Party>,
    mut bunnies: Query<(&mut Transform, &mut Bunny)>,
    platforms: Query<(&Transform, &Platform), Without<Bunny>>,
) {
    // No jumping during the fireworks party!
    if party.happening {
        return;
    }

    for (mut position, mut bunny) in &mut bunnies {
        // Where is the floor right now? The ground (0), or
        // the top of a platform cube (1.2)?
        let floor = floor_height_under_bunny(&platforms);

        // The bunny's middle sits half a bunny above the floor.
        let standing_height = floor + 0.5;

        // Is the bunny standing on the floor (or very close)?
        let on_the_floor = position.translation.y <= standing_height + 0.05;

        // If we press a key AND we are standing... JUMP!
        if keyboard.get_just_pressed().len() > 0 && on_the_floor && bunny.up_speed <= 0.0 {
            bunny.up_speed = JUMP_POWER;

            // Play the happy jump sound! BOOP!
            commands.spawn((
                AudioPlayer::new(sounds.load("jump.wav")),
                PlaybackSettings::DESPAWN,
            ));
        }

        // GRAVITY MATH: every frame, gravity SUBTRACTS from
        // our up-speed. So going up gets slower and slower,
        // then we start falling. Just like a real ball!
        bunny.up_speed = bunny.up_speed - GRAVITY * time.delta_secs();

        // MOVING MATH: new height = old height + speed × time
        position.translation.y = position.translation.y + bunny.up_speed * time.delta_secs();

        // Did we fall onto the floor? Land there and stop.
        // (Smacking into the SIDE of a cube is handled by the
        // crash checker — that one is game over!)
        if position.translation.y < standing_height && bunny.up_speed <= 0.0 {
            position.translation.y = standing_height;
            bunny.up_speed = 0.0;
        }
    }
}

// ======================================================
//  FLOPPY EARS — the ears tilt back when the bunny goes
//  UP and flop forward when the bunny comes DOWN!
// ======================================================

fn flop_ears(bunnies: Query<&Bunny>, mut ears: Query<&mut Transform, With<Ear>>) {
    for bunny in &bunnies {
        // MULTIPLYING MATH: up-speed × 0.08 turns a big
        // speed (like 9) into a small tilt (like 0.72).
        // "clamp" keeps the number between -0.9 and 0.9,
        // so the ears never spin all the way around!
        let tilt = (bunny.up_speed * 0.08).clamp(-0.9, 0.9);

        for mut ear in &mut ears {
            ear.rotation = Quat::from_rotation_x(tilt);
        }
    }
}

// ======================================================
//  MOVING THE LEVEL — every piece slides toward the
//  bunny. Each level is a little faster!
// ======================================================

fn move_level(
    time: Res<Time>,
    party: Res<Party>,
    game: Res<Game>,
    mut pieces: Query<&mut Transform, With<Scrolls>>,
) {
    // The level stops moving during the party!
    if party.happening {
        return;
    }

    // Ask the level book how fast this level goes.
    let speed = levels::level_speed(game.level);

    for mut position in &mut pieces {
        // MOVING MATH again: new spot = old spot + speed × time.
        // z gets BIGGER, which means "coming toward the camera".
        position.translation.z = position.translation.z + speed * time.delta_secs();
    }
}

// ======================================================
//  The finish line's golden gate slowly spins so you
//  can see it sparkle from far away!
// ======================================================

fn spin_finish_line(time: Res<Time>, mut gates: Query<&mut Transform, With<FinishLine>>) {
    for mut gate in &mut gates {
        gate.rotate_y(0.8 * time.delta_secs());
    }
}

// ======================================================
//  BOUNCING — the bad guys hop using WAVE MATH!
// ======================================================

fn bounce_bad_guys(time: Res<Time>, mut bad_guys: Query<&mut Transform, With<Bouncing>>) {
    for mut position in &mut bad_guys {
        // SINE is a magic wave: as time goes on, sin(time)
        // wiggles smoothly between -1 and +1 forever.
        //   × 0.5   → just a small hop, easy to jump over
        //   .abs()  → flip minus to plus so it BOUNCES
        //             instead of sinking underground!
        let wave = (time.elapsed_secs() * 2.0).sin().abs() * 0.5;

        // Height = resting height (0.55) + the wave.
        position.translation.y = 0.55 + wave;
    }
}

// ======================================================
//  THE BOSS FIGHT! He floats, he glares, he THROWS.
// ======================================================

fn boss_fight(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
    mut party: ResMut<Party>,
    mut bosses: Query<(&mut Boss, &mut Transform)>,
) {
    if party.happening {
        return;
    }

    for (mut boss, mut position) in &mut bosses {
        // Each boss moves in its own spooky way (wave math!).
        match boss.kind {
            // The Big Red Boss floats up and down menacingly.
            BossKind::BigRed => {
                position.translation.y = 2.2 + (time.elapsed_secs() * 1.5).sin() * 0.4;
            }
            // The Cursed Thorn slides SIDE TO SIDE in its
            // pot, so its thorns come at you from angles!
            BossKind::CursedThorn => {
                position.translation.x = (time.elapsed_secs() * 0.9).sin() * 2.5;
            }
        }

        // Count up to the next throw. The FEWER hearts
        // left, the FASTER it throws. Bosses get mad!
        boss.throw_timer += time.delta_secs();
        let time_between_throws = 0.8 + boss.hearts as f32 * 0.5;

        if boss.throw_timer > time_between_throws {
            boss.throw_timer = 0.0;

            match boss.kind {
                // The Big Red Boss throws a dark ball that
                // rolls straight at the bunny.
                BossKind::BigRed => {
                    commands.spawn((
                        LevelStuff,
                        Deadly,
                        BossShot {
                            velocity: Vec3::new(0.0, 0.0, 9.0),
                        },
                        Mesh3d(meshes.add(Sphere::new(0.4))),
                        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.0, 0.4))),
                        Transform::from_xyz(0.0, 0.4, position.translation.z),
                    ));
                }
                // The Cursed Thorn shoots a THORN from its
                // flower, AIMED at where the bunny stands!
                BossKind::CursedThorn => {
                    // The thorn starts up at the flower...
                    let flower = position.translation + Vec3::new(0.0, 3.7, 0.3);
                    // ...and flies toward the bunny's spot.
                    // "normalize" keeps the direction but
                    // makes its length exactly 1, so × 11
                    // means "speed 11 in that direction".
                    let aim_at = Vec3::new(0.0, 0.5, 0.0);
                    let velocity = (aim_at - flower).normalize() * 11.0;

                    commands.spawn((
                        LevelStuff,
                        Deadly,
                        BossShot { velocity },
                        Mesh3d(meshes.add(Cone::new(0.22, 0.8))),
                        MeshMaterial3d(materials.add(Color::srgb(0.35, 0.4, 0.25))),
                        Transform::from_translation(flower)
                            // Point the thorn the way it flies!
                            // (Cones point UP, so we rotate UP
                            // to match the flight direction.)
                            .with_rotation(Quat::from_rotation_arc(
                                Vec3::Y,
                                velocity.normalize(),
                            )),
                    ));
                }
            }
        }

        // Did we take away ALL its hearts? WE WIN!
        if boss.hearts <= 0 {
            let (message, next_level) = match boss.kind {
                // Beat the first boss → on to level 5!
                BossKind::BigRed => ("BOSS DEFEATED!", levels::FIRST_BOSS + 1),
                // Beat the FINAL boss → you won it all!
                BossKind::CursedThorn => ("YOU WIN THE WHOLE GAME!", 1),
            };
            start_party(
                &mut commands,
                &mut meshes,
                &mut materials,
                &mut party,
                message,
                next_level,
            );
        }
    }
}

// ======================================================
//  BOSS SHOTS — balls and thorns fly at you;
//  dodge them to take the boss's hearts!
// ======================================================

fn move_boss_shots(
    mut commands: Commands,
    time: Res<Time>,
    party: Res<Party>,
    mut shots: Query<(Entity, &mut Transform, &BossShot)>,
    mut bosses: Query<&mut Boss>,
) {
    if party.happening {
        return;
    }

    for (shot_id, mut position, shot) in &mut shots {
        // Fly along! spot = spot + velocity × time.
        position.translation += shot.velocity * time.delta_secs();

        // If the shot flew PAST the bunny, we DODGED it!
        if position.translation.z > 6.0 {
            commands.entity(shot_id).despawn();

            for mut boss in &mut bosses {
                boss.shots_dodged += 1;

                // REMAINDER MATH: "%" tells you what's left
                // over after making groups. 6 % 3 = 0 means
                // "6 makes perfect groups of 3" — so every
                // 3rd dodge, the boss loses a heart!
                if boss.shots_dodged % DODGES_PER_HEART == 0 {
                    boss.hearts -= 1;
                }
            }
        }
    }
}

// ======================================================
//  CRASH CHECK — did the bunny touch anything deadly,
//  or the SIDE of a platform cube?
//  If yes: restart the level. Instantly!
// ======================================================

fn check_for_crash(
    mut commands: Commands,
    sounds: Res<AssetServer>,
    mut score: ResMut<Score>,
    party: Res<Party>,
    mut game: ResMut<Game>,
    mut bunnies: Query<(&mut Transform, &mut Bunny), Without<LevelStuff>>,
    deadly_things: Query<&Transform, (With<Deadly>, Without<Bunny>)>,
    platforms: Query<(&Transform, &Platform), Without<Bunny>>,
) {
    // Nothing can hurt you during the party!
    if party.happening {
        return;
    }

    for (mut bunny_position, mut bunny) in &mut bunnies {
        let mut crashed = false;

        // Spikes, bad guys, boss balls: DISTANCE MATH!
        // How far apart are we? Bevy measures it for us
        // (using the Pythagorean theorem — a² + b² = c²!)
        for danger in &deadly_things {
            let distance = bunny_position.translation.distance(danger.translation);

            // COMPARING: if the distance is SMALLER than 1.0,
            // they are touching. CRASH!
            if distance < CRASH_DISTANCE {
                crashed = true;
            }
        }

        // Platform cubes: only the SIDE is dangerous!
        // If a cube is at our spot AND our body is LOWER
        // than that cube's top, we smacked into the side.
        for (cube, platform) in &platforms {
            let close_in_z = cube.translation.z.abs() < 1.0;
            let too_low = bunny_position.translation.y < platform.top - 0.15;
            if close_in_z && too_low {
                crashed = true;
            }
        }

        if crashed {
            // Play the sad crash sound. WAH...
            commands.spawn((
                AudioPlayer::new(sounds.load("die.wav")),
                PlaybackSettings::DESPAWN,
            ));

            // The bunny died! Score to zero, bunny back on
            // the ground, and rebuild this same level.
            score.points = 0.0;
            bunny_position.translation.y = 0.5;
            bunny.up_speed = 0.0;
            game.switch_to = Some(game.level);
        }
    }
}

// ======================================================
//  FINISH CHECK — did the bunny reach the golden gate?
//  Then it's PARTY TIME, and the NEXT level awaits!
// ======================================================

fn check_for_finish(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut party: ResMut<Party>,
    game: Res<Game>,
    gates: Query<&Transform, With<FinishLine>>,
) {
    if party.happening {
        return;
    }

    for gate in &gates {
        // Has the gate slid all the way to the bunny (z = 0)?
        if gate.translation.z >= 0.0 {
            start_party(
                &mut commands,
                &mut meshes,
                &mut materials,
                &mut party,
                "LEVEL COMPLETE!",
                game.level + 1, // the next level!
            );
        }
    }
}

// ======================================================
//  START THE PARTY — big words + a fireworks burst!
// ======================================================

fn start_party(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    party: &mut Party,
    message: &str,
    next_level: usize,
) {
    party.happening = true;
    party.timer = 0.0;
    party.next_level = next_level;

    // ----- The big words on the screen -----
    commands.spawn((
        BigMessage,
        Text::new(message),
        TextFont {
            font_size: 90.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.9, 0.2)),
        TextLayout::new_with_justify(Justify::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(35.0),
            width: Val::Percent(100.0),
            ..default()
        },
    ));

    // ----- FIREWORKS! -----
    // We launch 60 glowing balls in a circle. For ball
    // number i, we turn i into an angle, then cos & sin
    // aim around the circle — that's how math draws circles!
    let firework_colors = [
        Color::srgb(1.0, 0.3, 0.3), // red
        Color::srgb(1.0, 0.9, 0.2), // yellow
        Color::srgb(0.3, 1.0, 0.4), // green
        Color::srgb(0.3, 0.6, 1.0), // blue
        Color::srgb(1.0, 0.4, 0.9), // pink
    ];
    let spark_shape = meshes.add(Sphere::new(0.12));

    for i in 0..60 {
        // Spread 60 sparks around a full circle
        // (a full circle is about 6.28 radians).
        let angle = i as f32 * 6.28 / 60.0;

        // Every 7th spark flies a bit faster, so the
        // burst looks fluffy instead of a perfect ring.
        let speed = 3.0 + ((i % 7) as f32) * 0.7;

        let color = firework_colors[i % firework_colors.len()];

        commands.spawn((
            Firework {
                velocity: Vec3::new(
                    angle.cos() * speed,       // sideways
                    4.0 + ((i % 5) as f32),    // upward!
                    angle.sin() * speed - 2.0, // forward/back
                ),
                life: 2.5,
            },
            Mesh3d(spark_shape.clone()),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: color,
                emissive: color.to_linear() * 3.0, // makes it GLOW!
                ..default()
            })),
            Transform::from_xyz(0.0, 2.0, 0.0),
        ));
    }
}

// ======================================================
//  FIREWORK SPARKS — every spark flies, falls, and fades.
// ======================================================

fn sparkle_fireworks(
    mut commands: Commands,
    time: Res<Time>,
    mut sparks: Query<(Entity, &mut Transform, &mut Firework)>,
) {
    for (spark_id, mut position, mut spark) in &mut sparks {
        // Sparks move just like the bunny: spot + speed × time.
        position.translation += spark.velocity * time.delta_secs();

        // Gravity pulls sparks down too (but gently).
        spark.velocity.y -= GRAVITY * 0.35 * time.delta_secs();

        // Count down this spark's life...
        spark.life -= time.delta_secs();

        // ...and when it hits zero, the spark disappears.
        if spark.life <= 0.0 {
            commands.entity(spark_id).despawn();
        }
    }
}

// ======================================================
//  END THE PARTY — after a few seconds, clean up and
//  head to the next level!
// ======================================================

fn end_the_party(
    mut commands: Commands,
    time: Res<Time>,
    mut party: ResMut<Party>,
    mut game: ResMut<Game>,
    messages: Query<Entity, With<BigMessage>>,
) {
    if !party.happening {
        return;
    }

    // Count how long the party has lasted.
    party.timer += time.delta_secs();

    if party.timer > PARTY_SECONDS {
        party.happening = false;

        // Take down the big words.
        for message in &messages {
            commands.entity(message).despawn();
        }

        // Order up the next level!
        game.switch_to = Some(party.next_level);
    }
}

// ======================================================
//  THE CORNER WORDS — score, level number, boss hearts.
// ======================================================

fn update_words(
    time: Res<Time>,
    party: Res<Party>,
    game: Res<Game>,
    mut score: ResMut<Score>,
    bosses: Query<&Boss>,
    mut score_text: Query<&mut Text, (With<ScoreText>, Without<LevelText>)>,
    mut level_text: Query<&mut Text, (With<LevelText>, Without<ScoreText>)>,
) {
    // The score freezes during the party — you earned it!
    if !party.happening {
        // Add 10 points every second. (points + 10 × time)
        score.points = score.points + 10.0 * time.delta_secs();
    }

    for mut t in &mut score_text {
        // "as i32" chops off the decimal: 12.7 becomes 12.
        *t = Text::new(format!("Score: {}", score.points as i32));
    }

    for mut t in &mut level_text {
        // Is there a boss? Show his hearts too!
        if let Some(boss) = bosses.iter().next() {
            // "repeat" copies the heart once per health point.
            let hearts = "<3 ".repeat(boss.hearts.max(0) as usize);
            *t = Text::new(format!("BOSS  {hearts}"));
        } else {
            *t = Text::new(format!("Level {}", game.level));
        }
    }
}
