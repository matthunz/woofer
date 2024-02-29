use bevy::prelude::*;
use reqwest_eventsource::{Event as SseEvent, EventSource};
use tokio::{
    runtime::Runtime,
    sync::mpsc::{self, UnboundedSender},
};
use tokio_stream::StreamExt;
use visualizer::{update_revolute_joints, Link, RevoluteJoint};
use woofer::{Event, LegState};

pub struct Leg {
    shoulder: Entity,
    arm: Entity,
    wrist: Entity,
}

#[derive(Component)]
pub struct Plant {
    body: Entity,
    front_left_leg: Leg,
    front_right_leg: Leg,
    back_left_leg: Leg,
    back_right_leg: Leg,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 4_000.,
        })
        .add_event::<StreamEvent>()
        .add_systems(Startup, setup)
        .add_systems(Update, (read_stream, handle_event, update_revolute_joints))
        .run();
}

#[derive(Resource, Deref)]
struct StreamReceiver(mpsc::UnboundedReceiver<Event>);

#[derive(Event)]
struct StreamEvent(Event);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let (tx, rx) = mpsc::unbounded_channel();
    std::thread::spawn(move || {
        Runtime::new().unwrap().block_on(task(tx));
    });
    commands.insert_resource(StreamReceiver(rx));

    let mut cell = None;
    let body = commands
        .spawn((
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
            let front_left_leg = spawn_leg(body, &asset_server, true, true);
            let front_right_leg = spawn_leg(body, &asset_server, true, false);
            let back_left_leg = spawn_leg(body, &asset_server, false, true);
            let back_right_leg = spawn_leg(body, &asset_server, false, false);
            cell = Some([
                front_left_leg,
                front_right_leg,
                back_left_leg,
                back_right_leg,
            ]);
        })
        .id();

    let [front_left_leg, front_right_leg, back_left_leg, back_right_leg] = cell.unwrap();
    commands.spawn(Plant {
        body,
        front_left_leg,
        front_right_leg,
        back_left_leg,
        back_right_leg,
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(500., 500., -500.)
            .looking_at(Vec3::new(0., 200., 0.), Vec3::Y),
        ..default()
    });
}

fn spawn_leg(
    body: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,

    is_front: bool,
    is_left: bool,
) -> Leg {
    let mut arm = None;
    let mut wrist = None;

    let shoulder = body
        .spawn((
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
            arm = Some(
                shoulder
                    .spawn((
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
                        wrist = Some(
                            arm.spawn((
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
                            ))
                            .id(),
                        );
                    })
                    .id(),
            );
        })
        .id();

    Leg {
        shoulder,
        arm: arm.unwrap(),
        wrist: wrist.unwrap(),
    }
}

fn read_stream(mut receiver: ResMut<StreamReceiver>, mut events: EventWriter<StreamEvent>) {
    while let Ok(from_stream) = receiver.0.try_recv() {
        events.send(StreamEvent(from_stream));
    }
}

fn handle_event(
    mut reader: EventReader<StreamEvent>,
    mut plant_query: Query<&mut Plant>,
    mut joint_query: Query<&mut RevoluteJoint>,
) {
    for StreamEvent(event) in reader.read() {
        for plant in &mut plant_query {
            update_leg(
                &plant.front_left_leg,
                &event.front_left_leg,
                &mut joint_query,
            );
            update_leg(
                &plant.front_right_leg,
                &event.front_right_leg,
                &mut joint_query,
            );
            update_leg(&plant.back_left_leg, &event.back_left_leg, &mut joint_query);
            update_leg(
                &plant.back_right_leg,
                &event.back_right_leg,
                &mut joint_query,
            );
        }
    }
}

fn update_leg(leg: &Leg, state: &LegState, joint_query: &mut Query<&mut RevoluteJoint>) {
    let mut shoulder = joint_query.get_mut(leg.shoulder).unwrap();
    shoulder.angle = state.shoulder;

    let mut arm = joint_query.get_mut(leg.arm).unwrap();
    arm.angle = state.arm;

    let mut wrist = joint_query.get_mut(leg.wrist).unwrap();
    wrist.angle = state.wrist;
}

async fn task(tx: UnboundedSender<Event>) {
    let mut es = EventSource::get("http://localhost:8080/state");
    while let Some(event) = es.next().await {
        match event {
            Ok(SseEvent::Open) => println!("Connection Open!"),
            Ok(SseEvent::Message(message)) => {
                let event: Event = serde_json::from_str(&message.data).unwrap();
                tx.send(event).unwrap();
            }
            Err(err) => {
                println!("Error: {}", err);
                es.close();
            }
        }
    }
}
