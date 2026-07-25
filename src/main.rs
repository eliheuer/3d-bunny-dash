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
//!   * Sometimes spikes sit ON TOP of cubes. Sneaky!
//!   * If you die, the whole level starts over. Instantly.
//!   * Reach the GOLDEN FINISH LINE at the end and you
//!     get FIREWORKS and a LEVEL COMPLETE screen!
//!
//! THE MATH YOU WILL LEARN:
//!   * ADDING     : position = position + speed
//!   * GRAVITY    : a number that pulls you DOWN every frame
//!   * DISTANCE   : how far apart two things are
//!   * COMPARING  : is 2 smaller than 5?  (2 < 5 is true!)
//!   * WAVES      : sine makes numbers wiggle up and down!

use bevy::prelude::*;

// ======================================================
//  NUMBERS THAT CONTROL THE GAME  (try changing these!)
// ======================================================

/// How fast the level slides toward the bunny (units per second).
const GAME_SPEED: f32 = 8.0;

/// How strong the bunny's jump is.
const JUMP_POWER: f32 = 9.0;

/// Gravity pulls the bunny down. On Earth gravity is 9.8!
const GRAVITY: f32 = 22.0;

/// How close a spike or bad guy must be to hurt the bunny.
const CRASH_DISTANCE: f32 = 1.0;

/// How big the platform cubes are (they are 1.2 on every side).
const CUBE_SIZE: f32 = 1.2;

/// How long the fireworks party lasts before the level restarts.
const PARTY_SECONDS: f32 = 5.0;

// ======================================================
//  KINDS OF LEVEL PIECES — a menu to choose from!
// ======================================================

#[derive(Clone, Copy)]
enum Piece {
    Spike,        // orange cone on the ground — JUMP!
    Cube,         // purple platform cube — land on top, or jump over!
    CubeWithSpike, // a cube with a spike on top — do NOT land here!
    SkySpike,     // blue upside-down cone in the air — DON'T jump!
    BadGuy,       // red bouncing ball — jump over him!
}

// ======================================================
//  THE LEVEL! — a list of (what piece, how far away).
//  This is the whole level design. Add your own lines!
//  Bigger minus numbers = farther down the road.
//  Cubes right next to each other make a long platform!
// ======================================================

const LEVEL: [(Piece, f32); 13] = [
    (Piece::Spike, -18.0),
    (Piece::Cube, -27.0),
    (Piece::Spike, -36.0),
    // Three cubes in a row make a bridge to run across!
    (Piece::Cube, -45.0),
    (Piece::Cube, -46.2),
    (Piece::Cube, -47.4),
    (Piece::SkySpike, -56.0),
    (Piece::BadGuy, -65.0),
    // The trickiest part! A plain cube comes FIRST — hop
    // onto it, then jump again to clear the spiky cube
    // right behind it. Two jumps, quick quick!
    (Piece::Cube, -74.0),
    (Piece::CubeWithSpike, -75.2),
    (Piece::Spike, -85.0),
    (Piece::SkySpike, -91.0),
    (Piece::Spike, -97.0),
];

/// Where the golden finish line waits, past the last piece!
const FINISH_LINE: f32 = -106.0;

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

/// Everything that slides along with the level wears this.
/// It remembers its starting spot so the level can restart.
#[derive(Component)]
struct LevelPiece {
    start_z: f32,
}

/// Things that kill the bunny when touched (spikes, bad guy).
#[derive(Component)]
struct Deadly;

/// Platform cubes: safe on TOP, deadly on the SIDE!
#[derive(Component)]
struct Platform;

/// The golden gate at the end of the level.
#[derive(Component)]
struct FinishLine;

/// An extra sticker for the bad guy who bounces.
#[derive(Component)]
struct Bouncing;

/// The sticker for the bunny's ears, so they can flop!
#[derive(Component)]
struct Ear;

/// One little glowing ball of firework spark!
#[derive(Component)]
struct Firework {
    /// Which way (and how fast) this spark is flying.
    velocity: Vec3,
    /// How many seconds of sparkle are left.
    life: f32,
}

/// The big "LEVEL COMPLETE!" words.
#[derive(Component)]
struct BigMessage;

/// The sticker for the score words on the screen.
#[derive(Component)]
struct ScoreText;

/// The score: how long you have survived!
#[derive(Resource)]
struct Score {
    points: f32,
}

/// Is the party happening? (Did we finish the level?)
#[derive(Resource)]
struct Party {
    happening: bool,
    timer: f32,
}

// ======================================================
//  MAIN — where the program starts, like page 1 of a book
// ======================================================

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Score { points: 0.0 })
        .insert_resource(Party {
            happening: false,
            timer: 0.0,
        })
        // Run ONCE when the game starts:
        .add_systems(Startup, build_the_world)
        // Run EVERY FRAME (about 60 times each second!).
        // ".chain()" means: run them in exactly this order.
        .add_systems(
            Update,
            (
                bunny_jump,
                flop_ears,
                move_level,
                spin_finish_line,
                bounce_bad_guys,
                check_for_crash,
                check_for_finish,
                sparkle_fireworks,
                end_the_party,
                update_score,
            )
                .chain(),
        )
        .run();
}

// ======================================================
//  BUILD THE WORLD — make the ground, bunny, level,
//  sun, and camera. Like setting up a toy playset!
// ======================================================

fn build_the_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ---------- COLORS (mixed from red, green, blue) ----------
    // Each number goes from 0.0 (none) to 1.0 (lots).
    // PINK = lots of red + some green + some blue.
    let pink = materials.add(Color::srgb(1.0, 0.4, 0.7));
    let light_pink = materials.add(Color::srgb(1.0, 0.7, 0.85));
    let green = materials.add(Color::srgb(0.3, 0.8, 0.4));
    let orange = materials.add(Color::srgb(1.0, 0.5, 0.1));
    let purple = materials.add(Color::srgb(0.6, 0.2, 0.9));
    let blue = materials.add(Color::srgb(0.2, 0.5, 1.0));
    let red = materials.add(Color::srgb(0.9, 0.1, 0.1));
    let gold = materials.add(Color::srgb(1.0, 0.85, 0.2));
    let white = materials.add(Color::srgb(1.0, 1.0, 1.0));

    // Shapes we will reuse a lot:
    let spike_shape = meshes.add(Cone::new(0.6, 1.4));
    let cube_shape = meshes.add(Cuboid::new(CUBE_SIZE, CUBE_SIZE, CUBE_SIZE));

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
            // Two tall ears! One at x = -0.15 (left), one at x = +0.15 (right).
            // They wear the Ear sticker so they can FLOP when we jump!
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

    // ---------- BUILD THE LEVEL ----------
    // Go through our LEVEL list one line at a time and
    // build the right shape for each kind of piece.
    for (piece, start_z) in LEVEL {
        match piece {
            // Orange spike sitting on the ground. JUMP!
            Piece::Spike => {
                commands.spawn((
                    LevelPiece { start_z },
                    Deadly,
                    Mesh3d(spike_shape.clone()),
                    MeshMaterial3d(orange.clone()),
                    Transform::from_xyz(0.0, 0.7, start_z),
                ));
            }
            // Purple platform cube. Land on TOP — not the side!
            Piece::Cube => {
                commands.spawn((
                    LevelPiece { start_z },
                    Platform,
                    Mesh3d(cube_shape.clone()),
                    MeshMaterial3d(purple.clone()),
                    // The cube's middle is at half its height.
                    Transform::from_xyz(0.0, CUBE_SIZE / 2.0, start_z),
                ));
            }
            // A cube wearing a spike hat. Do NOT land on this one!
            Piece::CubeWithSpike => {
                commands.spawn((
                    LevelPiece { start_z },
                    Platform,
                    Mesh3d(cube_shape.clone()),
                    MeshMaterial3d(purple.clone()),
                    Transform::from_xyz(0.0, CUBE_SIZE / 2.0, start_z),
                ));
                commands.spawn((
                    LevelPiece { start_z },
                    Deadly,
                    Mesh3d(spike_shape.clone()),
                    MeshMaterial3d(orange.clone()),
                    // The spike sits on top: cube height + half the spike.
                    Transform::from_xyz(0.0, CUBE_SIZE + 0.7, start_z),
                ));
            }
            // Blue upside-down spike FLOATING IN THE AIR.
            // It hangs right where a jumping bunny would be.
            // So DON'T jump. Run under it!
            Piece::SkySpike => {
                commands.spawn((
                    LevelPiece { start_z },
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
            // Jump over him!
            Piece::BadGuy => {
                commands
                    .spawn((
                        LevelPiece { start_z },
                        Deadly,
                        Bouncing,
                        Mesh3d(meshes.add(Sphere::new(0.55))),
                        MeshMaterial3d(red.clone()),
                        Transform::from_xyz(0.0, 0.55, start_z),
                        Visibility::default(),
                    ))
                    .with_children(|bad_guy| {
                        // Two angry white eyes so he looks like
                        // a bad guy and not just a ball!
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

    // ---------- THE GOLDEN FINISH LINE ----------
    // Two tall golden posts with a spinning golden star bar
    // on top. Touch it (well, reach it) and you WIN!
    commands
        .spawn((
            LevelPiece {
                start_z: FINISH_LINE,
            },
            FinishLine,
            Transform::from_xyz(0.0, 0.0, FINISH_LINE),
            Visibility::default(),
        ))
        .with_children(|gate| {
            // Left post at x = -2, right post at x = +2.
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
            // The bar across the top.
            gate.spawn((
                Mesh3d(meshes.add(Cuboid::new(4.4, 0.4, 0.4))),
                MeshMaterial3d(gold.clone()),
                Transform::from_xyz(0.0, 4.0, 0.0),
            ));
        });

    // ---------- THE SUN ----------
    // A light shining down so we can see, with shadows!
    commands.spawn((
        DirectionalLight {
            illuminance: 8_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // ---------- THE CAMERA ----------
    // The camera is our eye. It sits up high and behind
    // the bunny, looking at the bunny.
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(6.0, 5.0, 9.0).looking_at(Vec3::new(0.0, 1.0, -3.0), Vec3::Y),
    ));

    // ---------- THE SCORE WORDS ----------
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
}

// ======================================================
//  A LITTLE HELPER — how high is the floor under the
//  bunny right now? Usually 0 (the ground), but if a
//  platform cube is under us, the floor is the cube top!
// ======================================================

fn floor_height_under_bunny(
    platforms: &Query<&Transform, (With<Platform>, Without<Bunny>)>,
) -> f32 {
    let mut floor = 0.0;

    for platform in platforms {
        // How far away is this cube from the bunny (at z = 0)?
        // ".abs()" makes minus numbers plus: -3 becomes 3.
        let how_far = platform.translation.z.abs();

        // If the cube is under our feet (closer than 1 away)...
        if how_far < 1.0 {
            // ...the floor is the TOP of the cube!
            floor = CUBE_SIZE;
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
    platforms: Query<&Transform, (With<Platform>, Without<Bunny>)>,
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
        // Jumping just means: set our up-speed to a big number.
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

        // Did we fall onto the floor? Land there and stop falling.
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
        //
        // "clamp" keeps the number between -0.9 and 0.9,
        // so the ears never spin all the way around!
        let tilt = (bunny.up_speed * 0.08).clamp(-0.9, 0.9);

        for mut ear in &mut ears {
            // Tilt both ears by that amount.
            ear.rotation = Quat::from_rotation_x(tilt);
        }
    }
}

// ======================================================
//  MOVING THE LEVEL — every piece slides toward the bunny.
// ======================================================

fn move_level(
    time: Res<Time>,
    party: Res<Party>,
    mut pieces: Query<&mut Transform, With<LevelPiece>>,
) {
    // The level stops moving during the party!
    if party.happening {
        return;
    }

    for mut position in &mut pieces {
        // MOVING MATH again: new spot = old spot + speed × time.
        // z gets BIGGER, which means "coming toward the camera".
        position.translation.z = position.translation.z + GAME_SPEED * time.delta_secs();
    }
}

// ======================================================
//  The finish line's top bar slowly spins, so you can
//  see it sparkle from far away!
// ======================================================

fn spin_finish_line(time: Res<Time>, mut gates: Query<&mut Transform, With<FinishLine>>) {
    for mut gate in &mut gates {
        gate.rotate_y(0.8 * time.delta_secs());
    }
}

// ======================================================
//  BOUNCING — the bad guy hops using WAVE MATH!
// ======================================================

fn bounce_bad_guys(time: Res<Time>, mut bad_guys: Query<&mut Transform, With<Bouncing>>) {
    for mut position in &mut bad_guys {
        // SINE is a magic wave: as time goes on, sin(time)
        // wiggles smoothly between -1 and +1 forever.
        //
        //   sin(time × 2.0)  → wiggle 2 times faster
        //   × 0.5            → only wiggle 0.5 high, a small
        //                      hop, so the bunny can JUMP
        //                      OVER the bad guy!
        //   .abs()           → flip minus numbers to plus,
        //                      so it BOUNCES like a ball
        //                      instead of sinking underground!
        let wave = (time.elapsed_secs() * 2.0).sin().abs() * 0.5;

        // Height = resting height (0.55) + the wave.
        position.translation.y = 0.55 + wave;
    }
}

// ======================================================
//  CRASH CHECK — did the bunny touch a spike, the bad
//  guy, or the SIDE of a platform cube?
//  If yes: the bunny dies, level restarts. Instantly!
// ======================================================

fn check_for_crash(
    mut commands: Commands,
    sounds: Res<AssetServer>,
    mut score: ResMut<Score>,
    party: Res<Party>,
    mut bunnies: Query<(&mut Transform, &mut Bunny), Without<LevelPiece>>,
    mut pieces: Query<(&mut Transform, &LevelPiece, Has<Deadly>, Has<Platform>)>,
) {
    // Nothing can hurt you during the party!
    if party.happening {
        return;
    }

    for (mut bunny_position, mut bunny) in &mut bunnies {
        let mut crashed = false;

        for (piece_position, _, is_deadly, is_platform) in &pieces {
            // DISTANCE MATH: how far apart are the bunny
            // and this piece? Bevy measures it for us
            // (it uses the Pythagorean theorem — a² + b² = c²!)
            let distance = bunny_position
                .translation
                .distance(piece_position.translation);

            // Spikes and bad guys: COMPARING — if the distance
            // is SMALLER than 1.0, they are touching. CRASH!
            if is_deadly && distance < CRASH_DISTANCE {
                crashed = true;
            }

            // Platform cubes: only the SIDE is dangerous!
            // If the cube is at our z spot AND our body is
            // LOWER than the cube's top, we smacked the side.
            if is_platform {
                let close_in_z = piece_position.translation.z.abs() < 1.0;
                let too_low = bunny_position.translation.y < CUBE_SIZE - 0.15;
                if close_in_z && too_low {
                    crashed = true;
                }
            }
        }

        if crashed {
            // Play the sad crash sound. WAH...
            commands.spawn((
                AudioPlayer::new(sounds.load("die.wav")),
                PlaybackSettings::DESPAWN,
            ));

            // The bunny died! Restart the level from the top:
            // 1. Score goes back to zero.
            score.points = 0.0;
            // 2. The bunny lands back on the ground.
            bunny_position.translation.y = 0.5;
            bunny.up_speed = 0.0;
            // 3. EVERY piece snaps back to the starting
            //    spot it remembered on its sticker.
            for (mut piece_position, piece, _, _) in &mut pieces {
                piece_position.translation.z = piece.start_z;
            }
        }
    }
}

// ======================================================
//  FINISH CHECK — did the bunny reach the golden gate?
//  Then it's PARTY TIME! Fireworks + LEVEL COMPLETE!
// ======================================================

fn check_for_finish(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut party: ResMut<Party>,
    gates: Query<&Transform, With<FinishLine>>,
) {
    if party.happening {
        return;
    }

    for gate in &gates {
        // Has the gate slid all the way to the bunny (z = 0)?
        if gate.translation.z >= 0.0 {
            // WE WON! Start the party!
            party.happening = true;
            party.timer = 0.0;

            // ----- The big words on the screen -----
            commands.spawn((
                BigMessage,
                Text::new("LEVEL COMPLETE!"),
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
            // number i, we turn i into an angle, and use
            // cos & sin to aim around the circle — that's how
            // math draws circles!
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
                            angle.cos() * speed,        // sideways
                            4.0 + ((i % 5) as f32),     // upward!
                            angle.sin() * speed - 2.0,  // forward/back
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
//  start the level again from the beginning!
// ======================================================

fn end_the_party(
    mut commands: Commands,
    time: Res<Time>,
    mut party: ResMut<Party>,
    mut score: ResMut<Score>,
    messages: Query<Entity, With<BigMessage>>,
    mut pieces: Query<(&mut Transform, &LevelPiece)>,
) {
    if !party.happening {
        return;
    }

    // Count how long the party has lasted.
    party.timer += time.delta_secs();

    // Party over after PARTY_SECONDS (5 seconds).
    if party.timer > PARTY_SECONDS {
        party.happening = false;
        score.points = 0.0;

        // Take down the LEVEL COMPLETE words.
        for message in &messages {
            commands.entity(message).despawn();
        }

        // Send every level piece back to its starting spot.
        for (mut position, piece) in &mut pieces {
            position.translation.z = piece.start_z;
        }
    }
}

// ======================================================
//  SCORE — the longer you survive, the more points!
// ======================================================

fn update_score(
    time: Res<Time>,
    party: Res<Party>,
    mut score: ResMut<Score>,
    mut text: Query<&mut Text, With<ScoreText>>,
) {
    // The score freezes during the party — you earned it!
    if !party.happening {
        // Add 10 points every second. (points + 10 × time)
        score.points = score.points + 10.0 * time.delta_secs();
    }

    for mut t in &mut text {
        // Turn the number into words for the screen.
        // "as i32" chops off the decimal part: 12.7 becomes 12.
        *t = Text::new(format!("Score: {}", score.points as i32));
    }
}
