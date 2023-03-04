use serde::{Deserialize, Serialize};

/**
location-provierから送られてくる位置情報
*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct CharacterLocations {
    pub(crate) action: LocationType,
    pub(crate) characters: Vec<Character>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum LocationType {
    UpdateCharacterPos,
    UpdateMyPos,
}

impl Default for LocationType {
    fn default() -> Self {
        Self::UpdateCharacterPos
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct Character {
    pub(crate) pos_x: f64,
    pub(crate) pos_y: f64,
    pub(crate) url: String,
    pub(crate) user_id: String,
}

/** サーバーに通知するための構造体 */
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct MyLocation {
    pub(crate) action: LocationType,
    pub(crate) pos_x: f64,
    pub(crate) pos_y: f64,
    pub(crate) user_id: String,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct ProductInfo {
    pub(crate) title: &'static str,
    pub(crate) url: &'static str,
    pub(crate) img_src: &'static str,
}
