use bevy::prelude::*;
use visualizer::{update_revolute_joints, Link, RevoluteJoint};

#[derive(Component)]
struct Body;

#[derive(Component)]
struct Shoulder;

#[derive(Component)]
struct Arm;

#[derive(Component)]
struct Wrist;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 4_000.,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (rotating_system, update_revolute_joints))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Body,
            SceneBundle {
                scene: asset_server.load("body.glb#Scene0"),
                transform: Transform::from_translation(Vec3::new(0., 300., 0.)),
                ..Default::default()
            },
            Link,
            RevoluteJoint {
                translation: Vec3::new(0., -20., 0.),
                axis: Vec3::X,
                angle: 0.,
            },
        ))
        .with_children(|body| {
            body
                .spawn((
                    Shoulder,
                    SceneBundle {
                        scene: asset_server.load("right_shoulder.glb#Scene0"),
                        transform: Transform::from_translation(Vec3::new(-100., 0., -50.)),
                        ..Default::default()
                    },
                    Link,
                    RevoluteJoint {
                        translation: Vec3::new(0., -20., 0.),
                        axis: Vec3::X,
                        angle: 0.,
                    },
                ))
                .with_children(|shoulder| {
                    shoulder
                        .spawn((
                            Arm,
                            SceneBundle {
                                scene: asset_server.load("right_arm.glb#Scene0"),
                                transform: Transform::from_translation(Vec3::new(0., -100., -30.)),
                                ..Default::default()
                            },
                            Link,
                            RevoluteJoint {
                                translation: Vec3::new(0., -130., 0.),
                                axis: Vec3::Z,
                                angle: 0.,
                            },
                        ))
                        .with_children(|arm| {
                            arm.spawn((
                                Wrist,
                                SceneBundle {
                                    scene: asset_server.load("right_wrist.glb#Scene0"),
                                    transform: Transform::from_translation(Vec3::new(
                                        0., -130., 0.,
                                    )),
                                    ..Default::default()
                                },
                                Link,
                                RevoluteJoint {
                                    translation: Vec3::new(0., -130., 0.),
                                    axis: Vec3::Z,
                                    angle: 0.,
                                },
                            ));
                        });
                });
        });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-500., 500., 500.)
            .looking_at(Vec3::new(0., 200., 0.), Vec3::Y),
        ..default()
    });
}

fn rotating_system(time: Res<Time>, mut query: Query<&mut RevoluteJoint, With<Shoulder>>) {
    for mut joint in &mut query {
        let rotation_angle = time.elapsed_seconds() * 0.01;
        joint.angle = rotation_angle;
    }
}
