use bevy::{prelude::*, sprite::Anchor};

pub struct InstructionsPlugin;

impl Plugin for InstructionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InstructionsEvent>()
            .add_systems(Update, instructions_spawn);
    }
}

#[derive(Event)]
pub struct InstructionsEvent(pub &'static str);

fn instructions_spawn(
    mut instruction_events: EventReader<InstructionsEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for event in instruction_events.read() {
        commands.spawn(Text2dBundle {
            text: Text::from_section(
                event.0,
                TextStyle {
                    font: asset_server.load("FiraMono-Medium.ttf"),
                    font_size: 22.0,
                    color: Color::WHITE,
                },
            )
            .with_justify(JustifyText::Center),
            text_anchor: Anchor::Center,
            transform: Transform::from_xyz(0., 320., 1.),
            ..Default::default()
        });
    }
}
