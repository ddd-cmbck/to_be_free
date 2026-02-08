// src/game/mod.rs
use bevy::prelude::*;

mod scene;

use crate::features;

/// Game-level composition plugin.
///
/// This is where we wire together the "world" parts of the application:
/// - minimal scene setup (camera / light / any debug ground)
/// - gameplay feature plugins (player, later: rotation, collision, UI, etc.)
///
/// Design rule:
/// - `game` decides *what runs*
/// - `features/*` decide *how it runs*
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // Minimal scene: camera + light (kept separate from gameplay code).
        app.add_systems(Startup, scene::setup_scene);

        // Gameplay features.
        //
        // We keep feature registration centralized so it's easy to toggle/replace
        // features later (e.g. swap Player controller, add physics, etc.).
        app.add_plugins(features::FeaturesPlugin);
    }
}
