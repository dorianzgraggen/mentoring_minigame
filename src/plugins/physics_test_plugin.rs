use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct PhysicsTestPlugin;

impl Plugin for PhysicsTestPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_physics);
    }
}

fn setup_physics(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-3.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        .insert(Restitution::coefficient(0.7))
        .insert(SceneBundle {
            scene: asset_server.load("level.gltf#Scene0"),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));
}
