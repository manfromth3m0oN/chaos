mod chaos;

use bevy::{prelude::*, window::PresentMode};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Chaos Game".to_string(),
            width: 800.,
            height: 800.,
            present_mode: PresentMode::Immediate,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(chaos::ChaosGame)
        .run();
}
