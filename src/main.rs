//! ======================================================
//!               3 D   B U N N Y   D A S H
//! ======================================================
//! A game about a pink bunny who runs forever and
//! jumps over obstacles. Press ANY key (or SPACE) to jump!
//!
//! THE GAME HAS FOUR SCREENS:
//!   * TITLE    — the big menu (press 1, 2, or 3!)
//!   * PLAYING  — run, jump, dodge, WIN!
//!   * SETTINGS — choose your starting level
//!   * EDITOR   — build your own levels, like
//!                Geometry Dash and Mario Maker!
//!
//! THE RULES (just like real Geometry Dash!):
//!   * SPIKES kill you if you touch them at all.
//!   * CUBES are PLATFORMS: you can LAND ON TOP of them,
//!     but if you smack into the SIDE... you die!
//!   * If you die, the level starts over. Instantly.
//!   * Reach the GOLDEN FINISH LINE to beat the level.
//!   * NINE levels and THREE bosses: after every three
//!     levels a boss appears! Rotten Tomato, then the
//!     Cursed Thorn, then the FINAL boss... BAD BAT!
//!
//! THE MATH YOU WILL LEARN:
//!   * ADDING     : position = position + speed
//!   * GRAVITY    : a number that pulls you DOWN every frame
//!   * DISTANCE   : how far apart two things are
//!   * COMPARING  : is 2 smaller than 5?  (2 < 5 is true!)
//!   * WAVES      : sine makes numbers wiggle up and down!
//!   * REMAINDERS : 7 dodged, groups of 3 → 1 left over

use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;

// The other chapters of our code:
mod editor; // the level editor!
mod levels; // reads and writes assets/levels.txt
mod title; // the title screen and settings screen

use levels::LevelBook;

// ======================================================
//  NUMBERS THAT CONTROL THE GAME  (try changing these!)
// ======================================================

/// ~~~ SECRET CHEAT CODE ~~~
/// Which level "Play" starts on! You can also change
/// this inside the game on the SETTINGS screen.
const STARTING_LEVEL: usize = 1;

/// How strong the bunny's jump is.
const JUMP_POWER: f32 = 9.0;

/// Gravity pulls the bunny down. On Earth gravity is 9.8!
const GRAVITY: f32 = 22.0;

/// How close a spike or bad guy must be to hurt the bunny.
const CRASH_DISTANCE: f32 = 1.0;

/// How big the platform cubes are (1.2 on every side).
pub const CUBE_SIZE: f32 = 1.2;

/// How long the fireworks party lasts between levels.
const PARTY_SECONDS: f32 = 5.0;

/// How many hearts each boss has.
const BOSS_HEARTS: i32 = 3;

/// Dodge this many boss shots to knock off one heart.
const DODGES_PER_HEART: i32 = 3;

// ======================================================
//  THE PAINTBOX — every color in the game comes from
//  this one little set, so everything matches! Like a
//  box of preschool paints: bright, friendly, and
//  softened just a touch so they look good together.
// ======================================================

pub const RED: Color = Color::srgb(0.90, 0.24, 0.21);
pub const ORANGE: Color = Color::srgb(0.96, 0.58, 0.12);
pub const YELLOW: Color = Color::srgb(1.0, 0.80, 0.20);
pub const GREEN: Color = Color::srgb(0.35, 0.72, 0.35);
pub const BLUE: Color = Color::srgb(0.25, 0.55, 0.95);
pub const PURPLE: Color = Color::srgb(0.58, 0.40, 0.85);
pub const PINK: Color = Color::srgb(0.97, 0.51, 0.71);
pub const LIGHT_PINK: Color = Color::srgb(1.0, 0.76, 0.86);
pub const BROWN: Color = Color::srgb(0.55, 0.36, 0.20);
pub const INK: Color = Color::srgb(0.16, 0.13, 0.17);
pub const WHITE: Color = Color::srgb(1.0, 1.0, 1.0);

// ======================================================
//  THE SCREENS — which part of the game are we on?
// ======================================================

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum Screen {
    #[default]
    Title,
    Settings,
    Playing,
    Editor,
}

// ======================================================
//  KINDS OF LEVEL PIECES — a menu to choose from!
//  (The level lists in assets/levels.txt use these.)
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
/// this sticker, so we can sweep it all away in one go.
#[derive(Component)]
pub struct LevelStuff;

/// Things that only exist while PLAYING (the bunny,
/// the score words) — swept away when we leave.
#[derive(Component)]
struct PlayStuff;

/// Things that slide along as the level scrolls.
#[derive(Component)]
struct Scrolls;

/// Things that kill the bunny when touched.
#[derive(Component)]
struct Deadly;

/// Platform cubes: safe on TOP, deadly on the SIDE!
/// Each one remembers how tall its top is.
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
/// Each ear knows which side it's on: -1 is the left
/// ear, +1 is the right ear, so they flop APART.
#[derive(Component)]
struct Ear {
    side: f32,
}

/// The one and only camera (the background is its child).
#[derive(Component)]
pub struct MainCamera;

/// Which boss is this?
#[derive(Clone, Copy, PartialEq)]
enum BossKind {
    RottenTomato, // boss 1: a giant moldy tomato — spits seeds!
    CursedThorn,  // boss 2: a spiky rose in a pot — shoots thorns!
    BadBat,       // the FINAL boss: fast, tricky... and LASERS!
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
#[derive(Component)]
struct BossShot {
    velocity: Vec3,
}

/// One little glowing ball of firework spark!
#[derive(Component)]
struct Firework {
    velocity: Vec3,
    life: f32,
}

/// The big words in the middle of the screen.
#[derive(Component)]
struct BigMessage;

/// The score words in the corner.
#[derive(Component)]
struct ScoreText;

/// The "Level 2" / boss hearts words in the corner.
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
pub struct Game {
    pub level: usize,
    pub switch_to: Option<usize>,
}

/// Is the fireworks party happening?
#[derive(Resource)]
struct Party {
    happening: bool,
    timer: f32,
    /// The level to go to when the party ends.
    next_level: usize,
}

/// Choices you can change on the SETTINGS screen.
#[derive(Resource)]
pub struct Settings {
    pub starting_level: usize,
}

/// Our fancy space font (Planet Kosmos) for big titles
/// and scores, loaded once and shared everywhere.
#[derive(Resource)]
pub struct GameFont(pub Handle<Font>);

/// Our easy-reading font (Virtua Grotesk — made by Dad!)
/// for smaller words like the editor's help text.
#[derive(Resource)]
pub struct ReadingFont(pub Handle<Font>);

/// The game's camera spot. The title screen and the game
/// both ask here, so they always match.
pub fn action_camera() -> Transform {
    // Aiming a little HIGHER (y = 2.2) tips the camera up,
    // which slides the whole scene DOWN in the window —
    // room for the tall bosses and flying bats up top!
    // And standing a bit closer makes the bunny nice
    // and big on screen.
    Transform::from_xyz(4.9, 4.4, 7.4).looking_at(Vec3::new(0.0, 2.2, -3.0), Vec3::Y)
}

// ======================================================
//  THE LAVA LAMP BACKGROUND — a custom shader material!
//  The real magic is in assets/lava_lamp.wgsl, a tiny
//  program that runs on the GRAPHICS CARD and paints
//  every dot of the background with wavy lava math.
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
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "3D Bunny Dash".to_string(),
            ..default()
        }),
        ..default()
    }));

    // Load our fonts before ANYTHING runs, so every
    // screen can count on them being ready.
    let font = app
        .world()
        .resource::<AssetServer>()
        .load("fonts/PlanetKosmos.ttf");
    app.insert_resource(GameFont(font));
    let reading = app
        .world()
        .resource::<AssetServer>()
        .load("fonts/VirtuaGrotesk.ttf");
    app.insert_resource(ReadingFont(reading));

    app.add_plugins(MaterialPlugin::<LavaLampMaterial>::default())
        .add_plugins(title::TitlePlugin)
        .add_plugins(editor::EditorPlugin)
        .init_state::<Screen>()
        // Read assets/levels.txt into the level book!
        .insert_resource(levels::load_level_book())
        .insert_resource(Score { points: 0.0 })
        .insert_resource(Settings {
            starting_level: STARTING_LEVEL,
        })
        .insert_resource(Game {
            level: 1,
            switch_to: None,
        })
        .insert_resource(Party {
            happening: false,
            timer: 0.0,
            next_level: 1,
        })
        // Run ONCE when the game starts:
        .add_systems(Startup, build_the_world)
        // Run when we START and STOP playing:
        .add_systems(OnEnter(Screen::Playing), start_playing)
        .add_systems(OnExit(Screen::Playing), stop_playing)
        // Run EVERY FRAME — but only while PLAYING!
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
                back_to_menu,
            )
                .chain()
                .run_if(in_state(Screen::Playing)),
        )
        .run();
}

// ======================================================
//  BUILD THE WORLD — the things that are ALWAYS there:
//  ground, sun, camera, and the lava lamp background.
// ======================================================

fn build_the_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut lava: ResMut<Assets<LavaLampMaterial>>,
) {

    // ---------- THE GROUND ----------
    // A big flat box: 8 wide, nice and long, thin like a
    // pancake. It runs from behind the bunny to far ahead...
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(8.0, 0.2, 140.0))),
        MeshMaterial3d(materials.add(GREEN)),
        Transform::from_xyz(0.0, -0.1, -55.0),
    ));
    // ...and then FADES AWAY into the swirly background!
    // The trick: 12 more slices of road, each one a bit
    // more see-through than the last. Slice 0 is almost
    // solid, slice 11 is almost invisible — a gradient!
    let slice_shape = meshes.add(Cuboid::new(8.0, 0.2, 6.0));
    for i in 0..12 {
        // FRACTION MATH: i ÷ 12 crawls from 0.0 up toward
        // 1.0, so "1 - that" fades from solid to nothing.
        let how_solid = 1.0 - (i as f32 / 12.0);
        commands.spawn((
            Mesh3d(slice_shape.clone()),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: GREEN.with_alpha(how_solid),
                alpha_mode: AlphaMode::Blend, // see-through-able!
                ..default()
            })),
            // Each slice is 6 long, parked right after the
            // solid road ends at 125.
            Transform::from_xyz(0.0, -0.1, -125.0 - 3.0 - i as f32 * 6.0),
        ));
    }

    // ---------- THE LIGHTS ----------
    // Real-world shadows are never pitch black, because
    // light BOUNCES off everything — the sky, the ground —
    // and sneaks into the shady spots. 3D artists call
    // this "ambient light", and copy it with three tricks:
    //
    // 1. AMBIENT light: a soft glow from everywhere at
    //    once, so no corner is ever fully dark. (It rides
    //    on the camera, below.)
    // 2. The KEY light: the sun. The only shadow-maker.
    commands.spawn((
        DirectionalLight {
            illuminance: 6_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    // 3. The FILL light: a gentler sun from the OTHER
    //    side, with no shadows — it plays the part of
    //    light bouncing back up off the ground.
    commands.spawn((
        DirectionalLight {
            illuminance: 2_500.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(-6.0, 4.0, -2.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // ---------- THE CAMERA (with the background stuck on!) ----------
    commands
        .spawn((
            MainCamera,
            Camera3d::default(),
            // The soft glow-from-everywhere ambient light.
            AmbientLight {
                color: Color::srgb(0.9, 0.9, 1.0), // a whisper of sky blue
                brightness: 400.0,
                ..default()
            },
            action_camera(),
        ))
        .with_children(|camera| {
            // The lava lamp background is a CHILD of the
            // camera — like a poster taped WAY out in front
            // of the lens — so wherever the camera goes,
            // the background fills the whole screen!
            // (Extra big, so even the wide lens can't see
            // past its edges.)
            camera.spawn((
                Mesh3d(meshes.add(Rectangle::new(3600.0, 1500.0))),
                MeshMaterial3d(lava.add(LavaLampMaterial {})),
                Transform::from_xyz(0.0, 0.0, -600.0),
            ));
        });
}

// ======================================================
//  SHARED SPAWN HELPERS — build a bunny or the thorn
//  plant anywhere. The game AND the title screen use
//  these, so we only write them once!
// ======================================================

/// Build a pink bunny out of simple shapes and put it
/// wherever "place" says. Gives back the bunny's id.
pub fn spawn_bunny_visual(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    place: Transform,
) -> Entity {
    let pink = materials.add(PINK);
    let light_pink = materials.add(LIGHT_PINK);
    let white = materials.add(WHITE);

    commands
        .spawn((place, Visibility::default()))
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
            // Two shiny dark eyes on the front of the head!
            let eye_dark = materials.add(INK);
            bunny.spawn((
                Mesh3d(meshes.add(Sphere::new(0.07))),
                MeshMaterial3d(eye_dark.clone()),
                Transform::from_xyz(-0.13, 0.63, -0.6),
            ));
            bunny.spawn((
                Mesh3d(meshes.add(Sphere::new(0.07))),
                MeshMaterial3d(eye_dark.clone()),
                Transform::from_xyz(0.13, 0.63, -0.6),
            ));
            // Two tall ears that FLOP APART when we jump!
            // Each ear is really TWO pieces: an invisible
            // hinge planted in the head, and the ear shape
            // hanging above it. When we turn the hinge,
            // the whole ear swings from its bottom — just
            // like a real ear stuck into a real head!
            for side in [-1.0, 1.0] {
                bunny
                    .spawn((
                        Ear { side },
                        // The hinge sits right at the head top.
                        Transform::from_xyz(side * 0.15, 0.8, -0.3),
                        Visibility::default(),
                    ))
                    .with_children(|ear| {
                        // The ear shape, standing on the hinge
                        // (its middle is 0.33 above the bottom).
                        ear.spawn((
                            Mesh3d(meshes.add(Capsule3d::new(0.08, 0.5))),
                            MeshMaterial3d(light_pink.clone()),
                            Transform::from_xyz(0.0, 0.33, 0.0),
                        ));
                    });
            }
            // A fluffy white tail on the back
            bunny.spawn((
                Mesh3d(meshes.add(Sphere::new(0.18))),
                MeshMaterial3d(white.clone()),
                Transform::from_xyz(0.0, 0.0, 0.55),
            ));
        })
        .id()
}

/// Build the Cursed Thorn plant: pot, stem, thorns, and
/// the evil rose on top. Gives back the plant's id.
pub fn spawn_thorn_plant(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    place: Transform,
) -> Entity {
    let brown = materials.add(BROWN);
    let stem_green = materials.add(GREEN);
    let rose_pink = materials.add(PINK);
    let thorn_gray = materials.add(Color::srgb(0.45, 0.45, 0.4));
    let yellow = materials.add(YELLOW);
    let white = materials.add(WHITE);

    commands
        .spawn((place, Visibility::default()))
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
            // Sharp thorns on the stem, pointing left and
            // right, taking turns (i % 2: even or odd?).
            for i in 0..4 {
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
                MeshMaterial3d(yellow.clone()),
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
        })
        .id()
}

/// Build the Rotten Tomato: a big moldy tomato with a
/// stem and leaves on top. Gives back the tomato's id.
pub fn spawn_tomato_visual(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    place: Transform,
) -> Entity {
    let tomato_red = materials.add(RED);
    let leaf_green = materials.add(GREEN);
    let stem_brown = materials.add(BROWN);
    let white = materials.add(WHITE);

    commands
        .spawn((place, Visibility::default()))
        .with_children(|tomato| {
            // The big squishy tomato body.
            tomato.spawn((
                Mesh3d(meshes.add(Sphere::new(1.4))),
                MeshMaterial3d(tomato_red.clone()),
                Transform::from_xyz(0.0, 0.0, 0.0),
            ));
            // The stem on top...
            tomato.spawn((
                Mesh3d(meshes.add(Capsule3d::new(0.12, 0.5))),
                MeshMaterial3d(stem_brown.clone()),
                Transform::from_xyz(0.0, 1.6, 0.0),
            ));
            // ...and floppy tomato leaves around the stem,
            // tipped outward like a little green star.
            for i in 0..5 {
                let angle = i as f32 * 6.28 / 5.0;
                tomato.spawn((
                    Mesh3d(meshes.add(Cone::new(0.22, 0.7))),
                    MeshMaterial3d(leaf_green.clone()),
                    Transform::from_xyz(angle.cos() * 0.5, 1.35, angle.sin() * 0.5)
                        // Lean each leaf away from the stem.
                        .with_rotation(Quat::from_rotation_z(-angle.cos() * 1.2)
                            * Quat::from_rotation_x(angle.sin() * 1.2)),
                ));
            }
            // Giant angry eyes!
            tomato.spawn((
                Mesh3d(meshes.add(Sphere::new(0.3))),
                MeshMaterial3d(white.clone()),
                Transform::from_xyz(-0.5, 0.4, 1.15),
            ));
            tomato.spawn((
                Mesh3d(meshes.add(Sphere::new(0.3))),
                MeshMaterial3d(white.clone()),
                Transform::from_xyz(0.5, 0.4, 1.15),
            ));
        })
        .id()
}

/// Build BAD BAT: a dark bat with big wings, pointy ears,
/// and glowing red eyes. Gives back the bat's id.
pub fn spawn_bat_visual(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    place: Transform,
) -> Entity {
    let bat_dark = materials.add(INK);
    let eye_red = materials.add(StandardMaterial {
        base_color: RED,
        emissive: LinearRgba::new(4.0, 0.2, 0.2, 1.0), // GLOWING eyes!
        ..default()
    });

    commands
        .spawn((place, Visibility::default()))
        .with_children(|bat| {
            // The round dark body.
            bat.spawn((
                Mesh3d(meshes.add(Sphere::new(0.9))),
                MeshMaterial3d(bat_dark.clone()),
                Transform::from_xyz(0.0, 0.0, 0.0),
            ));
            // Two BIG flat wings, one each side, tilted up.
            // (A wing is just a very squashed box!)
            bat.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.2, 0.1, 1.1))),
                MeshMaterial3d(bat_dark.clone()),
                Transform::from_xyz(-1.5, -0.1, 0.0)
                    .with_rotation(Quat::from_rotation_z(0.25)),
            ));
            bat.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.2, 0.1, 1.1))),
                MeshMaterial3d(bat_dark.clone()),
                Transform::from_xyz(1.5, -0.1, 0.0)
                    .with_rotation(Quat::from_rotation_z(-0.25)),
            ));
            // Two pointy ears on top.
            bat.spawn((
                Mesh3d(meshes.add(Cone::new(0.2, 0.6))),
                MeshMaterial3d(bat_dark.clone()),
                Transform::from_xyz(-0.4, 1.0, 0.0),
            ));
            bat.spawn((
                Mesh3d(meshes.add(Cone::new(0.2, 0.6))),
                MeshMaterial3d(bat_dark.clone()),
                Transform::from_xyz(0.4, 1.0, 0.0),
            ));
            // Glowing red eyes. Spooky!
            bat.spawn((
                Mesh3d(meshes.add(Sphere::new(0.14))),
                MeshMaterial3d(eye_red.clone()),
                Transform::from_xyz(-0.3, 0.15, 0.8),
            ));
            bat.spawn((
                Mesh3d(meshes.add(Sphere::new(0.14))),
                MeshMaterial3d(eye_red.clone()),
                Transform::from_xyz(0.3, 0.15, 0.8),
            ));
        })
        .id()
}

/// Build ONE level piece at one spot on the road.
/// The game builds whole levels with this, and the
/// LEVEL EDITOR uses it to show pieces as you place them!
pub fn spawn_piece(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    piece: Piece,
    start_z: f32,
) {
    let orange = materials.add(ORANGE);
    let purple = materials.add(PURPLE);
    let blue = materials.add(BLUE);
    let red = materials.add(RED);
    let white = materials.add(WHITE);
    let spike_shape = meshes.add(Cone::new(0.6, 1.4));

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
                Mesh3d(meshes.add(Cuboid::new(CUBE_SIZE, CUBE_SIZE, CUBE_SIZE))),
                MeshMaterial3d(purple.clone()),
                Transform::from_xyz(0.0, CUBE_SIZE / 2.0, start_z),
            ));
        }
        // A DOUBLE-TALL cube! Climb up from a normal cube.
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
        // A TRIPLE-TALL cube! Only reachable from a tall one.
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
                Mesh3d(meshes.add(Cuboid::new(CUBE_SIZE, CUBE_SIZE, CUBE_SIZE))),
                MeshMaterial3d(purple.clone()),
                Transform::from_xyz(0.0, CUBE_SIZE / 2.0, start_z),
            ));
            commands.spawn((
                LevelStuff,
                Scrolls,
                Deadly,
                Mesh3d(spike_shape.clone()),
                MeshMaterial3d(orange.clone()),
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

// ======================================================
//  STARTING AND STOPPING PLAY
// ======================================================

fn start_playing(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    settings: Res<Settings>,
    the_editor: Res<editor::Editor>,
    font: Res<GameFont>,
    mut game: ResMut<Game>,
    mut score: ResMut<Score>,
    mut cameras: Query<&mut Transform, With<MainCamera>>,
) {
    // Point the camera at the action.
    for mut camera in &mut cameras {
        *camera = action_camera();
    }

    score.points = 0.0;

    // Which level? Usually the settings choice — but if
    // we came from the EDITOR's playtest button, play
    // the level being edited!
    let start = if the_editor.playtesting {
        the_editor.level
    } else {
        settings.starting_level
    };
    game.switch_to = Some(start);

    // Spawn the bunny, ready to run!
    let bunny = spawn_bunny_visual(
        &mut commands,
        &mut meshes,
        &mut materials,
        Transform::from_xyz(0.0, 0.5, 0.0),
    );
    commands
        .entity(bunny)
        .insert((Bunny { up_speed: 0.0 }, PlayStuff));

    // The corner words.
    commands.spawn((
        PlayStuff,
        ScoreText,
        Text::new("Score: 0"),
        TextFont {
            font: font.0.clone(),
            font_size: 48.0,
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
        PlayStuff,
        LevelText,
        Text::new("Level 1"),
        TextFont {
            font: font.0.clone(),
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

/// Sweep away everything play-related when we leave.
fn stop_playing(
    mut commands: Commands,
    mut party: ResMut<Party>,
    stuff: Query<
        Entity,
        Or<(
            With<PlayStuff>,
            With<LevelStuff>,
            With<BigMessage>,
            With<Firework>,
        )>,
    >,
) {
    party.happening = false;
    for thing in &stuff {
        commands.entity(thing).despawn();
    }
}

/// Press ESCAPE while playing to go back — to the title
/// screen, or to the editor if we were playtesting.
fn back_to_menu(
    keyboard: Res<ButtonInput<KeyCode>>,
    the_editor: Res<editor::Editor>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        if the_editor.playtesting {
            next_screen.set(Screen::Editor);
        } else {
            next_screen.set(Screen::Title);
        }
    }
}

// ======================================================
//  SWITCH LEVEL — sweep away the old level and build
//  the new one from the level book!
// ======================================================

fn switch_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut game: ResMut<Game>,
    book: Res<LevelBook>,
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

    // Build every piece from the level book's list.
    for (piece, start_z) in book.get(new_level).pieces.clone() {
        spawn_piece(&mut commands, &mut meshes, &mut materials, piece, start_z);
    }

    let gold = materials.add(YELLOW);

    // The level book tells us if a boss lives on this
    // stage — and which one!
    let boss_word = book.get(new_level).boss.clone();

    if !boss_word.is_empty() {
        // ---------- A BOSS STAGE! ----------
        // Build the right villain for this stage.
        let start = Transform::from_xyz(0.0, 0.0, -16.0);
        let (villain, kind, hearts) = match boss_word.as_str() {
            // The moldy menace. Floats and spits seeds.
            "tomato" => (
                spawn_tomato_visual(&mut commands, &mut meshes, &mut materials, start),
                BossKind::RottenTomato,
                BOSS_HEARTS,
            ),
            // The spooky rose. Slides and shoots thorns.
            "thorn" => (
                spawn_thorn_plant(&mut commands, &mut meshes, &mut materials, start),
                BossKind::CursedThorn,
                BOSS_HEARTS,
            ),
            // BAD BAT — one EXTRA heart, because he's the
            // final boss and he knows it.
            _ => (
                spawn_bat_visual(&mut commands, &mut meshes, &mut materials, start),
                BossKind::BadBat,
                BOSS_HEARTS + 1,
            ),
        };
        commands.entity(villain).insert((
            LevelStuff,
            Boss {
                kind,
                hearts,
                throw_timer: 0.0,
                shots_dodged: 0,
            },
        ));
    } else {
        // ---------- THE GOLDEN FINISH LINE ----------
        // Normal levels end at a golden gate. Reach it to win!
        commands
            .spawn((
                LevelStuff,
                Scrolls,
                FinishLine,
                Transform::from_xyz(0.0, 0.0, book.get(new_level).finish),
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
        // the top of a platform cube?
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

fn flop_ears(time: Res<Time>, bunnies: Query<&Bunny>, mut ears: Query<(&mut Transform, &Ear)>) {
    for bunny in &bunnies {
        // MULTIPLYING MATH: up-speed × 0.13 turns a big
        // speed (like 9) into a big floppy tilt (like 1.2).
        //
        // The clamp starts at ZERO — ears only ever flop
        // OUTWARD, never inward, so they can never cross
        // through each other in the middle!
        let flop = (bunny.up_speed * 0.13).clamp(0.0, 1.2);

        for (mut hinge, ear) in &mut ears {
            // Where each ear WANTS to be: swung sideways,
            // left ear left, right ear right (that's what
            // multiplying by side = -1 or +1 does).
            let target = Quat::from_rotation_z(ear.side * -flop);

            // "slerp" = slide only PART of the way there
            // each frame. That makes the ears lag behind
            // and settle gently — floppy, not snappy!
            let how_fast = (10.0 * time.delta_secs()).min(1.0);
            hinge.rotation = hinge.rotation.slerp(target, how_fast);
        }
    }
}

// ======================================================
//  MOVING THE LEVEL — every piece slides toward the
//  bunny, at the speed written in the level book.
// ======================================================

fn move_level(
    time: Res<Time>,
    party: Res<Party>,
    game: Res<Game>,
    book: Res<LevelBook>,
    mut pieces: Query<&mut Transform, With<Scrolls>>,
) {
    // The level stops moving during the party!
    if party.happening {
        return;
    }

    let speed = book.get(game.level).speed;

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
//  THE BOSS FIGHTS! They float, they glare, they THROW.
// ======================================================

fn boss_fight(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
    sounds: Res<AssetServer>,
    font: Res<GameFont>,
    game: Res<Game>,
    mut party: ResMut<Party>,
    mut bosses: Query<(&mut Boss, &mut Transform)>,
) {
    if party.happening {
        return;
    }

    for (mut boss, mut position) in &mut bosses {
        // Each boss moves in its own spooky way (wave math!).
        match boss.kind {
            // The Rotten Tomato bobs up and down menacingly.
            BossKind::RottenTomato => {
                position.translation.y = 2.2 + (time.elapsed_secs() * 1.5).sin() * 0.4;
            }
            // The Cursed Thorn slides SIDE TO SIDE in its
            // pot, so its thorns come at you from angles!
            BossKind::CursedThorn => {
                position.translation.x = (time.elapsed_secs() * 0.9).sin() * 2.5;
            }
            // BAD BAT swoops EVERY which way — two different
            // waves at once, sideways AND up-down. Tricky!
            BossKind::BadBat => {
                position.translation.x = (time.elapsed_secs() * 1.3).sin() * 3.0;
                position.translation.y = 3.0 + (time.elapsed_secs() * 2.1).sin() * 1.3;
            }
        }

        // Count up to the next throw. The FEWER hearts
        // left, the FASTER it throws. Bosses get mad!
        boss.throw_timer += time.delta_secs();
        let time_between_throws = 0.8 + boss.hearts as f32 * 0.5;

        if boss.throw_timer > time_between_throws {
            boss.throw_timer = 0.0;

            match boss.kind {
                // The Rotten Tomato spits a big BLACK SEED
                // that rolls straight at the bunny.
                BossKind::RottenTomato => {
                    commands.spawn((
                        LevelStuff,
                        Deadly,
                        BossShot {
                            velocity: Vec3::new(0.0, 0.0, 9.0),
                        },
                        Mesh3d(meshes.add(Sphere::new(0.35))),
                        MeshMaterial3d(materials.add(INK)),
                        Transform::from_xyz(0.0, 0.35, position.translation.z),
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
                            .with_rotation(Quat::from_rotation_arc(
                                Vec3::Y,
                                velocity.normalize(),
                            )),
                    ));
                }
                // BAD BAT fires a LASER — long, glowing, and
                // FAST (speed 15!) — aimed from wherever he
                // swooped to, straight at the bunny's spot.
                BossKind::BadBat => {
                    let mouth = position.translation + Vec3::new(0.0, -0.2, 0.8);
                    let aim_at = Vec3::new(0.0, 0.5, 0.0);
                    let velocity = (aim_at - mouth).normalize() * 15.0;

                    commands.spawn((
                        LevelStuff,
                        Deadly,
                        BossShot { velocity },
                        // A long skinny glowing red beam!
                        Mesh3d(meshes.add(Cuboid::new(0.15, 0.15, 2.2))),
                        MeshMaterial3d(materials.add(StandardMaterial {
                            base_color: RED,
                            emissive: LinearRgba::new(6.0, 0.4, 0.4, 1.0),
                            ..default()
                        })),
                        Transform::from_translation(mouth)
                            // Point the beam the way it flies!
                            .with_rotation(Quat::from_rotation_arc(
                                Vec3::Z,
                                velocity.normalize(),
                            )),
                    ));
                }
            }
        }

        // Did we take away ALL its hearts? WE WIN!
        if boss.hearts <= 0 {
            let (message, next_level) = match boss.kind {
                // Beat BAD BAT → you won the whole game!
                BossKind::BadBat => ("YOU WIN THE WHOLE GAME!", 1),
                // Beat any other boss → on to the next stage!
                _ => ("BOSS DEFEATED!", game.level + 1),
            };
            start_party(
                &mut commands,
                &mut meshes,
                &mut materials,
                &sounds,
                &font,
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

        // Spikes, bad guys, boss shots: DISTANCE MATH!
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
    sounds: Res<AssetServer>,
    font: Res<GameFont>,
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
                &sounds,
                &font,
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
    sounds: &AssetServer,
    font: &GameFont,
    party: &mut Party,
    message: &str,
    next_level: usize,
) {
    party.happening = true;
    party.timer = 0.0;
    party.next_level = next_level;

    // Play the victory fanfare! Ta-da-da-DAAA!
    commands.spawn((
        AudioPlayer::new(sounds.load("win.wav")),
        PlaybackSettings::DESPAWN,
    ));

    // ----- The big words on the screen -----
    commands.spawn((
        BigMessage,
        Text::new(message),
        TextFont {
            font: font.0.clone(),
            font_size: 100.0,
            ..default()
        },
        TextColor(YELLOW),
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
    // Fireworks in every paintbox color!
    let firework_colors = [RED, ORANGE, YELLOW, GREEN, BLUE, PURPLE, PINK];
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
//  THE CORNER WORDS — score, level name, boss hearts.
// ======================================================

fn update_words(
    time: Res<Time>,
    party: Res<Party>,
    game: Res<Game>,
    book: Res<LevelBook>,
    mut score: ResMut<Score>,
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
        // Every screen asks the level book for the stage's
        // one true name tag — "Level 4: Sky Stairs" or
        // "Boss 1: Rotten Tomato" — so the counting always
        // matches everywhere!
        *t = Text::new(book.label(game.level));
    }
}
