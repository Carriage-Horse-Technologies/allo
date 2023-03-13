use serde::{Deserialize, Serialize};
use web_sys::DomRect;

/// location-provierから送られてくる位置情報
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct CharacterLocations {
    pub(crate) action: LocationType,
    pub(crate) characters: Vec<Character>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct ChatMessage {
    pub(crate) action: LocationType,
    pub(crate) user_id: String,
    pub(crate) message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum LocationType {
    UpdateCharacterPos,
    UpdateMyPos,
    UpdateCharacterPosExample,
    ActionChatMessage,
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

/// サーバーに通知するための構造体
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

/// Page左上からの絶対位置を格納．
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct PageOffsetDomRect {
    top: f64,
    bottom: f64,
    left: f64,
    right: f64,
}

impl PageOffsetDomRect {
    /// `dom_rect`と`page_offset`から絶対位置を計算する．
    ///
    /// * `page_offset` - `(x, y)`
    pub(crate) fn from_dom_rect_and_page_offset(
        dom_rect: DomRect,
        page_offset: (f64, f64),
    ) -> Self {
        PageOffsetDomRect {
            top: dom_rect.top() + page_offset.1,
            bottom: dom_rect.bottom() + page_offset.1,
            left: dom_rect.left() + page_offset.0,
            right: dom_rect.right() + page_offset.0,
        }
    }

    pub(crate) fn top(&self) -> f64 {
        self.top
    }
    pub(crate) fn bottom(&self) -> f64 {
        self.bottom
    }
    pub(crate) fn left(&self) -> f64 {
        self.left
    }
    pub(crate) fn right(&self) -> f64 {
        self.right
    }
}
