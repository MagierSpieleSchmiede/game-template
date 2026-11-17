//! The presentation of the example game (template default, ADR-0013): a
//! camera above the arena, one capsule per unit, and the intent written back
//! from the move axis. Replace it together with `game_sim::game`.
//!
//! The simulation ticks at a fixed rate; the screen draws at its own. Each
//! frame shows the state *between* the last two ticks
//! ([`interpolate`]), so motion is smooth at any frame rate.

use std::collections::BTreeMap;

use bevy::prelude::*;

use game_sim::checksum::state_hash;
use game_sim::control::PlayerId;
use game_sim::game::Action;
use game_sim::ids::SimId;
use game_sim::intent::{Intent, IntentBuffer};
use game_sim::observation::{Observation, TeamId, Viewer, observe};

use crate::input::ActionState;

/// The player at this client, and the unit it controls (the example game gives
/// unit 1 to player 0). A template default: a real game gets both from the
/// lobby or the server.
pub const LOCAL_PLAYER: PlayerId = PlayerId(0);
pub const LOCAL_ACTOR: SimId = SimId(1);

/// The team this client sees for.
pub const LOCAL_VIEWER: Viewer = Viewer::Team(TeamId(0));

/// The observations after the last two ticks, and the state hash after the
/// last one (for the debug overlay).
#[derive(Debug, Default, Resource)]
pub struct Frames {
    pub previous: Option<Observation>,
    pub current: Option<Observation>,
    pub hash: u64,
}

/// The visual of one simulation unit; keyed by its ID, never its `Entity`.
#[derive(Debug, Component)]
struct UnitVisual(SimId);

#[derive(Debug, Resource)]
struct Assets3d {
    unit: Handle<Mesh>,
    local: Handle<StandardMaterial>,
    other: Handle<StandardMaterial>,
}

/// Adds the presentation.
pub fn plugin(app: &mut App) {
    app.init_resource::<Frames>()
        .add_systems(Startup, setup_scene)
        .add_systems(FixedPreUpdate, write_intent)
        .add_systems(FixedPostUpdate, observe_tick)
        .add_systems(Update, (spawn_floor, sync_visuals));
}

/// The position to draw: `alpha` of the way from the previous tick's to the
/// current tick's. A unit that just appeared is drawn where it is.
pub fn interpolate(previous: Option<Vec3>, current: Vec3, alpha: f32) -> Vec3 {
    previous.map_or(current, |p| p.lerp(current, alpha))
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 40.0, 30.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        DirectionalLight {
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(8.0, 20.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.insert_resource(Assets3d {
        unit: meshes.add(Capsule3d::new(0.5, 1.0)),
        local: materials.add(Color::srgb(0.95, 0.65, 0.2)),
        other: materials.add(Color::srgb(0.35, 0.55, 0.9)),
    });
}

/// Turns the move axis into this tick's intent. Screen up is away from the
/// camera, which is −z on the arena floor.
fn write_intent(actions: Res<ActionState>, mut buffer: ResMut<IntentBuffer>) {
    let axis = actions.move_axis();
    buffer.push(Intent {
        player: LOCAL_PLAYER,
        actor: LOCAL_ACTOR,
        action: Action::Move {
            x: axis.x,
            z: -axis.y,
        },
    });
}

/// Reads the simulation after each tick, through the observation only.
fn observe_tick(world: &mut World) {
    let observation = observe(world, LOCAL_VIEWER);
    let hash = state_hash(world);
    let mut frames = world.resource_mut::<Frames>();
    frames.previous = frames.current.take();
    frames.current = Some(observation);
    frames.hash = hash;
}

/// Spawns the arena floor once the first observation says how large it is.
fn spawn_floor(
    mut commands: Commands,
    frames: Res<Frames>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut done: Local<bool>,
) {
    let Some(current) = &frames.current else {
        return;
    };
    if !*done {
        commands.spawn((
            Mesh3d(meshes.add(Circle::new(current.game.arena_radius))),
            MeshMaterial3d(materials.add(Color::srgb(0.25, 0.27, 0.3))),
            Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ));
        *done = true;
    }
}

/// Moves each unit's visual to its interpolated position, spawning visuals for
/// new units and despawning those of vanished ones.
fn sync_visuals(
    mut commands: Commands,
    frames: Res<Frames>,
    fixed: Res<Time<Fixed>>,
    assets: Option<Res<Assets3d>>,
    mut visuals: Query<(Entity, &UnitVisual, &mut Transform)>,
) {
    let (Some(current), Some(assets)) = (&frames.current, assets) else {
        return;
    };
    let previous: BTreeMap<SimId, Vec3> = frames
        .previous
        .iter()
        .flat_map(|o| &o.game.units)
        .map(|u| (u.id, u.position))
        .collect();
    let mut targets: BTreeMap<SimId, Vec3> = current
        .game
        .units
        .iter()
        .map(|u| {
            let at = interpolate(
                previous.get(&u.id).copied(),
                u.position,
                fixed.overstep_fraction(),
            );
            (u.id, at + Vec3::Y)
        })
        .collect();

    for (entity, visual, mut transform) in &mut visuals {
        match targets.remove(&visual.0) {
            Some(at) => transform.translation = at,
            None => commands.entity(entity).despawn(),
        }
    }
    for (id, at) in targets {
        let material = if id == LOCAL_ACTOR {
            assets.local.clone()
        } else {
            assets.other.clone()
        };
        commands.spawn((
            UnitVisual(id),
            Mesh3d(assets.unit.clone()),
            MeshMaterial3d(material),
            Transform::from_translation(at),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interpolates_between_ticks() {
        let (a, b) = (Vec3::ZERO, Vec3::new(2.0, 0.0, -4.0));
        assert_eq!(interpolate(Some(a), b, 0.0), a);
        assert_eq!(interpolate(Some(a), b, 0.5), Vec3::new(1.0, 0.0, -2.0));
        assert_eq!(interpolate(None, b, 0.3), b);
    }
}
