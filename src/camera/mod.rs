use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};
use bevy_egui::EguiContext;

use crate::map::MapSize;

#[derive(Component)]
pub struct GameCamera {
    pub position: Vec2,
    pub scroll: f32,
    pub pan_min: Vec2,
    pub pan_max: Vec2,
}

impl Default for GameCamera {
    fn default() -> Self {
        GameCamera {
            position: Vec2::ZERO,
            scroll: 1.0,
            pan_min: (0., 0.).into(),
            pan_max: (0., 0.).into(),
        }
    }
}

const MIN_ZOOM: f32 = 0.25;
const MIN_MAX_ZOOM: f32 = 1.0;
const KEYBOARD_SPEED: f32 = 10.0;

pub fn pan_orbit_camera(
    mouse_events: (
        EventReader<MouseMotion>,
        EventReader<MouseWheel>,
        Res<Input<MouseButton>>,
    ),
    keyboard_input: Res<Input<KeyCode>>,
    mut game_camera: ResMut<GameCamera>,
    mut camera: Query<&mut Transform, With<Camera>>,
    mut egui_context: ResMut<EguiContext>,
    windows: Res<Windows>,
    map_size: Res<MapSize>,
) {
    let (mut ev_motion, mut ev_scroll, input_mouse) = mouse_events;

    let map_pixel_size = map_size.pixel_size();
    let window = windows.primary();
    let max_zoom_x = map_pixel_size.x / window.width();
    let max_zoom_y = map_pixel_size.y / window.height();

    let max_zoom = f32::max(max_zoom_x, max_zoom_y);
    let max_zoom = f32::max(MIN_MAX_ZOOM, max_zoom);

    let pan_button = MouseButton::Left;
    let mut pan = Vec2::ZERO;
    let mut scroll = 0.0;

    if !egui_context.ctx_mut().wants_pointer_input() {
        for ev in ev_scroll.iter() {
            scroll += ev.y;
        }

        if input_mouse.pressed(pan_button) {
            for ev in ev_motion.iter() {
                pan += ev.delta;
            }
        }
    }

    if !egui_context.ctx_mut().wants_keyboard_input() {
        if keyboard_input.pressed(KeyCode::A) {
            pan += Vec2::new(KEYBOARD_SPEED, 0.0);
        }
        if keyboard_input.pressed(KeyCode::D) {
            pan += Vec2::new(-KEYBOARD_SPEED, 0.0);
        }
        if keyboard_input.pressed(KeyCode::W) {
            pan += Vec2::new(0.0, KEYBOARD_SPEED);
        }
        if keyboard_input.pressed(KeyCode::S) {
            pan += Vec2::new(0.0, -KEYBOARD_SPEED);
        }

        if keyboard_input.pressed(KeyCode::Z) {
            scroll += 10.0;
        }
        if keyboard_input.pressed(KeyCode::X) {
            scroll -= 10.0;
        }
    }

    let mut transform = camera.single_mut();

    game_camera.position.x -= pan.x * game_camera.scroll;
    game_camera.position.y += pan.y * game_camera.scroll;

    game_camera.position = game_camera
        .position
        .clamp(game_camera.pan_min, game_camera.pan_max);

    game_camera.scroll -= scroll / 300.0;
    game_camera.scroll = game_camera.scroll.clamp(MIN_ZOOM, max_zoom);

    transform.translation = game_camera.position.extend(0.0);
    transform.scale = Vec3::new(game_camera.scroll, game_camera.scroll, game_camera.scroll);
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameCamera>()
            .add_startup_system(spawn_camera);
    }
}
