use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::cursor::Cursor;

pub struct CoinPlugin;

impl Plugin for CoinPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RandomizeEvent>()
            .add_startup_system(coin_setup)
            .add_system(coin_update)
            .add_system(coin_spawned)
            .add_system(coin_update);
    }
}

#[derive(Default)]
struct RandomizeEvent;

fn coin_setup(
    mut commands: Commands,
    mut skeletons: ResMut<Assets<SkeletonData>>,
    asset_server: Res<AssetServer>,
) {
    let skeleton = SkeletonData::new_from_json(
        asset_server.load("coin/export/coin-pro.json"),
        asset_server.load("coin/export/coin.atlas"),
    );
    let skeleton_handle = skeletons.add(skeleton);

    commands.spawn_bundle(bevy_spine::SpineBundle {
        skeleton: skeleton_handle.clone(),
        transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::ONE),
        ..Default::default()
    });
}

fn coin_spawned(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut randomize_events: EventWriter<RandomizeEvent>,
    mut spine_query: Query<&mut Spine>,
) {
    for _ in spine_ready_event.iter() {
        randomize_events.send_default();
        for mut spine in spine_query.iter_mut() {
            let mut track = spine
                .animation_state
                .set_animation_by_name(0, "animation", true)
                .unwrap();
            track.set_timescale(0.);
        }
    }
}

fn coin_update(mut spine_query: Query<&mut Spine>, cursor: Res<Cursor>) {
    for mut spine in spine_query.iter_mut() {
        if let Some(mut animation) = spine.animation_state.track_at_index_mut(0 as usize) {
            animation.set_track_time(cursor.position.x / 100. + 10000.);
        }
    }
}
