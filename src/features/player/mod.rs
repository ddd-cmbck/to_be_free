// src/features/player/mod.rs
use bevy::prelude::*;

use crate::app::AppSet;

pub mod bundles;
pub mod component;
pub mod input;
pub mod movement;

/// Player feature plugin.
///
/// Scope (current slice):
/// - Spawns a single player entity at startup (Option A: player feature owns player entity)
/// - Update: reads keyboard input and writes local-space `MoveInput` intent
/// - FixedUpdate: converts local intent -> world velocity -> integrates position (temporary)
///
/// Design constraints:
/// - Input systems never write `Transform`.
/// - FixedUpdate owns movement stepping (collision/physics-ready pipeline).
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // Add default keybindings (can be overridden later by inserting your own resource).
        app.insert_resource(input::PlayerKeybindings::default());

        // Spawn the player entity (feature owns player).
        app.add_systems(Startup, bundles::spawn_player);

        // Input (variable timestep): keyboard -> MoveInput (local-space intent).
        app.add_systems(Update, input::read_player_input.in_set(AppSet::Input));

        // Movement (fixed timestep): intent -> velocity -> integration.
        //
        // We explicitly chain the movement pipeline to guarantee ordering.
        // This is robust and minimizes plugin cross-coupling.
        app.add_systems(
            FixedUpdate,
            (movement::compute_velocity_from_input, movement::integrate_velocity)
                .chain()
                .in_set(AppSet::FixedMovement),
        );
    }
}
