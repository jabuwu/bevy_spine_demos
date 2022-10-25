use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::cursor::Cursor;

const OWL_IDLE: i32 = 0;
const OWL_BLINK: i32 = 1;
const OWL_UP: i32 = 2;
const OWL_DOWN: i32 = 3;
const OWL_LEFT: i32 = 4;
const OWL_RIGHT: i32 = 5;

pub struct OwlPlugin;

impl Plugin for OwlPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RandomizeEvent>()
            .add_startup_system(owl_setup)
            .add_system(owl_update)
            .add_system(owl_spawned)
            .add_system(owl_update);
    }
}

#[derive(Default)]
struct RandomizeEvent;

fn owl_setup(
    mut commands: Commands,
    mut skeletons: ResMut<Assets<SkeletonData>>,
    asset_server: Res<AssetServer>,
) {
    let skeleton = SkeletonData::new_from_json(
        asset_server.load("owl/export/owl-pro.json"),
        asset_server.load("owl/export/owl.atlas"),
    );
    let skeleton_handle = skeletons.add(skeleton);

    commands.spawn_bundle(bevy_spine::SpineBundle {
        skeleton: skeleton_handle.clone(),
        transform: Transform::from_xyz(0., -170., 0.).with_scale(Vec3::ONE * 0.5),
        ..Default::default()
    });
}

fn owl_spawned(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut randomize_events: EventWriter<RandomizeEvent>,
    mut spine_query: Query<&mut Spine>,
) {
    for _ in spine_ready_event.iter() {
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

fn owl_update(mut spine_query: Query<&mut Spine>, cursor: Res<Cursor>) {
    for mut spine in spine_query.iter_mut() {
        let magnitude = (cursor.position.length() / 300.).clamp(0., 1.);
        let look = cursor.position.normalize() * magnitude;

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
