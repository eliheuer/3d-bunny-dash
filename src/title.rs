//! ======================================================
//!  THE TITLE SCREEN — the front door of our game!
//! ======================================================
//! The bunny stands on the left. The Cursed Thorn stands
//! on the right, shooting a thorn at the bunny! The lava
//! lamp swirls behind, and shapes decorate the bottom.
//!
//! Press 1 to PLAY, 2 for SETTINGS, 3 for the EDITOR.

use crate::{
    editor::Editor, levels::LevelBook, spawn_bunny_visual, spawn_thorn_plant, GameFont,
    MainCamera, ReadingFont, Screen, Settings,
};
use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

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
    // Point the camera straight at the stage.
    for mut camera in &mut cameras {
        *camera =
            Transform::from_xyz(0.0, 2.6, 12.0).looking_at(Vec3::new(0.0, 2.0, 0.0), Vec3::Y);
    }

    // ----- The bunny, on the left -----
    // Turned PARTWAY toward the villain (a full quarter
    // turn is 1.57, so 0.9 is a bit more than half a
    // quarter turn) — that way we can still see its face!
    let bunny = spawn_bunny_visual(
        &mut commands,
        &mut meshes,
        &mut materials,
        Transform::from_xyz(-3.6, 1.0, 2.0)
            .with_scale(Vec3::splat(1.5))
            .with_rotation(Quat::from_rotation_y(-0.9)),
    );
    commands.entity(bunny).insert(TitleStuff);

    // ----- The Cursed Thorn, on the right -----
    // Its eyes and petals are on its front, so we leave it
    // unturned — facing the camera, glaring right at YOU.
    let plant = spawn_thorn_plant(
        &mut commands,
        &mut meshes,
        &mut materials,
        Transform::from_xyz(3.8, 0.0, 1.0),
    );
    commands.entity(plant).insert(TitleStuff);

    // ----- A thorn frozen mid-flight between them! -----
    commands.spawn((
        TitleStuff,
        Mesh3d(meshes.add(Cone::new(0.22, 0.8))),
        MeshMaterial3d(materials.add(Color::srgb(0.35, 0.4, 0.25))),
        // Pointing left, at the bunny (a quarter turn).
        Transform::from_xyz(0.4, 2.9, 1.5).with_rotation(Quat::from_rotation_z(FRAC_PI_2)),
    ));

    // ----- Shapes decorating the bottom of the stage -----
    let orange = materials.add(Color::srgb(1.0, 0.5, 0.1));
    let purple = materials.add(Color::srgb(0.6, 0.2, 0.9));
    let blue = materials.add(Color::srgb(0.2, 0.5, 1.0));
    let red = materials.add(Color::srgb(0.9, 0.1, 0.1));

    commands.spawn((
        TitleStuff,
        Mesh3d(meshes.add(Cone::new(0.6, 1.4))),
        MeshMaterial3d(orange.clone()),
        Transform::from_xyz(-5.5, 0.7, 4.0),
    ));
    commands.spawn((
        TitleStuff,
        Mesh3d(meshes.add(Cuboid::new(1.2, 1.2, 1.2))),
        MeshMaterial3d(purple.clone()),
        Transform::from_xyz(-1.8, 0.6, 4.5),
    ));
    commands.spawn((
        TitleStuff,
        Mesh3d(meshes.add(Sphere::new(0.55))),
        MeshMaterial3d(red.clone()),
        Transform::from_xyz(1.2, 0.55, 4.5),
    ));
    commands.spawn((
        TitleStuff,
        // An upside-down sky spike, floating decoratively.
        Mesh3d(meshes.add(Cone::new(0.6, 1.4))),
        MeshMaterial3d(blue.clone()),
        Transform::from_xyz(5.6, 4.5, 2.0)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::PI)),
    ));

    // ----- The title words! -----
    commands.spawn((
        TitleStuff,
        Text::new("3D BUNNY GEOMETRY DASH"),
        TextFont {
            font: font.0.clone(),
            font_size: 64.0,
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
        Text::new("vs. THE CURSED THORN"),
        TextFont {
            font: font.0.clone(),
            font_size: 44.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.4, 0.7)),
        TextLayout::new_with_justify(Justify::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(17.0),
            width: Val::Percent(100.0),
            ..default()
        },
    ));
    commands.spawn((
        TitleStuff,
        Text::new("[1] PLAY        [2] SETTINGS        [3] LEVEL EDITOR"),
        TextFont {
            font: font.0.clone(),
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.9, 0.2)),
        TextLayout::new_with_justify(Justify::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(86.0),
            width: Val::Percent(100.0),
            ..default()
        },
    ));
}

fn close_title(mut commands: Commands, stuff: Query<Entity, With<TitleStuff>>) {
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
            font_size: 72.0,
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
            font_size: 32.0,
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

    // Keep the words up to date.
    for mut text in &mut words {
        *text = Text::new(format!(
            "Starting level:  <  {}: {}  >\n\n\
             press Left / Right to change\n\n\
             Esc: back to the title screen",
            settings.starting_level,
            book.get(settings.starting_level).name,
        ));
    }
}
