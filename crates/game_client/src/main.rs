//! The game client: presentation, input, HUD, localisation.
//!
//! The client is a pure observer of the simulation. It reads through the
//! observation interface and writes back exactly one thing: the intent
//! (ADR-0002). Replace this skeleton with your game's presentation.

use bevy::prelude::*;

use game_sim::SimPlugin;
use game_sim::scenario::{Scenario, spawn_scenario};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SimPlugin)
        .insert_resource(Scenario::default())
        .add_systems(Startup, (spawn_scenario, setup_camera))
        .add_systems(Update, move_camera_orbit)
        .run();
}

/// A simple top-down camera so the arena is visible; replace with the game's
/// actual camera (third person, first person, …).
fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 30.0, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
    ));
}

/// Slowly orbits the camera until real input exists.
fn move_camera_orbit(time: Res<Time>, mut camera: Single<&mut Transform, With<Camera3d>>) {
    let angle = time.elapsed_secs() * 0.1;
    camera.translation = Vec3::new(angle.cos() * 30.0, 30.0, angle.sin() * 30.0);
    camera.look_at(Vec3::ZERO, Vec3::Y);
}
