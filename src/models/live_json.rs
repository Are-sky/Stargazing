use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveJson {
    pub code: i64,
    pub msg: String,
    pub message: String,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub uid: i64,
    #[serde(rename = "room_id")]
    pub room_id: i64,
    #[serde(rename = "short_id")]
    pub short_id: i64,
    pub attention: i64,
    pub online: i64,
    #[serde(rename = "is_portrait")]
    pub is_portrait: bool,
    pub description: String,
    #[serde(rename = "live_status")]
    pub live_status: i64,
    #[serde(rename = "area_id")]
    pub area_id: i64,
    #[serde(rename = "parent_area_id")]
    pub parent_area_id: i64,
    #[serde(rename = "parent_area_name")]
    pub parent_area_name: String,
    #[serde(rename = "old_area_id")]
    pub old_area_id: i64,
    pub background: String,
    pub title: String,
    #[serde(rename = "user_cover")]
    pub user_cover: String,
    pub keyframe: String,
    #[serde(rename = "is_strict_room")]
    pub is_strict_room: bool,
    #[serde(rename = "live_time")]
    pub live_time: String,
    pub tags: String,
    #[serde(rename = "is_anchor")]
    pub is_anchor: i64,
    #[serde(rename = "room_silent_type")]
    pub room_silent_type: String,
    #[serde(rename = "room_silent_level")]
    pub room_silent_level: i64,
    #[serde(rename = "room_silent_second")]
    pub room_silent_second: i64,
    #[serde(rename = "area_name")]
    pub area_name: String,
    pub pendants: String,
    #[serde(rename = "area_pendants")]
    pub area_pendants: String,
    #[serde(rename = "hot_words")]
    pub hot_words: Vec<String>,
    #[serde(rename = "hot_words_status")]
    pub hot_words_status: i64,
    pub verify: String,
    #[serde(rename = "new_pendants")]
    pub new_pendants: Option<NewPendants>,
    #[serde(rename = "up_session")]
    pub up_session: String,
    #[serde(rename = "pk_status")]
    pub pk_status: i64,
    #[serde(rename = "pk_id")]
    pub pk_id: i64,
    #[serde(rename = "battle_id")]
    pub battle_id: i64,
    #[serde(rename = "allow_change_area_time")]
    pub allow_change_area_time: i64,
    #[serde(rename = "allow_upload_cover_time")]
    pub allow_upload_cover_time: i64,
    #[serde(rename = "studio_info")]
    pub studio_info: Option<StudioInfo>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewPendants {
    pub frame: Option<Frame>,
    pub badge: Option<Badge>,
    #[serde(rename = "mobile_frame")]
    pub mobile_frame: Option<MobileFrame>,
    #[serde(rename = "mobile_badge")]
    pub mobile_badge: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Frame {
    pub name: String,
    pub value: String,
    pub position: i64,
    pub desc: String,
    pub area: i64,
    #[serde(rename = "area_old")]
    pub area_old: i64,
    #[serde(rename = "bg_color")]
    pub bg_color: String,
    #[serde(rename = "bg_pic")]
    pub bg_pic: String,
    #[serde(rename = "use_old_area")]
    pub use_old_area: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Badge {
    pub name: String,
    pub position: i64,
    pub value: String,
    pub desc: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MobileFrame {
    pub name: String,
    pub value: String,
    pub position: i64,
    pub desc: String,
    pub area: i64,
    #[serde(rename = "area_old")]
    pub area_old: i64,
    #[serde(rename = "bg_color")]
    pub bg_color: String,
    #[serde(rename = "bg_pic")]
    pub bg_pic: String,
    #[serde(rename = "use_old_area")]
    pub use_old_area: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudioInfo {
    pub status: i64,
    #[serde(rename = "master_list")]
    pub master_list: Vec<Value>,
}
