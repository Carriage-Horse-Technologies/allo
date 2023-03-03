use serde::{Deserialize, Serialize};

/**
location-provierから送られてくる位置情報
*/
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct CharacterLocations {
    pub(crate) action: LocationType,
    pub(crate) characters: Vec<Character>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum LocationType {
    UpdateCharacterPos,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct Character {
    pub(crate) pos_x: f64,
    pub(crate) pos_y: f64,
    pub(crate) url: String,
    pub(crate) user_id: String,
}
