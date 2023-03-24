use bevy::prelude::*;

pub struct GameScenePlugin;

impl Plugin for GameScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_startup_system(handle_gltf_scene);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 10.0, 0.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::X),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn((
        SceneBundle {
            scene: asset_server.load("level.gltf#Scene0"),
            ..default()
        },
        LoadedScene,
    ));
}

fn handle_gltf_scene(
    time: Res<Time>,
    moved_scene: Query<Entity, With<LoadedScene>>,
    children: Query<&Children>,
    mut transforms: Query<&mut Transform>,
) {
    for moved_scene_entity in &moved_scene {
        let mut offset = 0.;
        // println("{:#?}", moved_scene_entity.);
        for entity in children.iter_descendants(moved_scene_entity) {
            println!("entitity");

            // if let Ok(mut transform) = transforms.get_mut(entity) {
            //     transform.translation = Vec3::new(
            //         offset * time.elapsed_seconds().sin() / 20.,
            //         0.,
            //         time.elapsed_seconds().cos() / 20.,
            //     );
            //     offset += 1.0;
            // }
        }
    }
}

#[derive(Component)]
pub struct LoadedScene;
