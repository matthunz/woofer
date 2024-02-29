use bevy::prelude::*;

use visualizer::{update_revolute_joints, Link, RevoluteJoint};

#[derive(Component)]
pub struct Body;

#[derive(Component)]
pub struct Shoulder;

#[derive(Component)]
pub struct Arm;

#[derive(Component)]
pub struct Wrist;

#[derive(Component)]
pub struct Front;

#[derive(Component)]
pub struct Back;

#[derive(Component)]
pub struct Right;

#[derive(Component)]
pub struct Left;

#[derive(Component)]
pub struct Leg<X, Z> {
    x: X,
    z: Z,
}

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
            spawn_leg(body, &asset_server, Front, Left, true, true);
            spawn_leg(body, &asset_server, Front, Right, true, false);
            spawn_leg(body, &asset_server, Back, Left, false, true);
            spawn_leg(body, &asset_server, Back, Right, false, false);
        });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(500., 500., -500.)
            .looking_at(Vec3::new(0., 200., 0.), Vec3::Y),
        ..default()
    });
}

fn spawn_leg<X, Z>(
    body: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    x: X,
    z: Z,
    is_front: bool,
    is_left: bool,
) where
    X: Component,
    Z: Component,
{
    body.spawn((
        Leg { x, z },
        Shoulder,
        SceneBundle {
            scene: asset_server.load("right_shoulder.glb#Scene0"),
            transform: Transform::from_translation(Vec3::new(
                if is_front { 100. } else { -100. },
                0.,
                if is_left { 50. } else { -50. },
            )),
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
                        transform: Transform::from_translation(Vec3::new(0., -130., 0.)),
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
}

fn rotating_system(time: Res<Time>, mut query: Query<&mut RevoluteJoint, With<Shoulder>>) {
    for mut joint in &mut query {
        let rotation_angle = time.elapsed_seconds() * 0.01;
        joint.angle = rotation_angle;
    }
}
