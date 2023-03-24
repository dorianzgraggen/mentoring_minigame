use bevy::gltf::Gltf;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_system(print_keyboard_event_system)
        // .add_startup_system(setup_physics)
        // .add_startup_system(load_gltf)
        // .add_system(spawn_gltf_objects)
        .add_startup_system(setup)
        .run();
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

fn print_keyboard_event_system(mut keyboard_input_events: EventReader<KeyboardInput>) {
    for event in keyboard_input_events.iter() {
        info!("{:?}", event);
    }
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
            if let Ok(mut transform) = transforms.get_mut(entity) {
                transform.translation = Vec3::new(
                    offset * time.elapsed_seconds().sin() / 20.,
                    0.,
                    time.elapsed_seconds().cos() / 20.,
                );
                offset += 1.0;
            }
        }
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

#[derive(Resource)]
struct MyAssetPack(Handle<Gltf>);

#[derive(Component)]
struct LoadedScene;

fn load_gltf(mut commands: Commands, ass: Res<AssetServer>) {
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

    let gltf = ass.load("level.gltf");
    commands.insert_resource(MyAssetPack(gltf));
}

fn spawn_gltf_objects(
    mut commands: Commands,
    my: Res<MyAssetPack>,
    assets_gltf: Res<Assets<Gltf>>,
) {
    // if the GLTF has loaded, we can navigate its contents
    if let Some(gltf) = assets_gltf.get(&my.0) {
        // spawn the first scene in the file
        commands.spawn(SceneBundle {
            scene: gltf.scenes[0].clone(),
            ..Default::default()
        });

        // spawn the scene named "YellowCar"
        commands.spawn(SceneBundle {
            scene: gltf.named_scenes["YellowCar"].clone(),
            transform: Transform::from_xyz(1.0, 2.0, 3.0),
            ..Default::default()
        });

        let s = gltf.named_scenes["YellowCar"].clone();

        // PERF: the `.clone()`s are just for asset handles, don't worry :)
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

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            // println!("hello {}!", name.0);
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_startup_system(add_people)
            // .add_system(hello_world)
            .add_system(greet_people);
    }
}
