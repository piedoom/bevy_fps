use bevy::{prelude::*, render::camera::PerspectiveProjection};
use bevy_rapier3d::{
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut scene_spawner: ResMut<SceneSpawner>,
    scene_handle: Res<Handle<DynamicScene>>,
) {
    /* Create the player. */
    let locked_dofs = RigidBodyMassPropsFlags::ROTATION_LOCKED_Y
        | RigidBodyMassPropsFlags::ROTATION_LOCKED_X
        | RigidBodyMassPropsFlags::ROTATION_LOCKED_Z;

    let rigid_body = RigidBodyBundle {
        position: Vec3::new(0.0, 10.0, 0.0).into(),
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
            GlobalTransform::from_translation(Vec3::new(0f32, 2f32, 0f32)),
            Transform::from_translation(Vec3::new(0f32, 2f32, 0f32)),
            Movement::default(),
            RigidBodyPositionSync::Discrete,
            Player,
        ))
        .insert_bundle(rigid_body)
        .insert_bundle(collider)
        .id();

    // add the cameras as a child so we can rotate them independently
    cmd.entity(player).with_children(|parent| {
        parent.spawn_bundle(PerspectiveCameraBundle {
            perspective_projection: PerspectiveProjection {
                fov: 90f32.to_radians(),
                near: 0.1f32,
                ..Default::default()
            },
            // Put the camera slightly above the center so we are taller
            transform: Transform::from_translation(Vec3::new(0.0, 2.0, 0.0)),
            ..Default::default()
        });
    });

    // let mesh = meshes.add(Mesh::from(shape::Box::new(10f32, 0.5, 10f32)));
    // let material = materials.add(StandardMaterial {
    //     base_color: Color::hex("ffd891").unwrap(),
    //     // vary key PBR parameters on a grid of spheres to show the effect
    //     unlit: true,
    //     ..Default::default()
    // });
    // // ground
    // let ground_collider = ColliderBundle {
    //     shape: ColliderShape::cuboid(10.0, 0.1, 10.0),
    //     ..Default::default()
    // };
    //  cmd.spawn_bundle(ground_collider)
    //      .insert_bundle(PbrBundle {
    //          mesh,
    //          material,
    //          ..Default::default()
    //      })
    //      .insert(ColliderDebugRender::from(Color::RED))
    //      .insert(ColliderPositionSync::Discrete);

    scene_spawner.spawn_dynamic(scene_handle.clone());
}
