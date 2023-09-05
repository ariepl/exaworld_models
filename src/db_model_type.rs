use exaworld_dependencies::DatabaseError;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{LobbyModel, MoodleObjectModel, PlayerModel, WorldModel};

#[derive(Debug, Clone, PartialEq, EnumIter, Serialize, Deserialize)]
pub enum DbTable {
    Lobbies,
    MoodleObjects,
    Players,
    Worlds,
}

impl DbTable {
    pub fn get_model_type_with_dummy(&self) -> ModelType {
        match self {
            Self::Lobbies => ModelType::Lobby(LobbyModel::default()),
            Self::Worlds => ModelType::World(WorldModel::default()),
            Self::Players => ModelType::Player(PlayerModel::default()),
            Self::MoodleObjects => ModelType::MoodleObject(MoodleObjectModel::default()),
        }
    }

    pub fn from_model_type(model_type: &ModelType) -> Self {
        match model_type {
            ModelType::World(_) => Self::Worlds,
            ModelType::Player(_) => Self::Players,
            ModelType::Lobby(_) => Self::Lobbies,
            ModelType::MoodleObject(_) => Self::MoodleObjects,
        }
    }
}

#[derive(Debug, Clone, PartialEq, EnumIter, Serialize, Deserialize)]
pub enum ModelType {
    Lobby(LobbyModel),
    MoodleObject(MoodleObjectModel),
    Player(PlayerModel),
    World(WorldModel),
}

impl Display for DbTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lobbies => write!(f, "lobbies"),
            Self::MoodleObjects => write!(f, "moodle_objects"),
            Self::Players => write!(f, "players"),
            Self::Worlds => write!(f, "worlds"),
        }
    }
}

impl FromStr for DbTable {
    type Err = DatabaseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for table in DbTable::iter() {
            if table.to_string().to_lowercase() == s.to_lowercase() {
                return Ok(table);
            };
        }

        Err(Self::Err::IncorrectTableName(
            "The table name entered is incorrect.".into(),
        ))
    }
}

impl ModelType {
    pub fn to_model_string(&self) -> String {
       match self {
            Self::World(model) => model.to_string(),
            Self::Lobby(model) => model.to_string(),
            Self::MoodleObject(model) => model.to_string(),
            Self::Player(model) => model.to_string(),
       }
    }

    pub fn from_model_str(string: &str, table: DbTable) -> Result<ModelType, DatabaseError> {
        match table.get_model_type_with_dummy() {
            Self::Lobby(_) =>  {
                let Ok(model) = string.parse::<LobbyModel>() else {
                    return Err(DatabaseError::EntryCouldNotBeParsed(format!("Could not parse stringto lobby model.")));
                };
                return Ok(Self::Lobby(model))
            }
            Self::World(_) =>  {
                let Ok(model) = string.parse::<WorldModel>() else {
                    return Err(DatabaseError::EntryCouldNotBeParsed(format!("Could not parse stringto world model.")));
                };
                return Ok(Self::World(model))
            }
            Self::Player(_) => {
                let Ok(model) = string.parse::<PlayerModel>() else {
                    return Err(DatabaseError::EntryCouldNotBeParsed(format!("Could not parse stringto player model.")));
                };
                return Ok(Self::Player(model))
            }
            Self::MoodleObject(_) => {
                let Ok(model) = string.parse::<MoodleObjectModel>() else {
                    return Err(DatabaseError::EntryCouldNotBeParsed(format!("Could not parse stringto moodle object model.")));
                };
                return Ok(Self::MoodleObject(model))
            }
        }
    }
}
