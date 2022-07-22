use serde::{Deserialize, Serialize};

use crate::ImageInfoData;

/// Album Info Response
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct AlbumInfo {
    /// Image Data
    pub data: AlbumInfoData,
    /// Request processed success or not.
    pub success: bool,
    /// HTTP status code from API request.
    pub status: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct AlbumInfoData {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub datetime: i64,
    pub cover: String,
    pub cover_edited: Option<String>,
    pub cover_width: i64,
    pub cover_height: i64,
    pub account_url: Option<String>,
    pub account_id: Option<String>,
    pub privacy: String,
    pub layout: String,
    pub views: i64,
    pub link: String,
    pub favorite: bool,
    pub nsfw: bool,
    pub section: Option<String>,
    pub images_count: i64,
    pub in_gallery: bool,
    pub is_ad: bool,
    pub include_album_ads: bool,
    pub is_album: bool,
    pub images: Vec<ImageInfoData>,
    pub ad_config: AdConfig,
}



#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct AdConfig {
    #[serde(rename = "safeFlags")]
    pub safe_flags: Vec<String>,
    #[serde(rename = "highRiskFlags")]
    pub high_risk_flags: Vec<String>,
    #[serde(rename = "unsafeFlags")]
    pub unsafe_flags: Vec<String>,
    #[serde(rename = "wallUnsafeFlags")]
    pub wall_unsafe_flags: Vec<String>,
    #[serde(rename = "showsAds")]
    pub shows_ads: bool,
    #[serde(rename = "showAdLevel")]
    pub show_ad_level: i64,
}