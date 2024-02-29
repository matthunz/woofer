use bevy::math::Quat;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct LegState {
    shoulder: f64,
    arm: f64,
    wrist: f64,
}

#[derive(Deserialize, Serialize)]
pub struct Event {
    body: Quat,
    front_left_leg: LegState,
    front_right_leg: LegState,
    back_left_leg: LegState,
    back_right_leg: LegState,
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "kind", content = "data")]
pub enum Message {
    Pose { body: Quat },
}
