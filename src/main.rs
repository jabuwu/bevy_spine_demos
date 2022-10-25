use bevy::prelude::*;
use bevy_spine::{prelude::*, rusty_spine::Skin};
use rand::{seq::SliceRandom, thread_rng};

#[derive(Default)]
struct RandomizeEvent;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SpinePlugin)
        .add_event::<RandomizeEvent>()
        .add_startup_system(setup)
        .add_system(mix_and_match_spawned)
        .add_system(mix_and_match_dress_up)
        .add_system(mix_and_match_input)
        .run();
}

fn setup(
    mut commands: Commands,
    mut skeletons: ResMut<Assets<SkeletonData>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    let skeleton = SkeletonData::new_from_json(
        asset_server.load("mix-and-match/export/mix-and-match-pro.json"),
        asset_server.load("mix-and-match/export/mix-and-match.atlas"),
    );
    let skeleton_handle = skeletons.add(skeleton);

    commands.spawn_bundle(bevy_spine::SpineBundle {
        skeleton: skeleton_handle.clone(),
        transform: Transform::from_xyz(0., -200., 0.).with_scale(Vec3::ONE * 0.5),
        ..Default::default()
    });
}

fn mix_and_match_spawned(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut randomize_events: EventWriter<RandomizeEvent>,
    mut spine_query: Query<&mut Spine>,
) {
    for _ in spine_ready_event.iter() {
        randomize_events.send_default();
        for mut spine in spine_query.iter_mut() {
            let _ = spine.animation_state.set_animation_by_name(0, "idle", true);
            let _ = spine
                .animation_state
                .set_animation_by_name(1, "dress-up", false);
        }
    }
}

fn mix_and_match_dress_up(
    mut spine_query: Query<&mut Spine>,
    mut randomize_events: EventReader<RandomizeEvent>,
) {
    for _ in randomize_events.iter() {
        for mut spine in spine_query.iter_mut() {
            let mut rng = thread_rng();
            let skin = {
                let skeleton_data = spine.skeleton.data();
                let mut skin = Skin::new("custom");
                let child_skins = [
                    "skin-base",
                    [
                        "accessories/backpack",
                        "accessories/bag",
                        "accessories/cape-blue",
                        "accessories/cape-red",
                        "accessories/hat-pointy-blue-yellow",
                        "accessories/hat-red-yellow",
                    ]
                    .choose(&mut rng)
                    .unwrap(),
                    [
                        "clothes/dress-blue",
                        "clothes/dress-green",
                        "clothes/hoodie-blue-and-scarf",
                        "clothes/hoodie-orange",
                    ]
                    .choose(&mut rng)
                    .unwrap(),
                    ["eyelids/girly", "eyelids/semiclosed"]
                        .choose(&mut rng)
                        .unwrap(),
                    ["eyes/eyes-blue", "eyes/green", "eyes/violet", "eyes/yellow"]
                        .choose(&mut rng)
                        .unwrap(),
                    ["hair/blue", "hair/brown", "hair/pink", "hair/short-red"]
                        .choose(&mut rng)
                        .unwrap(),
                    [
                        "legs/boots-pink",
                        "legs/boots-red",
                        "legs/pants-green",
                        "legs/pants-jeans",
                    ]
                    .choose(&mut rng)
                    .unwrap(),
                    ["nose/long", "nose/short"].choose(&mut rng).unwrap(),
                ];
                for child_skin in child_skins.iter() {
                    skin.add_skin(skeleton_data.find_skin(child_skin).unwrap().as_ref());
                }
                skin
            };

            let _ = spine.skeleton.set_skin(&skin);

            if spine
                .animation_state
                .track_at_index(1)
                .map(|track| track.animation_time() == track.animation_end())
                .unwrap_or(true)
            {
                let _ = spine
                    .animation_state
                    .set_animation_by_name(1, "dress-up", false);
            }
        }
    }
}

fn mix_and_match_input(
    mut randomize_events: EventWriter<RandomizeEvent>,
    mouse_buttons: Res<Input<MouseButton>>,
) {
    if mouse_buttons.just_pressed(MouseButton::Left) {
        randomize_events.send_default();
    }
}
