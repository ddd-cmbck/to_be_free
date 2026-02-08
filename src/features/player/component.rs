// src/features/player/component.rs
use bevy::prelude::*;

/// Tag component marking the user-controlled player entity.
#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Player;

/// Player movement speed in world units per second.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct MoveSpeed(pub f32);

/// Local-space movement intent (direction) produced by input.
///
/// Coordinate conventions (Bevy-style):
/// - +X: right
/// - +Y: up
/// - -Z: forward
///
/// This is an *intent*, not a velocity:
/// - It should be normalized (length ~ 1) when non-zero.
/// - A separate FixedUpdate system converts it into world-space velocity.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct MoveInput(pub Vec3);

/// World-space velocity (units per second).
///
/// For now we integrate this directly into `Transform.translation` in FixedUpdate.
/// Later, a physics/collision engine will own integration and write the transform.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Velocity(pub Vec3);



#[cfg(test)]
mod tests {
    use super::*;

    
    // --- Compile-time trait contracts ---
    // These tests won't run anything meaningful; they just ensure trait bounds hold.
    fn assert_copy<T: Copy>() {}
    fn assert_default<T: Default>() {}
    fn assert_eq_hash<T: Eq + std::hash::Hash>() {}
    fn assert_partial_eq<T: PartialEq>() {}
    fn assert_component<T: Component>() {}

    #[test]
    fn trait_contracts_hold() {
        assert_component::<Player>();
        assert_copy::<Player>();
        assert_default::<Player>();
        assert_eq_hash::<Player>();

        assert_component::<MoveSpeed>();
        assert_copy::<MoveSpeed>();
        assert_partial_eq::<MoveSpeed>();

        assert_component::<MoveInput>();
        assert_copy::<MoveInput>();
        assert_partial_eq::<MoveInput>();

        assert_component::<Velocity>();
        assert_copy::<Velocity>();
        assert_partial_eq::<Velocity>();
    }

    // --- Minimal ECS sanity ---
    #[test]
    fn can_spawn_and_query_player_components() {
        let mut world = World::new();

        let e = world.spawn((
            Player,
            MoveSpeed(3.5),
            MoveInput(Vec3::new(1.0, 0.0, 0.0)),
            Velocity(Vec3::new(0.0, 0.0, -2.0)),
        )).id();

        let (speed, input, vel) = world
            .query::<(&MoveSpeed, &MoveInput, &Velocity)>()
            .get(&world, e)
            .expect("entity should have speed/input/velocity");

        assert_eq!(*speed, MoveSpeed(3.5));
        assert_eq!(*input, MoveInput(Vec3::new(1.0, 0.0, 0.0)));
        assert_eq!(*vel, Velocity(Vec3::new(0.0, 0.0, -2.0)));
    }
}
