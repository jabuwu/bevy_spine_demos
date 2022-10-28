use bevy::prelude::*;
use bevy_spine::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{cursor::CursorPlugin, instructions::InstructionsPlugin};

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug, EnumIter)]
pub enum AppState {
    #[default]
    MixAndMatch,
    Owl,
    Coin,
}

#[derive(Component)]
pub struct Persistent;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins)
        .add_plugin(SpinePlugin)
        .add_plugin(CursorPlugin)
        .add_plugin(InstructionsPlugin)
        .add_plugin(mix_and_match::MixAndMatchPlugin)
        .add_plugin(owl::OwlPlugin)
        .add_plugin(coin::CoinPlugin)
        .add_state(AppState::default())
        .add_startup_system(setup)
        .add_system(next_demo)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(Persistent);

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section(
                "press space for next demo",
                TextStyle {
                    font: asset_server.load("FiraMono-Medium.ttf"),
                    font_size: 22.0,
                    color: Color::WHITE,
                },
            )
            .with_alignment(TextAlignment {
                horizontal: HorizontalAlign::Center,
                vertical: VerticalAlign::Center,
            }),
            transform: Transform::from_xyz(0., -320., 1.),
            ..Default::default()
        })
        .insert(Persistent);
}

fn cleanup(
    mut commands: Commands,
    entities: Query<Entity, (Without<Persistent>, Without<Parent>)>,
) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn next_demo(mut app_state: ResMut<State<AppState>>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Space) {
        let mut index = AppState::iter()
            .position(|state| state == *app_state.current())
            .unwrap();
        index = (index + 1) % AppState::iter().count();
        let _ = app_state.set(AppState::iter().nth(index).unwrap());
    }
}

pub mod coin;
pub mod cursor;
pub mod instructions;
pub mod mix_and_match;
pub mod owl;
