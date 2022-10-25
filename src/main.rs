use bevy::prelude::*;
use bevy_spine::prelude::*;
use cursor::CursorPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins)
        .add_plugin(SpinePlugin)
        .add_plugin(CursorPlugin)
        //.add_plugin(mix_and_match::MixAndMatchPlugin)
        .add_plugin(owl::OwlPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

pub mod cursor;
pub mod mix_and_match;
pub mod owl;
