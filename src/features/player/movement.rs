use bevy::prelude::*;

use super::component::{MoveInput, MoveSpeed, Player, Velocity};

/// Convert local-space movement intent into world-space velocity.
///
/// Pipeline contract:
/// - Reads: MoveInput (local), MoveSpeed, Transform.rotation
/// - Writes: Velocity (world units/sec)
pub fn compute_velocity_from_input(
    mut q_player: Query<(&MoveInput, &MoveSpeed, &Transform, &mut Velocity), With<Player>>,
) {
    for (move_input, speed, transform, mut velocity) in &mut q_player {
        // Local intent is already normalized (input system guarantees this).
        // Rotate local intent into world space using the player's current orientation.
        let world_dir = transform.rotation * move_input.0;

        // Velocity is in world units per second.
        velocity.0 = world_dir * speed.0;
    }
}

/// Integrate velocity into translation using the fixed timestep.
///
/// Temporary integration step:
/// - Reads: Velocity
/// - Writes: Transform.translation
/// Later, swap this out for physics engine integration.
pub fn integrate_velocity(
    time: Res<Time<Fixed>>,
    mut q_player: Query<(&Velocity, &mut Transform), With<Player>>,
) {
    let dt = time.delta_secs();

    for (velocity, mut transform) in &mut q_player {
        transform.translation += velocity.0 * dt;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use bevy::ecs::system::RunSystemOnce;
    use std::time::Duration;

    #[test]
    fn compute_velocity_from_input_rotates_and_scales() {
        let mut world = World::new();

        // Local intent along +X, rotate +90° around Y -> world direction should become -Z.
        world.spawn((
            Player,
            MoveInput(Vec3::X),
            MoveSpeed(10.0),
            Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)),
            Velocity(Vec3::ZERO),
        ));

        let _ = world.run_system_once(compute_velocity_from_input);

        let v = world.query::<&Velocity>().single(&world).unwrap().0;
        let expected = Vec3::NEG_Z * 10.0;

        assert!(
            (v - expected).length() < 1e-5,
            "Expected velocity {expected:?}, got {v:?}"
        );
    }

    #[test]
    fn integrate_velocity_moves_translation_by_fixed_dt() {
        let mut world = World::new();

        // Create a fixed clock and make its delta deterministic.
        let mut fixed_time = Time::<Fixed>::from_hz(60.0);
        fixed_time.advance_by(Duration::from_secs_f32(1.0 / 60.0));
        world.insert_resource(fixed_time);

        world.spawn((
            Player,
            Velocity(Vec3::new(6.0, 0.0, 0.0)), // 6 units/sec
            Transform::from_translation(Vec3::ZERO),
        ));

        let _ = world.run_system_once(integrate_velocity);

        let pos = world.query::<&Transform>().single(&world).unwrap().translation;
        let expected = Vec3::new(0.1, 0.0, 0.0); // 6 * (1/60) = 0.1

        assert!(
            (pos - expected).length() < 1e-6,
            "Expected translation {expected:?}, got {pos:?}"
        );
    }

    #[test]
    fn integrate_velocity_does_nothing_when_dt_is_zero() {
        let mut world = World::new();

        // Default Time<Fixed> has 0 delta until advanced.
        world.insert_resource(Time::<Fixed>::from_hz(60.0));

        world.spawn((
            Player,
            Velocity(Vec3::new(100.0, 0.0, 0.0)),
            Transform::from_translation(Vec3::new(1.0, 2.0, 3.0)),
        ));

        let _ = world.run_system_once(integrate_velocity);

        let pos = world.query::<&Transform>().single(&world).unwrap().translation;
        assert!(
            (pos - Vec3::new(1.0, 2.0, 3.0)).length() < 1e-6,
            "With dt=0, translation should not change"
        );
    }
}
