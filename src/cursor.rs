use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum CursorSystem {
    Update,
}

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Cursor>().add_system_to_stage(
            CoreStage::PreUpdate,
            cursor_position.label(CursorSystem::Update),
        );
    }
}

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
    windows: Res<Windows>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    if let Some(window) = windows.get_primary() {
        if let Some(position) = window.cursor_position() {
            if let Ok((camera, camera_transform)) = camera.get_single() {
                let window_size = Vec2::new(window.width() as f32, window.height() as f32);
                let ndc = (position / window_size) * 2.0 - Vec2::ONE;
                let ndc_to_world =
                    camera_transform.compute_matrix() * camera.projection_matrix().inverse();
                let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
                let world_pos: Vec2 = world_pos.truncate();
                cursor.position = world_pos;
            }
        }
    }
}
