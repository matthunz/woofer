use bevy::math::Quat;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct LegState {
    pub  shoulder: f64,
    pub  arm: f64,
    pub  wrist: f64,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Event {
    pub   body: Quat,
    pub  front_left_leg: LegState,
    pub  front_right_leg: LegState,
    pub  back_left_leg: LegState,
    pub  back_right_leg: LegState,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case", tag = "kind", content = "data")]
pub enum Message {
    Pose { body: Quat },
}
