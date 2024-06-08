use bevy::{prelude::*, sprite::Anchor, window::WindowResolution};
use bevy_spine::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{cursor::CursorPlugin, instructions::InstructionsPlugin};

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug, EnumIter, States)]
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
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1024., 768.),
                title: "Spine Demos".to_owned(),
                ..Default::default()
            }),
            ..default()
        }))
        .add_plugins((
            SpinePlugin,
            CursorPlugin,
            InstructionsPlugin,
            mix_and_match::MixAndMatchPlugin,
            owl::OwlPlugin,
            coin::CoinPlugin,
        ))
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        .add_systems(Update, next_demo)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default()).insert(Persistent);

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "press space for next demo",
                TextStyle {
                    font: asset_server.load("FiraMono-Medium.ttf"),
                    font_size: 22.0,
                    color: Color::WHITE,
                },
            )
            .with_justify(JustifyText::Center),
            text_anchor: Anchor::Center,
            transform: Transform::from_xyz(0., -320., 1.),
            ..Default::default()
        },
        Persistent,
    ));
}

fn cleanup(
    mut commands: Commands,
    entities: Query<Entity, (Without<Persistent>, Without<Parent>, Without<Window>)>,
) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn next_demo(
    mut next_app_state: ResMut<NextState<AppState>>,
    app_state: ResMut<State<AppState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        let mut index = AppState::iter()
            .position(|state| state == *app_state.get())
            .unwrap();
        index = (index + 1) % AppState::iter().count();
        next_app_state.set(AppState::iter().nth(index).unwrap());
    }
}

pub mod coin;
pub mod cursor;
pub mod instructions;
pub mod mix_and_match;
pub mod owl;
