use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};
use bevy_egui::EguiContext;

#[derive(Component)]
pub struct GameCamera {
    pub position: Vec2,
    pub scroll: f32,
}

impl Default for GameCamera {
    fn default() -> Self {
        GameCamera {
            position: Vec2::ZERO,
            scroll: 1.0,
        }
    }
}

const MIN_ZOOM: f32 = 0.5;
const MAX_ZOOM: f32 = 3.0;
const KEYBOARD_SPEED: f32 = 10.0;

pub fn pan_orbit_camera(
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut game_camera: ResMut<GameCamera>,
    mut camera: Query<&mut Transform, With<Camera>>,
    mut egui_context: ResMut<EguiContext>,
) {
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
    game_camera.scroll -= scroll / 300.0;
    game_camera.scroll = game_camera.scroll.max(MIN_ZOOM).min(MAX_ZOOM);

    transform.translation = game_camera.position.extend(0.0);
    transform.scale = Vec3::new(game_camera.scroll, game_camera.scroll, game_camera.scroll);
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
