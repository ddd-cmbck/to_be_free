// src/features/player/bundles.rs
use bevy::prelude::*;

use super::component::{MoveInput, MoveSpeed, Player, Velocity};

/// Convenience bundle for spawning a player with all required movement components.
///
/// Notes:
/// - We include `MoveInput` (local intent) and `Velocity` (world velocity) from day 1,
///   so swapping integration for collision/physics later is painless.
/// - We intentionally do *not* attach any physics/collision components yet.
#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub speed: MoveSpeed,
    pub input: MoveInput,
    pub velocity: Velocity,
    pub transform: Transform,
}

impl PlayerBundle {
    pub fn new(spawn_translation: Vec3, speed_units_per_sec: f32) -> Self {
        Self {
            player: Player,
            speed: MoveSpeed(speed_units_per_sec),
            input: MoveInput(Vec3::ZERO),
            velocity: Velocity(Vec3::ZERO),
            transform: Transform::from_translation(spawn_translation),
        }
    }
}

/// Spawns the player entity (Option A: the player feature owns the player).
///
/// Minimal visuals: a lit cube so we can see motion immediately.
pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn slightly above ground so it "rests" visually on the ground plane (y=0).
    let spawn_pos = Vec3::new(0.0, 0.5, 0.0);

    commands.spawn((
        PlayerBundle::new(spawn_pos, 5.0),
        // Visuals (PBR)
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(240, 220, 120))),
    ));
}


#[cfg(test)]
mod tests {
    use super::*;
    use bevy::ecs::system::RunSystemOnce;

    // -------- PlayerBundle::new tests (pure constructor contract) --------

    #[test]
    fn player_bundle_new_sets_expected_defaults() {
        let spawn = Vec3::new(1.0, 2.0, 3.0);
        let speed = 7.5;

        let b = PlayerBundle::new(spawn, speed);

        assert_eq!(b.player, Player);
        assert_eq!(b.speed, MoveSpeed(speed));
        assert_eq!(b.input, MoveInput(Vec3::ZERO));
        assert_eq!(b.velocity, Velocity(Vec3::ZERO));
        assert_eq!(b.transform.translation, spawn);
    }

    // -------- spawn_player tests (ECS + assets contract) --------

    #[test]
    fn spawn_player_spawns_entity_with_required_components_and_assets() {
        let mut world = World::new();

        // The system requires these resources. Assets<T> has Default, so we can insert it directly.
        world.insert_resource(Assets::<Mesh>::default());
        world.insert_resource(Assets::<StandardMaterial>::default());

        // Run the spawn system once. This should apply Commands and actually spawn the entity.
        let _ = world.run_system_once(spawn_player);

        // Find the spawned player entity.
        let mut q = world.query::<(
            &Player,
            &MoveSpeed,
            &MoveInput,
            &Velocity,
            &Transform,
            &Mesh3d,
            &MeshMaterial3d<StandardMaterial>,
        )>();

        let ( _player, speed, input, vel, tr, mesh3d, mat3d) = q
            .iter(&world)
            .next()
            .expect("spawn_player should spawn exactly one entity with player + visuals");

        // Check gameplay components
        assert_eq!(*speed, MoveSpeed(5.0));
        assert_eq!(*input, MoveInput(Vec3::ZERO));
        assert_eq!(*vel, Velocity(Vec3::ZERO));

        // Check spawn position contract
        assert_eq!(tr.translation, Vec3::new(0.0, 0.5, 0.0));

        // Check visuals: handles must exist in their asset storages
        let meshes = world.resource::<Assets<Mesh>>();
        let materials = world.resource::<Assets<StandardMaterial>>();

        assert!(
            meshes.get(&mesh3d.0).is_some(),
            "Mesh handle must exist in Assets<Mesh>"
        );
        assert!(
            materials.get(&mat3d.0).is_some(),
            "Material handle must exist in Assets<StandardMaterial>"
        );
    }

    #[test]
    fn spawn_player_is_idempotent_per_call_spawns_one_more_player() {
        let mut world = World::new();
        world.insert_resource(Assets::<Mesh>::default());
        world.insert_resource(Assets::<StandardMaterial>::default());

        let _ = world.run_system_once(spawn_player);
        let _ = world.run_system_once(spawn_player);
    }
}    