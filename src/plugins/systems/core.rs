use bevy::{prelude::*, render::camera::PerspectiveProjection};
use bevy_rapier3d::{
    na::Vector,
    physics::{ColliderBundle, ColliderPositionSync, RigidBodyBundle, RigidBodyPositionSync},
    prelude::{ColliderMaterial, ColliderShape, RigidBodyMassPropsFlags},
    render::ColliderDebugRender,
};

use crate::prelude::*;

pub(crate) struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.insert_resource(Msaa { samples: 4 })
            .add_state(crate::GameState::default())
            .add_system_set(
                SystemSet::on_enter(GameState::Main).with_system(setup_system.system()),
            );
    }
}

fn setup_system(
    mut cmd: Commands,
    mut scene_spawner: ResMut<SceneSpawner>,
    mut physics_config: ResMut<bevy_rapier3d::physics::RapierConfiguration>,
    scene_handle: Res<Handle<DynamicScene>>,
) {
    physics_config.gravity = Vector::z() * -9.81;

    /* Create the player. */
    let locked_dofs = RigidBodyMassPropsFlags::ROTATION_LOCKED_Y
        | RigidBodyMassPropsFlags::ROTATION_LOCKED_X
        | RigidBodyMassPropsFlags::ROTATION_LOCKED_Z;

    let rigid_body = RigidBodyBundle {
        position: Vec3::new(0.0, 0.0, 2.0).into(),
        mass_properties: locked_dofs.into(),
        ..Default::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::ball(1f32),
        material: ColliderMaterial {
            restitution: 0f32,
            friction: 3f32,
            ..Default::default()
        },
        ..Default::default()
    };
    let player = cmd
        .spawn_bundle((
            GlobalTransform::from_translation(Vec3::new(0f32, 0f32, 2f32)),
            Transform::from_translation(Vec3::new(0f32, 0f32, 2f32)),
            Movement::default(),
            RigidBodyPositionSync::Discrete,
            Player,
        ))
        .insert_bundle(rigid_body)
        .insert_bundle(collider)
        .id();

    // add the cameras as a child so we can rotate them independently
    let mut trans = Transform::from_rotation(Quat::from_axis_angle(-Vec3::X, 90f32.to_radians()));
    trans.translation += Vec3::new(0.0, 0.0, 2.0);
    cmd.entity(player).with_children(|parent| {
        parent.spawn_bundle(PerspectiveCameraBundle {
            perspective_projection: PerspectiveProjection {
                fov: 100f32.to_radians(),
                near: 0.1f32,
                ..Default::default()
            },
            // Put the camera slightly above the center so we are taller
            transform: trans,
            ..Default::default()
        });
    });

    scene_spawner.spawn_dynamic(scene_handle.clone());
}
