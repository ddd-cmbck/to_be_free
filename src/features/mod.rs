// src/features/mod.rs
use bevy::prelude::*;

pub mod player;

/// Registers all gameplay feature plugins.
///
/// This is intentionally minimal: it only composes feature plugins.
/// - `game::GamePlugin` decides *what features are enabled*
/// - `features/*` implement *how each feature works*
///
/// Keeping this as a dedicated plugin makes it trivial to:
/// - toggle features for experiments
/// - build different app modes (menu/gameplay/tests)
/// - swap implementations (e.g. different player controllers)
pub struct FeaturesPlugin;

impl Plugin for FeaturesPlugin {
    fn build(&self, app: &mut App) {
        // Minimal feature set for now: user-controlled player movement.
        app.add_plugins(player::PlayerPlugin);
    }
}
