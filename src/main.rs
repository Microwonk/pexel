// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::window::{close_on_esc, CursorGrabMode, PrimaryWindow, WindowMode};
use bevy::winit::WinitWindows;
use bevy::DefaultPlugins;
use pexel::GamePlugin; // ToDo: Replace bevy_game with your new crate name.
use std::io::Cursor;
use winit::window::Icon;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.7)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Pexel".to_string(), // ToDo
                // Bind to canvas included in `index.html`
                canvas: Some("#bevy".to_owned()),
                // The canvas size is constrained in index.html and build/web/styles.css
                fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5 and Ctrl+R
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(GamePlugin)
        .add_systems(Startup, set_window_icon)
        .add_systems(Update, close_on_esc)
        .add_systems(Update, hotkeys_system)
        .run();
}

fn hotkeys_system(keyboard_input: Res<Input<KeyCode>>, mut windows: Query<&mut Window>) {
    if keyboard_input.just_pressed(KeyCode::F11) {
        let mut window = windows.single_mut();
        window.mode = match window.mode {
            WindowMode::BorderlessFullscreen => WindowMode::Windowed,
            WindowMode::Windowed => WindowMode::BorderlessFullscreen,
            _ => WindowMode::Windowed,
        };
    }

    if keyboard_input.just_pressed(KeyCode::AltLeft) {
        let mut window = windows.single_mut();
        window.cursor.grab_mode = match window.cursor.grab_mode {
            CursorGrabMode::None => CursorGrabMode::Confined,
            _ => CursorGrabMode::None,
        };
        window.cursor.visible = !window.cursor.visible;
    }
}

fn set_window_icon(
    windows: NonSend<WinitWindows>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    let primary_entity = primary_window.single();
    let Some(primary) = windows.get_window(primary_entity) else {
        return;
    };
    let icon_buf = Cursor::new(include_bytes!("../assets/icon.png"));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}
