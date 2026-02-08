use bevy::prelude::*;

/// Public module tree used by integration tests and the binary.
pub mod app;
pub mod features;
pub mod game;

/// Build the game `App` with all required plugins.
///
/// Contract:
/// - This is the one place where the "real game wiring" lives.
/// - Both `main.rs` and tests can use this to avoid drift.
pub fn build_app() -> App {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugins(app::AppPlugin)
        .add_plugins(game::GamePlugin);

    app
}

/// Run the game.
///
/// Keeping this in the library makes it easy to:
/// - reuse the same wiring in examples,
/// - create alternate entrypoints (headless, server mode),
/// - avoid duplication between `main.rs` and tests.
pub fn run() {
    build_app().run();
}
