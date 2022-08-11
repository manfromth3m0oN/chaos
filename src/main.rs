mod chaos;

use bevy::{prelude::*, window::PresentMode};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Chaos Game".to_string(),
            width: 1024.,
            height: 768.,
            present_mode: PresentMode::Immediate,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(chaos::ChaosGame)
        .run();
}
