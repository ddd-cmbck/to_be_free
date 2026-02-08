use bevy::prelude::*;

use super::component::{MoveInput, Player};

/// Keybindings for player movement input.
///
/// Coordinate conventions (Bevy-style):
/// - +X: right
/// - +Y: up
/// - -Z: forward
#[derive(Resource, Debug, Clone)]
pub struct PlayerKeybindings {
    pub forward: KeyCode,
    pub back: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub up: KeyCode,
    pub down: KeyCode,
}

impl Default for PlayerKeybindings {
    fn default() -> Self {
        Self {
            forward: KeyCode::KeyW,
            back: KeyCode::KeyS,
            left: KeyCode::KeyA,
            right: KeyCode::KeyD,
            up: KeyCode::Space,
            down: KeyCode::ShiftLeft,
        }
    }
}

/// Update: read keyboard input and write local-space movement intent.
///
/// - Uses match-based dispatch (clean Rust, fewer branches)
/// - Produces normalized local intent
/// - Does NOT touch Transform (collision-ready)
pub fn read_player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    bindings: Option<Res<PlayerKeybindings>>,
    mut q_player_input: Query<&mut MoveInput, With<Player>>,
) {

    let Some(bindings) = bindings else {
        // If you see this spam, PlayerPlugin init_resource isn't running.
        // (You can replace with `warn!` if you want logging.)
        return;
    };

    let mut dir = Vec3::ZERO;

    // Iterate only over pressed keys and dispatch via match
    for key in keyboard.get_pressed() {
        match *key {
            k if k == bindings.right => dir.x += 1.0,
            k if k == bindings.left => dir.x -= 1.0,

            k if k == bindings.up => dir.y += 1.0,
            k if k == bindings.down => dir.y -= 1.0,

            // Bevy-style: forward is -Z
            k if k == bindings.forward => dir.z -= 1.0,
            k if k == bindings.back => dir.z += 1.0,

            _ => {}
        }
    }

    // Normalize safely (zero stays zero, no diagonal speed boost)
    dir = dir.normalize_or_zero();

    // Apply intent to all player entities (exactly one for now)
    for mut move_input in &mut q_player_input {
        move_input.0 = dir;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use bevy::ecs::system::RunSystemOnce;

    fn setup_world_with_player() -> (World, Entity) {
        let mut world = World::new();

        // Keyboard state resource (we control it manually via press/release).
        world.insert_resource(ButtonInput::<KeyCode>::default());

        // One player with initial (non-zero) input so we can detect "unchanged".
        let e = world.spawn((Player, MoveInput(Vec3::new(9.0, 9.0, 9.0)))).id();

        (world, e)
    }

    #[test]
    fn early_return_when_bindings_missing() {
        let (mut world, e) = setup_world_with_player();

        // No PlayerKeybindings inserted -> system should early-return
        let _ = world.run_system_once(read_player_input);

        let got = world.entity(e).get::<MoveInput>().unwrap().0;
        assert_eq!(got, Vec3::new(9.0, 9.0, 9.0));
    }

    #[test]
    fn forward_is_negative_z() {
        let (mut world, e) = setup_world_with_player();
        world.insert_resource(PlayerKeybindings::default());

        // Press W
        world
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyW);

        let _ = world.run_system_once(read_player_input);

        let got = world.entity(e).get::<MoveInput>().unwrap().0;
        assert_eq!(got, Vec3::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn diagonal_is_normalized_no_speed_boost() {
        let (mut world, e) = setup_world_with_player();
        world.insert_resource(PlayerKeybindings::default());

        let mut keyboard = world.resource_mut::<ButtonInput<KeyCode>>();
        keyboard.press(KeyCode::KeyW); // -Z
        keyboard.press(KeyCode::KeyD); // +X
        drop(keyboard);

        let _ = world.run_system_once(read_player_input);

        let got = world.entity(e).get::<MoveInput>().unwrap().0;

        // Expected direction: (1,0,-1) normalized
        let expected = Vec3::new(1.0, 0.0, -1.0).normalize();
        assert!((got - expected).length() < 1e-6, "got={got:?} expected={expected:?}");

        // Also ensure it's unit length (or zero).
        assert!((got.length() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn opposite_keys_cancel_out() {
        let (mut world, e) = setup_world_with_player();
        world.insert_resource(PlayerKeybindings::default());

        let mut keyboard = world.resource_mut::<ButtonInput<KeyCode>>();
        keyboard.press(KeyCode::KeyW);
        keyboard.press(KeyCode::KeyS);
        drop(keyboard);

        let _ = world.run_system_once(read_player_input);

        let got = world.entity(e).get::<MoveInput>().unwrap().0;
        assert_eq!(got, Vec3::ZERO);
    }

    #[test]
    fn custom_bindings_are_respected() {
        let (mut world, e) = setup_world_with_player();

        // Remap: forward = ArrowUp instead of W
        let mut bindings = PlayerKeybindings::default();
        bindings.forward = KeyCode::ArrowUp;
        world.insert_resource(bindings);

        world
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::ArrowUp);

        let _ = world.run_system_once(read_player_input);

        let got = world.entity(e).get::<MoveInput>().unwrap().0;
        assert_eq!(got, Vec3::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn applies_to_all_players() {
        let mut world = World::new();
        world.insert_resource(ButtonInput::<KeyCode>::default());
        world.insert_resource(PlayerKeybindings::default());

        let e1 = world.spawn((Player, MoveInput(Vec3::ZERO))).id();
        let e2 = world.spawn((Player, MoveInput(Vec3::ZERO))).id();

        world
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyD);

        let _ = world.run_system_once(read_player_input);

        let a = world.entity(e1).get::<MoveInput>().unwrap().0;
        let b = world.entity(e2).get::<MoveInput>().unwrap().0;
        assert_eq!(a, Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(b, Vec3::new(1.0, 0.0, 0.0));
    }
}
