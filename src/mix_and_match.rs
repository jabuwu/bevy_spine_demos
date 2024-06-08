use bevy::prelude::*;
use bevy_spine::prelude::*;
use rand::{seq::SliceRandom, thread_rng};

use crate::{cleanup, instructions::InstructionsEvent, AppState};

pub struct MixAndMatchPlugin;

impl Plugin for MixAndMatchPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RandomizeEvent>()
            .add_systems(OnEnter(AppState::MixAndMatch), mix_and_match_setup)
            .add_systems(OnExit(AppState::MixAndMatch), cleanup)
            .add_systems(
                Update,
                (
                    mix_and_match_spawned,
                    mix_and_match_dress_up.before(SpineSystem::UpdateAnimation),
                    mix_and_match_input,
                ),
            );
    }
}

#[derive(Component)]
pub struct MixAndMatch;

#[derive(Default, Event)]
struct RandomizeEvent;

fn mix_and_match_setup(
    mut commands: Commands,
    mut skeletons: ResMut<Assets<SkeletonData>>,
    mut instructions_events: EventWriter<InstructionsEvent>,
    asset_server: Res<AssetServer>,
) {
    let skeleton = SkeletonData::new_from_json(
        asset_server.load("mix-and-match/export/mix-and-match-pro.json"),
        asset_server.load("mix-and-match/export/mix-and-match.atlas"),
    );
    let skeleton_handle = skeletons.add(skeleton);

    commands.spawn((
        bevy_spine::SpineBundle {
            skeleton: skeleton_handle.clone(),
            transform: Transform::from_xyz(0., -200., 0.).with_scale(Vec3::ONE * 0.5),
            ..Default::default()
        },
        MixAndMatch,
    ));

    instructions_events.send(InstructionsEvent("left click to randomize outfit"));
}

fn mix_and_match_spawned(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut randomize_events: EventWriter<RandomizeEvent>,
    mut spine_query: Query<&mut Spine, With<MixAndMatch>>,
) {
    for _ in spine_ready_event.read() {
        randomize_events.send_default();
        for mut spine in spine_query.iter_mut() {
            let _ = spine.animation_state.set_animation_by_name(0, "idle", true);
        }
    }
}

fn mix_and_match_dress_up(
    mut spine_query: Query<&mut Spine, With<MixAndMatch>>,
    mut randomize_events: EventReader<RandomizeEvent>,
) {
    for _ in randomize_events.read() {
        for mut spine in spine_query.iter_mut() {
            let mut rng = thread_rng();
            let skins = {
                [
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
                ]
            };

            let _ = spine.skeleton.set_skins_by_name("combined", skins);

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
    mouse_buttons: Res<ButtonInput<MouseButton>>,
) {
    if mouse_buttons.just_pressed(MouseButton::Left) {
        randomize_events.send_default();
    }
}
