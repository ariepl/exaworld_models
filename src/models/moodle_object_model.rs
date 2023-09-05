use std::str::FromStr;

use exaworld_dependencies::{Coords, DatabaseError, MOODLE_OBJECT_SEPARATOR};
use serde::{Deserialize, Serialize};

use crate::DbModel;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MoodleObjectModel {
    pub coords: Coords,
    pub link: String,
}

impl FromStr for MoodleObjectModel {
    type Err = DatabaseError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let split_string: Vec<&str> = string.split(MOODLE_OBJECT_SEPARATOR).collect();

        if split_string.len() != 2 {
            return Err(DatabaseError::EntryCouldNotBeParsed(format!(
                "'{}' could not be parsed to a moodle object because the number of fileds is not 2.",
                string
            )));
        }

        Ok(Self {
            coords: split_string[0].parse::<Coords>()?,
            link: split_string[1].to_string(),
        })
    }
}

impl ToString for MoodleObjectModel {
    fn to_string(&self) -> String {
        format!(
            "{}{}{}",
            self.coords.to_string(),
            MOODLE_OBJECT_SEPARATOR,
            self.link
        )
    }
}

impl DbModel for MoodleObjectModel {}
