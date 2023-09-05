use std::str::FromStr;

use exaworld_dependencies::{DatabaseError, LOBBY_SEPARATOR, LOBBY_PLAYERS_IDS_SEPARATOR};
use serde::{Deserialize, Serialize};

use crate::{WorldModel, DbModel};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LobbyModel {
    pub name: String,
    pub world: WorldModel,
    pub players_ids: Vec<u64>,
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

        let mut players_ids: Vec<u64> = vec![];

        for player_id in split_string[2]
            .split(LOBBY_PLAYERS_IDS_SEPARATOR)
            .map(|x| x.trim())
            .collect::<Vec<&str>>()
        {
            if !player_id.is_empty() {
                let Ok(value) = player_id.parse::<u64>() else {
                    return Err(DatabaseError::EntryCouldNotBeParsed(format!("Could not construct a lobby from '{}' because a player's id ('{}') could not be parsed.", string, player_id)))
                };

                players_ids.push(value);
            }
        }

        Ok(Self {
            name,
            world,
            players_ids,
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
            self.players_ids
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(LOBBY_PLAYERS_IDS_SEPARATOR)
                .to_string()
        )
    }
}

impl DbModel for LobbyModel {}
