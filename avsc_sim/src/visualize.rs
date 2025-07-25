use crate::model::{Module, GateType};
use bevy::prelude::*;

pub fn print_gate_level(module: &Module) {
    println!("\nGate-Level Schematic:");
    for (i, gate) in module.gates.iter().enumerate() {
        let in_names: Vec<_> = gate.inputs.iter().map(|&sid| &module.signals[sid].name).collect();
        let out_name = &module.signals[gate.output].name;
        println!("[{}] {:?}: {} -> {}", i, gate.gate_type, in_names.join(", "), out_name);
    }
    println!("\nSignal Connections:");
    for (sid, sig) in module.signals.iter().enumerate() {
        let mut conn = format!("{}", sig.name);
        if !sig.drivers.is_empty() {
            conn = format!("{} <-- {}", conn, sig.drivers.iter().map(|&gid| format!"[{}]", gid).collect::<Vec<_>>().join(", "));
        }
        if !sig.loads.is_empty() {
            conn = format!("{} --> {}", conn, sig.loads.iter().map(|&gid| format!"[{}]", gid).collect::<Vec<_>>().join(", "));
        }
        println!("{}", conn);
    }
}

pub fn launch_visualization() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "AVSC Visualization".to_string(),
                resolution: (1280., 720.).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ViewMode::TwoD)
        .add_startup_system(setup_ui)
        .add_startup_system(setup_2d)
        .add_system(toggle_view)
        .add_system(update_visibility)
        .run();
}

#[derive(Resource, PartialEq, Eq, Clone, Copy)]
pub enum ViewMode {
    TwoD,
    ThreeD,
}

fn setup_ui(mut commands: Commands) {
    // Placeholder: could add UI for toggling view, etc.
}

fn setup_2d(mut commands: Commands) {
    // 2D Camera
    commands.spawn(Camera2dBundle::default());
    // Draw demo gates as rectangles/circles
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.2, 0.7, 0.2),
            custom_size: Some(Vec2::new(100.0, 60.0)),
            ..default()
        },
        transform: Transform::from_xyz(-200.0, 0.0, 0.0),
        ..default()
    });
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.7, 0.2, 0.2),
            custom_size: Some(Vec2::new(100.0, 60.0)),
            ..default()
        },
        transform: Transform::from_xyz(200.0, 0.0, 0.0),
        ..default()
    });
    // Add more demo gates/wires as needed
}

fn setup_3d(mut commands: Commands) {
    // 3D Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 600.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    // Draw demo cubes (representing cells/layers)
    commands.spawn(PbrBundle {
        mesh: bevy::prelude::Mesh::from(shape::Cube { size: 100.0 }),
        material: StandardMaterial {
            base_color: Color::rgb(0.2, 0.2, 0.7),
            ..default()
        },
        transform: Transform::from_xyz(-150.0, 0.0, 0.0),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: bevy::prelude::Mesh::from(shape::Cube { size: 100.0 }),
        material: StandardMaterial {
            base_color: Color::rgb(0.7, 0.7, 0.2),
            ..default()
        },
        transform: Transform::from_xyz(150.0, 0.0, 0.0),
        ..default()
    });
    // Add a light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 300.0, 300.0),
        ..default()
    });
}

fn toggle_view(
    keys: Res<Input<KeyCode>>,
    mut view_mode: ResMut<ViewMode>,
    mut commands: Commands,
    query_2d: Query<Entity, With<Camera2d>>,
    query_3d: Query<Entity, With<Camera3d>>,
) {
    if keys.just_pressed(KeyCode::F2) {
        if *view_mode == ViewMode::TwoD {
            // Switch to 3D
            for entity in query_2d.iter() {
                commands.entity(entity).despawn_recursive();
            }
            setup_3d(commands);
            *view_mode = ViewMode::ThreeD;
        } else {
            // Switch to 2D
            for entity in query_3d.iter() {
                commands.entity(entity).despawn_recursive();
            }
            setup_2d(commands);
            *view_mode = ViewMode::TwoD;
        }
    }
}

fn update_visibility(
    view_mode: Res<ViewMode>,
    mut query_2d: Query<&mut Visibility, With<Camera2d>>,
    mut query_3d: Query<&mut Visibility, With<Camera3d>>,
) {
    let show_2d = *view_mode == ViewMode::TwoD;
    for mut vis in query_2d.iter_mut() {
        *vis = if show_2d { Visibility::Visible } else { Visibility::Hidden };
    }
    for mut vis in query_3d.iter_mut() {
        *vis = if show_2d { Visibility::Hidden } else { Visibility::Visible };
    }
}