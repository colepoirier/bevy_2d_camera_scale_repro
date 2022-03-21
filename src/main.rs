use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_system(setup)
        .add_system(pan_zoom_camera_system)
        .add_system(camera_changed_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        ..default()
    });
}

pub fn pan_zoom_camera_system(
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<Input<MouseButton>>,
    mut q_camera: Query<&mut Transform, With<Camera>>,
) {
    // change input mapping for panning here.
    let pan_button = MouseButton::Left;

    let mut pan = Vec2::ZERO;
    let mut scroll = 0.0;

    if input_mouse.pressed(pan_button) {
        for ev in ev_motion.iter() {
            pan += ev.delta;
        }
    }

    for ev in ev_scroll.iter() {
        scroll += ev.y;
    }

    // assuming there is exacly one main camera entity, so this is ok.
    if let Ok(mut transform) = q_camera.get_single_mut() {
        if pan.length_squared() > 0.0 {
            let scale = transform.scale.x;
            transform.translation.x -= pan.x * scale / 4.0;
            transform.translation.y += pan.y * scale / 4.0;
        } else if scroll.abs() > 0.0 {
            let scale = transform.scale.x - scroll;
            transform.scale = Vec3::new(scale, scale, scale);
        }
    }
}

fn camera_changed_system(camera_q: Query<&Transform, (Changed<Transform>, With<Camera>)>) {
    for c in camera_q.iter() {
        info!("Camera new transform {:?}", c);
    }
}
