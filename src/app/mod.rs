use bevy::prelude::*;

mod schedules;

pub use schedules::AppSet;

/// Centralized engine / application configuration plugin.
///
/// This plugin is the single authority for:
/// - Global engine configuration (fixed timestep, etc.)
/// - Schedule-level system sets (Update vs FixedUpdate ordering points)
///
/// Keeping this out of `main.rs` prevents startup from turning into a junk drawer.
pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Fixed timestep configuration.
        //
        // Bevy's default fixed timestep is 64 Hz.
        // We override it to 60 Hz by inserting Time<Fixed> at startup.
        // Ref: Bevy notes mention restoring 60 Hz via Time::<Fixed>::from_hz(60.0).
        // https://bevy-cheatbook.github.io/fundamentals/fixed-timestep.html
        app.insert_resource(Time::<Fixed>::from_hz(60.0));

        // Define ordering / grouping labels for systems.
        //
        // Important Bevy rule: system set configuration is stored *per schedule*,
        // so we configure sets separately for Update and FixedUpdate.
        // https://bevy-cheatbook.github.io/programming/system-sets.html
        app.configure_sets(Update, AppSet::Input);
        app.configure_sets(FixedUpdate, AppSet::FixedMovement);
    }
}
