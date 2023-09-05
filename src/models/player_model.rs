use std::str::FromStr;

use exaworld_dependencies::{Rgba, Vector, Rotation, PLAYER_SEPARATOR};
use serde::{Deserialize, Serialize};

use crate::DbModel;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerStyle {
    pub color: Rgba,
}

impl ToString for PlayerStyle {
    fn to_string(&self) -> String {
        self.color.to_string()
    }
}

impl FromStr for PlayerStyle {
    type Err = Box<dyn std::error::Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            color: string.parse::<Rgba>()?,
        })
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerModel {
    pub position: Vector,
    pub direction: Rotation,
    pub style: PlayerStyle,
    pub username: String,
    pub lobby_id: Option<u64>,
}

impl FromStr for PlayerModel {
    type Err = Box<dyn std::error::Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let split_string: Vec<&str> = string.split(PLAYER_SEPARATOR).collect();

        if split_string.len() != 5 {
            return Err(format!("Incorrect number of fields. ('{}')", string).into());
        }

        let split_pos_string: Vec<&str> = split_string[1].split(":").collect();

        if split_pos_string.len() != 3 {
            return Err(format!("Incorrect number of fields. ('{}')", string).into());
        }

        let pos_x: f32 = split_pos_string[0].parse()?;
        let pos_z: f32 = split_pos_string[2].parse()?;
        let pos_y: f32 = split_pos_string[1].parse()?;

        Ok(Self {
            username: split_string[0].to_string(),
            position: Vector::new(pos_x, pos_y, pos_z),
            direction: Rotation::new(split_string[2].parse()?),
            style: PlayerStyle::from_str(split_string[3])?,
            lobby_id: split_string[4].trim().parse::<u64>().ok(),
        })
    }
}

impl ToString for PlayerModel {
    fn to_string(&self) -> String {
        format!(
            "{}{}{}:{}:{}{}{}{}{}{}{}",
            self.username,
            PLAYER_SEPARATOR,
            self.position.x,
            self.position.y,
            self.position.z,
            PLAYER_SEPARATOR,
            self.direction.get_rad(),
            PLAYER_SEPARATOR,
            self.style.to_string(),
            PLAYER_SEPARATOR,
            match self.lobby_id {
                Some(val) => val.to_string(),
                None => "".to_string(),
            },
        )
    }
}

impl DbModel for PlayerModel {}
