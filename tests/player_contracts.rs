use bevy::prelude::*;
use to_be_free::features::player::{input::PlayerKeybindings, PlayerPlugin};

#[test]
fn player_plugin_inserts_keybindings_resource() {
    let mut app = App::new();

    app.add_plugins(PlayerPlugin);

    assert!(
        app.world().contains_resource::<PlayerKeybindings>(),
        "PlayerPlugin must insert PlayerKeybindings so input system can run."
    );
}
