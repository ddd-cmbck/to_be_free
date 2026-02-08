use bevy::prelude::*;
use to_be_free::app::AppPlugin;
use to_be_free::features::player::PlayerPlugin;
use to_be_free::features::player::component::Player;

#[test]
fn player_plugin_runs_with_app_prereqs_and_spawns_player() {
    let mut app = App::new();

    // App-level baseline
    app.add_plugins(MinimalPlugins);
    app.add_plugins(AppPlugin);
    app.add_plugins(PlayerPlugin);

    // Headless prerequisites for systems that normally depend on engine plugins:
    // - spawn_player needs asset storages
    app.insert_resource(Assets::<Mesh>::default());
    app.insert_resource(Assets::<StandardMaterial>::default());
    // - read_player_input needs keyboard input resource (normally created by InputPlugin)
    app.insert_resource(ButtonInput::<KeyCode>::default());

    // Run one frame: Startup should spawn the player, Update will run too (now safe).
    app.update();

    let world = app.world_mut();
    let count = world.query::<&Player>().iter(world).count();

    assert_eq!(count, 1, "Startup should spawn exactly one Player");
}
