use std::str::FromStr;

use exaworld_dependencies::{Coords, DatabaseError, WORLD_SEPARATOR, MOODLE_OBJECT_SEPARATOR};
use serde::{Deserialize, Serialize};

use crate::{MoodleObjectModel, DbModel};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldModel {
    pub width: f32,
    pub height: f32,
    pub spawn_point: Coords,
    pub moodle_objects: Vec<MoodleObjectModel>,
}

impl FromStr for WorldModel {
    type Err = DatabaseError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let split_string: Vec<_> = string.split(WORLD_SEPARATOR).collect();

        if split_string.len() != 4 {
            return Err(DatabaseError::EntryCouldNotBeParsed(format!("World data could not be parsed from '{}' because the number of fields is not correct.", string)));
        }

        let Ok(width) = split_string[0].parse::<f32>() else {
            return Err(DatabaseError::EntryCouldNotBeParsed(format!(
                "World data could not be parsed from '{}' because the width doesn't have a correct value.",
                string
            )));
        };

        let Ok(height) =  split_string[1].parse::<f32>() else {
            return Err(DatabaseError::EntryCouldNotBeParsed(format!(
                "World data could not be parsed from '{}' because the height doesn't have a correct value.",
                string
            )));
        };

        let Ok(spawn_point) = split_string[2].parse::<Coords>() else {
            return Err(DatabaseError::EntryCouldNotBeParsed(format!(
                "World data could not be parsed from '{}' because the spawn point doesn't have correct values.",
                string
            )));
        };

        let objects = split_string[3]
            .split(MOODLE_OBJECT_SEPARATOR)
            .collect::<Vec<_>>();

        let mut moodle_objects: Vec<MoodleObjectModel> = vec![];

        for object in objects {
            if object.is_empty() {
                continue;
            }

            match object.parse::<MoodleObjectModel>() {
                Ok(moodle_object) => {
                    moodle_objects.push(moodle_object);
                    continue;
                }

                Err(_) => {
                    return Err(DatabaseError::EntryCouldNotBeParsed(format!(
                        "World data could not be parsed from '{}'. A moodle object could not be constructed from '{}'.",
                        string,
                        object
                    )));
                }
            }
        }

        Ok(Self {
            width,
            height,
            spawn_point,
            moodle_objects,
        })
    }
}

impl ToString for WorldModel {
    fn to_string(&self) -> String {
        format!(
            "{}{}{}{}{}{}{}",
            self.width,
            WORLD_SEPARATOR,
            self.height,
            WORLD_SEPARATOR,
            self.spawn_point.to_string(),
            WORLD_SEPARATOR,
            self.moodle_objects
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(MOODLE_OBJECT_SEPARATOR)
        )
    }
}

impl DbModel for WorldModel {}
