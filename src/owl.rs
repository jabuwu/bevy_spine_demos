use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{cleanup, cursor::Cursor, instructions::InstructionsEvent, AppState};

const OWL_IDLE: usize = 0;
const OWL_BLINK: usize = 1;
const OWL_UP: usize = 2;
const OWL_DOWN: usize = 3;
const OWL_LEFT: usize = 4;
const OWL_RIGHT: usize = 5;

pub struct OwlPlugin;

impl Plugin for OwlPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RandomizeEvent>()
            .add_systems(OnEnter(AppState::Owl), owl_setup)
            .add_systems(OnExit(AppState::Owl), cleanup)
            .add_systems(Update, (owl_update, owl_spawned, owl_update));
    }
}

#[derive(Component)]
pub struct Owl;

#[derive(Default, Event)]
struct RandomizeEvent;

fn owl_setup(
    mut commands: Commands,
    mut skeletons: ResMut<Assets<SkeletonData>>,
    mut instructions_events: EventWriter<InstructionsEvent>,
    asset_server: Res<AssetServer>,
) {
    let skeleton = SkeletonData::new_from_json(
        asset_server.load("owl/export/owl-pro.json"),
        asset_server.load("owl/export/owl.atlas"),
    );
    let skeleton_handle = skeletons.add(skeleton);

    commands.spawn((
        bevy_spine::SpineBundle {
            skeleton: skeleton_handle.clone(),
            transform: Transform::from_xyz(0., -170., 0.).with_scale(Vec3::ONE * 0.5),
            ..Default::default()
        },
        Owl,
    ));

    instructions_events.send(InstructionsEvent(
        "move the mouse and owl will follow your cursor",
    ));
}

fn owl_spawned(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut randomize_events: EventWriter<RandomizeEvent>,
    mut spine_query: Query<&mut Spine, With<Owl>>,
) {
    for _ in spine_ready_event.read() {
        randomize_events.send_default();
        for mut spine in spine_query.iter_mut() {
            let _ = spine
                .animation_state
                .set_animation_by_name(OWL_IDLE, "idle", true);
            let _ = spine
                .animation_state
                .set_animation_by_name(OWL_BLINK, "blink", true);
            let _ = spine
                .animation_state
                .set_animation_by_name(OWL_UP, "up", true);
            let _ = spine
                .animation_state
                .set_animation_by_name(OWL_DOWN, "down", true);
            let _ = spine
                .animation_state
                .set_animation_by_name(OWL_LEFT, "left", true);
            let _ = spine
                .animation_state
                .set_animation_by_name(OWL_RIGHT, "right", true);
        }
    }
}

fn owl_update(mut spine_query: Query<&mut Spine, With<Owl>>, cursor: Res<Cursor>) {
    for mut spine in spine_query.iter_mut() {
        let magnitude = (cursor.position.length() / 300.).clamp(0., 1.);
        let look = cursor.position.normalize_or_zero() * magnitude;

        if let Some(mut up) = spine.animation_state.track_at_index_mut(OWL_UP as usize) {
            up.set_alpha(look.y.clamp(0., 1.));
        }
        if let Some(mut down) = spine.animation_state.track_at_index_mut(OWL_DOWN as usize) {
            down.set_alpha((-look.y).clamp(0., 1.));
        }
        if let Some(mut left) = spine.animation_state.track_at_index_mut(OWL_LEFT as usize) {
            left.set_alpha(look.x.clamp(0., 1.));
        }
        if let Some(mut right) = spine.animation_state.track_at_index_mut(OWL_RIGHT as usize) {
            right.set_alpha((-look.x).clamp(0., 1.));
        }
    }
}
