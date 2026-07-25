//! ======================================================
//!  THE LEVEL EDITOR — build your own levels!
//! ======================================================
//! Just like Geometry Dash and Mario Maker, but simple:
//!
//!   *  ← / →   glide the golden cursor along the road
//!   *  1..7    place a piece where the cursor is
//!   *  X       delete the piece under the cursor
//!   *  ↑ / ↓   pick which level you're editing
//!   *  P       PLAYTEST the level right now!
//!   *  S       SAVE everything into assets/levels.txt
//!   *  Esc     back to the title screen
//!
//! The editor changes the LEVEL BOOK in the computer's
//! memory. Playtesting uses your changes instantly —
//! but only SAVING (S) writes them into the file so
//! they're still there tomorrow!

use crate::{
    levels, levels::LevelBook, spawn_piece, LevelStuff, MainCamera, Piece, ReadingFont, Screen,
};
use bevy::light::NotShadowCaster;
use bevy::prelude::*;


/// The cursor hops along the road in steps this big.
/// Whole-number steps mean every piece lands on a nice
/// round spot — 18, 19, 20 — and nothing gets crooked!
const GRID_STEP: f32 = 1.0;

/// What the editor remembers.
#[derive(Resource)]
pub struct Editor {
    /// Which level is on the workbench.
    pub level: usize,
    /// Where the cursor is (how far down the road).
    pub cursor: f32,
    /// True while we're playtesting from the editor.
    pub playtesting: bool,
    /// True when the picture needs redrawing.
    needs_redraw: bool,
    /// A little message like "SAVED!", and its countdown.
    message: String,
    message_timer: f32,
    /// Counts down between steps while an arrow key is
    /// HELD, so the cursor can zoom along the road.
    move_timer: f32,
}

/// The sticker for the editor's own things
/// (the cursor and the help words).
#[derive(Component)]
struct EditorStuff;

/// The glowing golden cursor column.
#[derive(Component)]
struct CursorMarker;

/// The words that tell you where you are.
#[derive(Component)]
struct StatusWords;

/// Snap every piece in a level onto the whole-number
/// grid, and throw away doubles that land on the same
/// spot. The editor runs this on any level it opens, so
/// the cursor and the pieces ALWAYS agree on the grid!
fn snap_level_to_grid(level: &mut crate::levels::LevelData) {
    for (_, z) in &mut level.pieces {
        *z = z.round();
    }
    // Sweep out doubles: keep a piece only if no EARLIER
    // piece already sits on its exact spot.
    let mut seen_spots: Vec<f32> = Vec::new();
    level.pieces.retain(|(_, z)| {
        if seen_spots.contains(z) {
            false // a piece is already here — drop this one
        } else {
            seen_spots.push(*z);
            true
        }
    });
}

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Editor {
            level: 1,
            cursor: 18.0,
            playtesting: false,
            needs_redraw: true,
            message: String::new(),
            message_timer: 0.0,
            move_timer: 0.0,
        })
        .add_systems(OnEnter(Screen::Editor), open_editor)
        .add_systems(OnExit(Screen::Editor), close_editor)
        .add_systems(
            Update,
            (editor_keys, redraw_pieces, move_cursor_and_camera, update_status)
                .chain()
                .run_if(in_state(Screen::Editor)),
        );
    }
}

// ======================================================
//  OPENING AND CLOSING THE WORKSHOP
// ======================================================

fn open_editor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    font: Res<ReadingFont>,
    mut editor: ResMut<Editor>,
    mut book: ResMut<LevelBook>,
) {
    editor.playtesting = false;
    editor.needs_redraw = true;

    // Lock the level we're opening onto the grid.
    snap_level_to_grid(book.get_mut(editor.level));

    // One glowing green for EVERYTHING the editor draws —
    // the grid lines and the cursor share this exact
    // material, so they always match.
    let green_glow = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 1.0, 0.3),
        emissive: LinearRgba::new(0.4, 3.0, 0.6, 1.0), // GLOW!
        ..default()
    });

    // ---- THE GRID OVERLAY ----
    // A line across the road at every cell edge, so you
    // can SEE the grid you're snapping to.
    let line_shape = meshes.add(Cuboid::new(8.0, 0.02, 0.06));

    // The lines sit at the halfway points (5.5, 6.5, 7.5…)
    // so they mark the EDGES between cells. That way every
    // whole-number spot is the exact CENTER of its own
    // little cell, with a line on each side of it.
    for boundary in 6..=241 {
        commands.spawn((
            EditorStuff,
            Mesh3d(line_shape.clone()),
            MeshMaterial3d(green_glow.clone()),
            Transform::from_xyz(0.0, 0.02, -(boundary as f32 - 0.5)),
        ));
    }

    // The cursor: a green WIREFRAME box, like in 3D
    // modeling programs! It is EXACTLY one grid cell wide
    // (1.0), so its edges land right on the grid lines.
    // A box has 12 edges — 4 standing up, 4 across the
    // top, 4 across the bottom — and we draw each edge
    // as a very skinny glowing green stick.
    // The box is 1.0 wide (one cell!) and 5.0 tall, so its
    // edges sit half of that from the middle: 0.5 out,
    // 2.5 up and down.
    let w = 0.5; // halfway across — right on the grid line!
    let h = 2.5; // halfway up
    let t = 0.04; // how skinny each stick is

    commands
        .spawn((
            EditorStuff,
            CursorMarker,
            Transform::from_xyz(0.0, 2.5, -18.0),
            Visibility::default(),
        ))
        .with_children(|frame| {
            // The 4 standing-up edges, one per corner.
            // (-w,-w) (-w,+w) (+w,-w) (+w,+w) — every combo!
            for (x, z) in [(-w, -w), (-w, w), (w, -w), (w, w)] {
                frame.spawn((
                    NotShadowCaster, // wireframes don't need shadows!
                    Mesh3d(meshes.add(Cuboid::new(t, h * 2.0, t))),
                    MeshMaterial3d(green_glow.clone()),
                    Transform::from_xyz(x, 0.0, z),
                ));
            }
            // The 8 lying-down edges: 4 on top, 4 on the
            // bottom, going both directions.
            for y in [-h, h] {
                for z in [-w, w] {
                    frame.spawn((
                        NotShadowCaster,
                        Mesh3d(meshes.add(Cuboid::new(w * 2.0, t, t))),
                        MeshMaterial3d(green_glow.clone()),
                        Transform::from_xyz(0.0, y, z),
                    ));
                }
                for x in [-w, w] {
                    frame.spawn((
                        NotShadowCaster,
                        Mesh3d(meshes.add(Cuboid::new(t, t, w * 2.0))),
                        MeshMaterial3d(green_glow.clone()),
                        Transform::from_xyz(x, y, 0.0),
                    ));
                }
            }
        });

    // The status words up top.
    commands.spawn((
        EditorStuff,
        StatusWords,
        Text::new(""),
        TextFont {
            font: font.0.clone(),
            font_size: 34.0,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(Justify::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(16.0),
            width: Val::Percent(100.0),
            ..default()
        },
    ));

    // The help words along the bottom. Each command is a
    // pair: the KEY in yellow, then what it does in white,
    // so your eye can find the buttons fast! (A text can
    // hold many differently-colored pieces — each piece
    // is called a "span".)
    let commands_list = [
        ("1", " spike    "),
        ("2", " cube    "),
        ("3", " tall    "),
        ("4", " triple    "),
        ("5", " cube+spike    "),
        ("6", " sky spike    "),
        ("7", " bad guy\n"),
        ("arrows", " move      "),
        ("L", " change level      "),
        ("X", " delete      "),
        ("P", " playtest      "),
        ("S", " save      "),
        ("Esc", " menu"),
    ];
    commands
        .spawn((
            EditorStuff,
            Text::new(""),
            TextFont {
                font: font.0.clone(),
                font_size: 26.0,
                ..default()
            },
            TextLayout::new_with_justify(Justify::Center),
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(16.0),
                width: Val::Percent(100.0),
                ..default()
            },
        ))
        .with_children(|line| {
            for (key, what_it_does) in commands_list {
                // The key, in bright yellow...
                line.spawn((
                    TextSpan::new(key),
                    TextColor(crate::YELLOW),
                    TextFont {
                        font: font.0.clone(),
                        font_size: 26.0,
                        ..default()
                    },
                ));
                // ...then what it does, in white.
                line.spawn((
                    TextSpan::new(what_it_does),
                    TextColor(Color::WHITE),
                    TextFont {
                        font: font.0.clone(),
                        font_size: 26.0,
                        ..default()
                    },
                ));
            }
        });
}

fn close_editor(
    mut commands: Commands,
    stuff: Query<Entity, Or<(With<EditorStuff>, With<LevelStuff>)>>,
) {
    for thing in &stuff {
        commands.entity(thing).despawn();
    }
}

// ======================================================
//  THE KEYS — the whole editor is your keyboard!
// ======================================================

fn editor_keys(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut editor: ResMut<Editor>,
    mut book: ResMut<LevelBook>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    // ---- Gliding along the road (the arrow keys) ----
    // Right or Up moves farther down the road,
    // Left or Down brings it back — whichever feels right!
    // A quick tap moves ONE cell. HOLD the key and after
    // a short pause the cursor zooms along, one cell
    // every 0.06 seconds!
    let going_far =
        keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::ArrowUp);
    let going_near =
        keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::ArrowDown);

    if going_far || going_near {
        let step = if going_far { GRID_STEP } else { -GRID_STEP };
        let just_tapped = keyboard.any_just_pressed([
            KeyCode::ArrowRight,
            KeyCode::ArrowUp,
            KeyCode::ArrowLeft,
            KeyCode::ArrowDown,
        ]);

        if just_tapped {
            // The first press: move once, then wait a
            // moment before zooming starts.
            editor.cursor += step;
            editor.move_timer = 0.3;
        } else {
            // Still holding: count down, step, repeat!
            editor.move_timer -= time.delta_secs();
            if editor.move_timer <= 0.0 {
                editor.cursor += step;
                editor.move_timer = 0.06;
            }
        }
    }

    // Keep the cursor on the road (between 6 and 240)...
    editor.cursor = editor.cursor.clamp(6.0, 240.0);
    // ...and snap it to a whole number, so every piece
    // lands exactly on the grid: 18, 19, 20 — never 19.2!
    editor.cursor = editor.cursor.round();

    // ---- Picking which level to edit (the L key) ----
    // Pressing L hops to the next stage — but SKIPS the
    // boss stages, because bosses bring their own trouble
    // and have no pieces to edit! "%" wraps back around
    // to stage 1 after the last one.
    if keyboard.just_pressed(KeyCode::KeyL) {
        let total = book.levels.len();
        let mut next = editor.level;
        loop {
            next = next % total + 1; // 1, 2, 3 ... then back to 1
            if book.get(next).boss.is_empty() {
                break; // found a real level — stop here!
            }
        }
        editor.level = next;
        // Lock the newly opened level onto the grid too.
        snap_level_to_grid(book.get_mut(next));
        editor.needs_redraw = true;
    }

    // ---- Placing pieces (the number keys) ----
    let choices = [
        (KeyCode::Digit1, Piece::Spike),
        (KeyCode::Digit2, Piece::Cube),
        (KeyCode::Digit3, Piece::TallCube),
        (KeyCode::Digit4, Piece::TripleCube),
        (KeyCode::Digit5, Piece::CubeWithSpike),
        (KeyCode::Digit6, Piece::SkySpike),
        (KeyCode::Digit7, Piece::BadGuy),
    ];
    for (key, piece) in choices {
        if keyboard.just_pressed(key) {
            let level = book.get_mut(editor.level);
            let here = -editor.cursor; // down the road = minus z

            // First clear out anything already in this spot
            // ("retain" keeps only pieces that are far away).
            level.pieces.retain(|(_, z)| (z - here).abs() > 0.6);

            // Then put the new piece down!
            level.pieces.push((piece, here));

            // Keep the list sorted: closest pieces first.
            level.pieces.sort_by(|a, b| b.1.total_cmp(&a.1));

            // If we built PAST the finish line, scoot the
            // golden gate 10 farther back so the level
            // always ends after the last piece!
            if here - 10.0 < level.finish {
                level.finish = here - 10.0;
            }

            editor.needs_redraw = true;
        }
    }

    // ---- Deleting (X or Backspace) ----
    if keyboard.just_pressed(KeyCode::KeyX) || keyboard.just_pressed(KeyCode::Backspace) {
        let level = book.get_mut(editor.level);
        let here = -editor.cursor;
        level.pieces.retain(|(_, z)| (z - here).abs() > 0.9);
        editor.needs_redraw = true;
    }

    // ---- Saving (S) ----
    if keyboard.just_pressed(KeyCode::KeyS) {
        levels::save_level_book(&book);
        editor.message = "SAVED!".to_string();
        editor.message_timer = 2.5;
    }

    // ---- Playtesting (P) ----
    if keyboard.just_pressed(KeyCode::KeyP) {
        editor.playtesting = true;
        next_screen.set(Screen::Playing);
    }

    // ---- Leaving (Esc) ----
    if keyboard.just_pressed(KeyCode::Escape) {
        next_screen.set(Screen::Title);
    }
}

// ======================================================
//  REDRAWING — when the level changes, sweep away the
//  old picture and build it again from the book.
// ======================================================

fn redraw_pieces(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut editor: ResMut<Editor>,
    book: Res<LevelBook>,
    old_pieces: Query<Entity, With<LevelStuff>>,
) {
    if !editor.needs_redraw {
        return;
    }
    editor.needs_redraw = false;

    for old in &old_pieces {
        commands.entity(old).despawn();
    }

    for (piece, z) in book.get(editor.level).pieces.clone() {
        spawn_piece(&mut commands, &mut meshes, &mut materials, piece, z);
    }
}

// ======================================================
//  THE CAMERA FOLLOWS THE CURSOR down the road, and the
//  golden column stands wherever the cursor is.
// ======================================================

fn move_cursor_and_camera(
    editor: Res<Editor>,
    mut cursors: Query<&mut Transform, (With<CursorMarker>, Without<MainCamera>)>,
    mut cameras: Query<&mut Transform, (With<MainCamera>, Without<CursorMarker>)>,
) {
    let here = -editor.cursor;

    for mut cursor in &mut cursors {
        cursor.translation = Vec3::new(0.0, 2.5, here);
    }

    for mut camera in &mut cameras {
        *camera = Transform::from_xyz(7.0, 4.5, here + 9.0)
            .looking_at(Vec3::new(0.0, 1.5, here), Vec3::Y);
    }
}

// ======================================================
//  THE STATUS WORDS — where you are, what you're editing.
// ======================================================

fn update_status(
    time: Res<Time>,
    mut editor: ResMut<Editor>,
    book: Res<LevelBook>,
    mut words: Query<&mut Text, With<StatusWords>>,
) {
    // Count down the little "SAVED!" message.
    if editor.message_timer > 0.0 {
        editor.message_timer -= time.delta_secs();
        if editor.message_timer <= 0.0 {
            editor.message.clear();
        }
    }

    let level = book.get(editor.level);
    for mut text in &mut words {
        *text = Text::new(format!(
            "Editing  {}     spot {:.0}     {} pieces     {}",
            book.label(editor.level),
            editor.cursor,
            level.pieces.len(),
            editor.message,
        ));
    }
}
