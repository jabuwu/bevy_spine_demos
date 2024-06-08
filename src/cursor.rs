use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum CursorSystem {
    Update,
}

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Cursor>()
            .add_systems(PreUpdate, cursor_position.in_set(CursorSystem::Update));
    }
}

#[derive(Resource)]
pub struct Cursor {
    pub position: Vec2,
}

impl Default for Cursor {
    fn default() -> Self {
        Self {
            position: Vec2::new(-9999.0, -9999.0),
        }
    }
}

fn cursor_position(
    mut cursor: ResMut<Cursor>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    let Ok((camera, camera_global_transform)) = camera_query.get_single() else {
        return;
    };
    let Ok(window) = window_query.get_single() else {
        return;
    };
    let Some(cursor_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_global_transform, cursor))
        .map(|ray| ray.origin.truncate())
    else {
        return;
    };
    cursor.position = cursor_position;
}
