//! ======================================================
//!  THE TITLE SCREEN — the front door of our game!
//! ======================================================
//! The bunny stands on the left. The Cursed Thorn stands
//! on the right, shooting a thorn at the bunny! The lava
//! lamp swirls behind, and shapes decorate the bottom.
//!
//! Press 1 to PLAY, 2 for SETTINGS, 3 for the EDITOR.

use crate::{
    editor::Editor, levels::LevelBook, spawn_bat_visual, spawn_bunny_visual,
    spawn_thorn_plant, spawn_tomato_visual, GameFont, MainCamera, ReadingFont, Screen,
    Settings,
};
use bevy::prelude::*;

/// The sticker for everything on the title screen.
#[derive(Component)]
struct TitleStuff;

/// The sticker for everything on the settings screen.
#[derive(Component)]
struct SettingsStuff;

/// The settings words that change when you press keys.
#[derive(Component)]
struct SettingsWords;

pub struct TitlePlugin;

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::Title), open_title)
            .add_systems(OnExit(Screen::Title), close_title)
            .add_systems(Update, title_keys.run_if(in_state(Screen::Title)))
            .add_systems(OnEnter(Screen::Settings), open_settings)
            .add_systems(OnExit(Screen::Settings), close_settings)
            .add_systems(Update, settings_keys.run_if(in_state(Screen::Settings)));
    }
}

// ======================================================
//  BUILD THE TITLE SCREEN
// ======================================================

fn open_title(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    font: Res<GameFont>,
    mut cameras: Query<&mut Transform, With<MainCamera>>,
) {
    // The title screen IS a little level! Same camera as
    // the game, the bunny on the road, pieces up ahead —
    // and all three bosses waiting at the end of it.
    for mut camera in &mut cameras {
        *camera =
            Transform::from_xyz(6.0, 5.0, 9.0).looking_at(Vec3::new(0.0, 1.0, -3.0), Vec3::Y);
    }

    // ----- The bunny, right where it stands in the game -----
    let bunny = spawn_bunny_visual(
        &mut commands,
        &mut meshes,
        &mut materials,
        Transform::from_xyz(0.0, 0.5, 0.0),
    );
    commands.entity(bunny).insert(TitleStuff);

    // ----- A little stretch of level up the road -----
    // (These are REAL level pieces, built by the same
    // spawn_piece the game and the editor use!)
    for (piece, spot) in [
        (crate::Piece::Spike, -5.0),
        (crate::Piece::Cube, -8.0),
        (crate::Piece::TallCube, -9.0),
        (crate::Piece::SkySpike, -12.0),
        (crate::Piece::BadGuy, -15.0),
    ] {
        crate::spawn_piece(&mut commands, &mut meshes, &mut materials, piece, spot);
    }

    // ----- ALL THREE BOSSES at the end of the road! -----
    let tomato = spawn_tomato_visual(
        &mut commands,
        &mut meshes,
        &mut materials,
        Transform::from_xyz(-2.6, 2.0, -20.0).with_scale(Vec3::splat(0.8)),
    );
    commands.entity(tomato).insert(TitleStuff);

    let plant = spawn_thorn_plant(
        &mut commands,
        &mut meshes,
        &mut materials,
        Transform::from_xyz(2.6, 0.0, -21.0),
    );
    commands.entity(plant).insert(TitleStuff);

    let bat = spawn_bat_visual(
        &mut commands,
        &mut meshes,
        &mut materials,
        Transform::from_xyz(0.0, 5.0, -24.0),
    );
    commands.entity(bat).insert(TitleStuff);

    // ----- The title words! -----
    commands.spawn((
        TitleStuff,
        Text::new("3D BUNNY DASH"),
        TextFont {
            font: font.0.clone(),
            font_size: 100.0,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(Justify::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(6.0),
            width: Val::Percent(100.0),
            ..default()
        },
    ));
    commands.spawn((
        TitleStuff,
        Text::new("[1] PLAY        [2] SETTINGS        [3] LEVEL EDITOR"),
        TextFont {
            font: font.0.clone(),
            font_size: 38.0,
            ..default()
        },
        TextColor(crate::YELLOW),
        TextLayout::new_with_justify(Justify::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(86.0),
            width: Val::Percent(100.0),
            ..default()
        },
    ));
}

fn close_title(
    mut commands: Commands,
    stuff: Query<Entity, Or<(With<TitleStuff>, With<crate::LevelStuff>)>>,
) {
    for thing in &stuff {
        commands.entity(thing).despawn();
    }
}

/// Which menu button did we press?
fn title_keys(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut the_editor: ResMut<Editor>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    // 1 (or Space or Enter) → PLAY!
    if keyboard.just_pressed(KeyCode::Digit1)
        || keyboard.just_pressed(KeyCode::Space)
        || keyboard.just_pressed(KeyCode::Enter)
    {
        // Playing from the menu, not from the editor.
        the_editor.playtesting = false;
        next_screen.set(Screen::Playing);
    }
    // 2 → SETTINGS
    if keyboard.just_pressed(KeyCode::Digit2) {
        next_screen.set(Screen::Settings);
    }
    // 3 → LEVEL EDITOR
    if keyboard.just_pressed(KeyCode::Digit3) {
        next_screen.set(Screen::Editor);
    }
}

// ======================================================
//  THE SETTINGS SCREEN
// ======================================================

fn open_settings(mut commands: Commands, font: Res<GameFont>, reading: Res<ReadingFont>) {
    commands.spawn((
        SettingsStuff,
        Text::new("SETTINGS"),
        TextFont {
            font: font.0.clone(),
            font_size: 84.0,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(Justify::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(10.0),
            width: Val::Percent(100.0),
            ..default()
        },
    ));
    commands.spawn((
        SettingsStuff,
        SettingsWords,
        Text::new(""),
        TextFont {
            // The easy-reading font for the smaller words.
            font: reading.0.clone(),
            font_size: 40.0,
            ..default()
        },
        TextColor(crate::YELLOW),
        TextLayout::new_with_justify(Justify::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(35.0),
            width: Val::Percent(100.0),
            ..default()
        },
    ));
}

fn close_settings(mut commands: Commands, stuff: Query<Entity, With<SettingsStuff>>) {
    for thing in &stuff {
        commands.entity(thing).despawn();
    }
}

fn settings_keys(
    keyboard: Res<ButtonInput<KeyCode>>,
    book: Res<LevelBook>,
    mut settings: ResMut<Settings>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut words: Query<&mut Text, With<SettingsWords>>,
) {
    // Left and Right arrows pick the starting level.
    if keyboard.just_pressed(KeyCode::ArrowLeft) && settings.starting_level > 1 {
        settings.starting_level -= 1;
    }
    if keyboard.just_pressed(KeyCode::ArrowRight)
        && settings.starting_level < book.levels.len()
    {
        settings.starting_level += 1;
    }

    // Escape or Enter → back to the title screen.
    if keyboard.just_pressed(KeyCode::Escape) || keyboard.just_pressed(KeyCode::Enter) {
        next_screen.set(Screen::Title);
    }

    // Keep the words up to date. The level book hands us
    // the stage's one true name tag, so the counting here
    // matches the rest of the game!
    for mut text in &mut words {
        *text = Text::new(format!(
            "Start at:  <  {}  >\n\n\
             press Left / Right to change\n\n\
             Esc: back to the title screen",
            book.label(settings.starting_level),
        ));
    }
}
