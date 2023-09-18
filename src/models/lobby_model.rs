use std::str::FromStr;

use exaworld_dependencies::{DatabaseError, LOBBY_PLAYER_MODELS_SEPARATOR, LOBBY_SEPARATOR};
use serde::{Deserialize, Serialize};

use crate::{DbModel, PlayerModel, WorldModel};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LobbyPlayerModel {
    pub id: u64,
    pub model: PlayerModel,
}

impl ToString for LobbyPlayerModel {
    fn to_string(&self) -> String {
        format!(
            "{}{}{}",
            self.id,
            LOBBY_PLAYER_MODELS_SEPARATOR,
            self.model.to_string()
        )
    }
}

impl FromStr for LobbyPlayerModel {
    type Err = DatabaseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split_string: Vec<&str> = s.split(LOBBY_PLAYER_MODELS_SEPARATOR).collect();

        if split_string.len() != 2 {
            return Err(DatabaseError::EntryCouldNotBeParsed(format!(
                "The entry ('{}') could not be parsed while trying to get a player model for the lobby. The number of fields isn't correct.",
                s
            )));
        }

        let Ok(id) = split_string[0].parse::<u64>() else {
            return Err(DatabaseError::EntryCouldNotBeParsed(format!("The entry ('{}') could not be parsed while trying to get the id of the player model for the lobby.", s)));
        };

        let Ok(model) = split_string[1].parse::<PlayerModel>() else {
            return Err(DatabaseError::EntryCouldNotBeParsed(format!("The entry ('{}') could not be parsed while trying to parse the player model for a player model entry for the lobby.", s)));
        };

        Ok(Self { id, model })
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LobbyModel {
    pub name: String,
    pub world: WorldModel,
    pub players: Vec<LobbyPlayerModel>,
}

impl FromStr for LobbyModel {
    type Err = DatabaseError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let split_string: Vec<_> = string.split(LOBBY_SEPARATOR).collect();

        if split_string.len() != 3 {
            return Err(DatabaseError::EntryCouldNotBeParsed(format!("Could not construct a lobby from '{}' because the fields provided aren't exactly 2.", string)));
        }

        let Ok(world) = split_string[1].parse::<WorldModel>() else {
            return Err(DatabaseError::EntryCouldNotBeParsed(format!("Could not construct a lobby from '{}' because the world data ('{}') could not be constructed.", string, split_string[1])));
        };

        let name = split_string[0].to_string();

        let mut players: Vec<LobbyPlayerModel> = vec![];

        for player in split_string[2]
            .split(LOBBY_PLAYER_MODELS_SEPARATOR)
            .map(|x| x.trim())
            .collect::<Vec<&str>>()
        {
            if !player.is_empty() {
                let Ok(value) = player.parse::<LobbyPlayerModel>() else {
                    return Err(DatabaseError::EntryCouldNotBeParsed(format!("Could not construct a lobby from '{}' because a player's id ('{}') could not be parsed.", string, player)))
                };

                players.push(value);
            }
        }

        Ok(Self {
            name,
            world,
            players,
        })
    }
}

impl ToString for LobbyModel {
    fn to_string(&self) -> String {
        format!(
            "{}{}{}{}{}",
            self.name,
            LOBBY_SEPARATOR,
            self.world.to_string(),
            LOBBY_SEPARATOR,
            self.players
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(LOBBY_PLAYER_MODELS_SEPARATOR)
                .to_string()
        )
    }
}

impl DbModel for LobbyModel {}
