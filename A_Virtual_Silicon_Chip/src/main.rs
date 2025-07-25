use bevy::prelude::*;

mod layers;
mod simulation;
mod ui;

fn main() {
    App::new()
        // Set up the window
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "A Virtual Silicon Chip (AVSC)".to_string(),
                resolution: (1280., 720.).into(),
                ..default()
            }),
            ..default()
        }))
        // Set a background color
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.15)))
        // Add a startup system to spawn the camera
        .add_startup_system(setup_camera)
        .run();
}

/// Spawns a 2D camera (for now; can be upgraded to 3D later)
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}