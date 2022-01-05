use bevy::{
    input::{keyboard::KeyboardInput, mouse::MouseMotion},
    prelude::*,
    render::camera::Camera,
};
use bevy_rapier3d::{
    na::Vector3,
    prelude::{RigidBodyForces, RigidBodyMassProps, RigidBodyPosition, RigidBodyVelocity},
};

use crate::prelude::*;

pub(crate) struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(GameState::Main)
                .with_system(player_look_input_system.system())
                .with_system(player_move_input_system.system())
                .with_system(player_jump_input_system.system())
                .with_system(apply_look_system.system())
                .with_system(apply_movement_system.system())
                .with_system(apply_jump_system.system()),
        );
    }
}

/// Very roughly get input from the mouse/kb
fn player_look_input_system(
    mut mouse: EventReader<MouseMotion>,
    windows: Res<Windows>,
    query: Query<(Entity, &mut Movement), With<Player>>,
    time: Res<Time>,
) {
    let window = windows.get_primary().unwrap();
    let window_scale = window.height().min(window.width());
    let look_dir = mouse
        .iter()
        .fold(Vec2::ZERO, |acc, mouse| acc + mouse.delta);

    query.for_each_mut(|(entity, mut movement)| {
        // Update the `Fps` component with a new position based on the mouse delta
        movement.yaw += look_dir.x * window_scale * (movement.sensitivity / 1000f32);
        movement.pitch += look_dir.y * window_scale * (movement.sensitivity / 1000f32);
        movement.pitch = movement.pitch.clamp(-180f32, 0f32);

        // Apply the rotation to our camera
        let yaw_radians = movement.yaw.to_radians();
        let pitch_radians = movement.pitch.to_radians();

        movement.rotation_yaw = Quat::from_axis_angle(Vec3::Z, -yaw_radians);
        movement.rotation_pitch = Quat::from_axis_angle(-Vec3::X, pitch_radians);
    });
}

fn player_move_input_system(
    keys: Res<Input<KeyCode>>,
    players: Query<&mut Movement, With<Player>>,
) {
    let dir = keys.get_pressed().fold(Vec3::ZERO, |acc, k| {
        acc + match k {
            KeyCode::W => Vec3::Y,
            KeyCode::S => -Vec3::Y,
            KeyCode::A => -Vec3::X,
            KeyCode::D => Vec3::X,
            _ => Vec3::ZERO,
        }
    });
    players.for_each_mut(|mut movement| {
        movement.set_direction(dir);
    });
}

fn player_jump_input_system(keyboard_input: Res<Input<KeyCode>>, movements: Query<&mut Movement>) {
    let wants_to_jump = keyboard_input.pressed(KeyCode::Space);
    movements.for_each_mut(|mut movement| movement.wants_to_jump = wants_to_jump);
}

/// Apply the desired movement from our component to the actual transform
fn apply_look_system(
    mut transforms: Query<&mut Transform>,
    mut camera: Query<Entity, With<Camera>>,
    query: Query<(&mut Movement, &Children), With<Player>>,
) {
    query.for_each_mut(|(mut movement, children)| {
        // Find and rotate the camera
        children.iter().for_each(|e| {
            if let Ok(camera_transform) = camera.get_mut(*e) {
                if let Ok(mut trans) = transforms.get_mut(camera_transform) {
                    trans.rotation = movement.rotation_yaw * movement.rotation_pitch;
                    return;
                }
            }
        });
    });
}

/// Move the movements via the physics system
fn apply_movement_system(time: Res<Time>, bodies: Query<(&mut RigidBodyVelocity, &Movement)>) {
    // Combine the rotation of our camera with the forces we want to apply
    bodies.for_each_mut(|(mut body, movement)| {
        // Multiply our rotation by the new forward/strafe direction to get the final force
        let force: Vec3 = movement.facing().mul_vec3(movement.direction);
        let scaled_force = force * movement.acceleration * time.delta_seconds();

        // TODO: Check the normal of the next collision surface before movement is applied and move to contact &
        // adjust the slope as needed

        body.linvel += Vector3::from(scaled_force);
        if body.linvel.magnitude_squared() > movement.speed.powi(2) {
            // preserve y speed when clamping magnitude
            let fall_speed = body.linvel.z;
            body.linvel.set_magnitude(movement.speed);
            body.linvel.z = fall_speed;
        }
        return;
    });
}

fn apply_jump_system(movements: Query<(&mut RigidBodyVelocity, &Movement)>) {
    movements.for_each_mut(|(mut velocity, movement)| {
        if movement.wants_to_jump {
            velocity.linvel += Vector3::from(Vec3::new(0.0, 0.0, 1.0));
        }
    });
}
