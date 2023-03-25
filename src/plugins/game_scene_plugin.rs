use bevy::prelude::*;

pub struct GameScenePlugin;

impl Plugin for GameScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(handle_gltf_scene);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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

    let player_mesh = meshes.add(shape::Capsule::default().into());
    let player_material = materials.add(StandardMaterial {
        base_color: Color::TOMATO,
        ..default()
    });

    commands.spawn((
        PbrBundle {
            mesh: player_mesh,
            material: player_material,
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        Player,
    ));
}

#[derive(Component)]
struct Player;

fn handle_gltf_scene(
    time: Res<Time>,
    moved_scene: Query<Entity, With<LoadedScene>>,
    children: Query<&Children>,
    mut transforms: Query<&mut Transform>,
    meshes: Res<Assets<Mesh>>,
) {
    // println!("============== begin");
    for (handle_id, mesh) in meshes.iter() {
        // println!("mesh {:#?}", handle_id);
        // TODO: add colliders
    }

    for moved_scene_entity in &moved_scene {
        if let Ok(t) = transforms.get_mut(moved_scene_entity) {
            // println!("transform: {:#?}", t.translation);
        }

        let mut offset = 0.;
        // println("{:#?}", moved_scene_entity.);
        for entity in children.iter_descendants(moved_scene_entity) {
            // println!("============== entitity");

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
