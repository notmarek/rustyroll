use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    pub height: Option<u32>,
    pub source: Option<String>,
    pub r#type: Option<String>,
    pub width: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SeriesMetadata {
    pub availability_notes: Option<String>,
    pub episode_count: Option<u32>,
    pub is_dubbed: Option<bool>,
    pub is_mature: Option<bool>,
    pub is_simulcast: Option<bool>,
    pub is_subbed: Option<bool>,
    pub mature_blocked: Option<bool>,
    pub maturity_ratings: Option<Vec<String>>,
    pub season_count: Option<u32>,
    pub tenant_categories: Option<Vec<String>>,
    pub slug: Option<String>,
    pub title: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EpisodeMetadata {
    pub ad_breaks: Option<Vec<AdBreak>>,
    pub availability_notes: Option<String>,
    pub available_offline: Option<bool>,
    pub duration_ms: Option<u32>,
    pub episode: Option<String>,
    pub episode_air_date: Option<String>,
    pub episode_number: Option<u32>,
    pub is_clip: Option<bool>,
    pub is_dubbed: Option<bool>,
    pub is_mature: Option<bool>,
    pub is_premium_only: Option<bool>,
    pub is_subbed: Option<bool>,
    pub mature_blocked: Option<bool>,
    pub maturity_ratings: Option<Vec<String>>,
    pub season_id: Option<String>,
    pub season_number: Option<u32>,
    pub season_title: Option<String>,
    pub sequence_number: Option<u32>,
    pub series_id: Option<String>,
    pub series_title: Option<String>,
    pub subtitle_locales: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchMetadata {
    pub score: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
    pub struct AdBreak {
    pub offset_ms: Option<u32>,
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Links {
    #[serde(flatten)]
    pub continuation: Option<String>,  
    #[serde(flatten)]
    pub resource: Option<String>,
    #[serde(rename = "resource/channel")]
    #[serde(flatten)]
    pub resource_channel: Option<String>,
    #[serde(rename = "episode/season")]
    #[serde(flatten)]
    pub episode_season: Option<String>,
    #[serde(rename = "episode/series")]
    #[serde(flatten)]
    pub episode_series: Option<String>,
    #[serde(flatten)]
    pub streams: Option<String>,
  
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Images {
    pub poster_tall: Option<Vec<Vec<Image>>>,
    pub poster_wide: Option<Vec<Vec<Image>>>,
    pub thumbnail: Option<Vec<Vec<Image>>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaceholderEmpty {

}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchItem {
    #[serde(rename = "__actions__")]
    pub actions: Option<PlaceholderEmpty>,
    #[serde(rename = "__class__")]
    pub class: Option<String>,
    #[serde(rename = "__href__")]
    pub href: Option<String>,
    #[serde(rename = "__resource_key__")]
    pub resource_key: Option<String>,
    #[serde(rename = "__links__")]
    pub links: Option<Links>,
    pub r#type: Option<String>,
    pub channel_id: Option<String>,
    pub description: Option<String>,
    pub external_id: Option<String>,
    pub id: Option<String>,
    pub images: Option<Images>,
    pub linked_resource_key: Option<String>,
    pub new: Option<bool>,
    pub new_content: Option<bool>,
    pub promo_description: Option<String>,
    pub promo_title: Option<String>,
    pub search_metadata: Option<SearchMetadata>,
    pub series_metadata: Option<SeriesMetadata>,
    pub episode_metadata: Option<EpisodeMetadata>,
    pub slug: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchWrapper {
    #[serde(rename = "__actions__")]
    pub actions: Option<PlaceholderEmpty>,
    #[serde(rename = "__class__")]
    pub class: Option<String>,
    #[serde(rename = "__href__")]
    pub href: Option<String>,
    #[serde(rename = "__resource_key__")]
    pub resource_key: Option<String>,
    #[serde(rename = "__links__")]
    pub links: Option<Links>,
    pub r#type: Option<String>,
    pub total: Option<u32>,
    pub items: Option<Vec<SearchItem>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Search {
    #[serde(rename = "__actions__")]
    pub actions: Option<PlaceholderEmpty>,
    #[serde(rename = "__class__")]
    pub class: Option<String>,
    #[serde(rename = "__href__")]
    pub href: Option<String>,
    #[serde(rename = "__resource_key__")]
    pub resource_key: Option<String>,
    #[serde(rename = "__links__")]
    pub links: Option<Links>,
    pub r#type: Option<String>,
    pub total: Option<u32>,
    pub items: Option<Vec<SearchWrapper>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub token_type: Option<String>,
    pub scope: Option<String>,
    pub country: Option<String>,
    pub expires_in: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CMS {
    pub bucket: Option<String>,
    pub policy: Option<String>,
    pub signature: Option<String>,
    pub key_pair_id: Option<String>,
    pub expires: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CMSwrapper {
    pub cms: Option<CMS>,
    pub service_available: Option<bool>,
}
