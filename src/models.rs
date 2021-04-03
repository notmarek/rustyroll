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
pub struct Me {
    pub account_id: Option<String>,
    pub external_id: Option<String>,
    pub email_verified: Option<bool>,
    pub created: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Benefit {
    #[serde(rename = "__actions__")]
    pub actions: Option<PlaceholderEmpty>,
    #[serde(rename = "__class__")]
    pub class: Option<String>,
    #[serde(rename = "__href__")]
    pub href: Option<String>,
    #[serde(rename = "__links__")]
    pub links: Option<Links>,
    pub benefit: Option<String>,
    pub source: Option<String>,
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
pub struct HREF {
    pub href: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Links {
    pub continuation: Option<HREF>,
    pub resource: Option<HREF>,
    #[serde(rename = "resource/channel")]
    pub resource_channel: Option<HREF>,
    #[serde(rename = "series/channel")]
    pub series_channel: Option<HREF>,
    #[serde(rename = "series/season")]
    pub series_season: Option<HREF>,
    #[serde(rename = "episode/season")]
    pub episode_season: Option<HREF>,
    #[serde(rename = "episode/series")]
    pub episode_series: Option<HREF>,
    #[serde(rename = "episode/channel")]
    pub episode_channel: Option<HREF>,
    #[serde(rename = "episode/next_episode")]
    pub episode_next_episode: Option<HREF>,
    #[serde(rename = "season/channel")]
    pub season_channel: Option<HREF>,
    #[serde(rename = "season/episodes")]
    pub season_episodes: Option<HREF>,
    #[serde(rename = "season/series")]
    pub season_series: Option<HREF>,
    pub streams: Option<HREF>,
    pub ads: Option<HREF>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Images {
    pub poster_tall: Option<Vec<Vec<Image>>>,
    pub poster_wide: Option<Vec<Vec<Image>>>,
    pub promo_image: Option<Vec<Vec<Image>>>,
    pub thumbnail: Option<Vec<Vec<Image>>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaceholderEmpty {}

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
pub struct Wrapper<T> {
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
    pub items: Option<Vec<T>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Season {
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
    pub id: Option<String>,
    pub channel_id: Option<String>,
    pub title: Option<String>,
    pub series_id: Option<String>,
    pub season_number: Option<u32>,
    pub is_complete: Option<bool>,
    pub description: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub season_tags: Option<Vec<String>>,
    pub images: Option<Images>,
    pub is_mature: Option<bool>,
    pub mature_blocked: Option<bool>,
    pub is_subbed: Option<bool>,
    pub is_dubbed: Option<bool>,
    pub is_simulcast: Option<bool>,
    pub seo_title: Option<String>,
    pub seo_description: Option<String>,
    pub availability_notes: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Episode {
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
    pub id: Option<String>,
    pub channel_id: Option<String>,
    pub title: Option<String>,
    pub series_id: Option<String>,
    pub season_number: Option<u32>,
    pub season_title: Option<String>,
    pub series_title: Option<String>,
    pub episode: Option<String>,
    pub episode_number: Option<u32>,
    pub sequence_number: Option<u32>,
    pub production_episode_id: Option<String>,
    pub description: Option<String>,
    pub next_episode_id: Option<String>,
    pub next_episode_title: Option<String>,
    pub hd_flag: Option<bool>,
    pub is_mature: Option<bool>,
    pub mature_blocked: Option<bool>,
    pub episode_air_date: Option<String>,
    pub is_subbed: Option<bool>,
    pub is_dubbed: Option<bool>,
    pub is_clip: Option<bool>,
    pub seo_title: Option<String>,
    pub seo_description: Option<String>,
    pub season_tags: Option<Vec<String>>,
    pub available_offline: Option<bool>,
    pub media_type: Option<String>,
    pub slug: Option<String>,
    pub images: Option<Images>,
    pub duration_ms: Option<u64>,
    pub ad_breaks: Option<Vec<AdBreak>>,
    pub is_premium_only: Option<bool>,
    pub listing_id: Option<String>,
    pub subtitle_locales: Option<Vec<String>>,
    pub playback: Option<String>,
    pub availability_notes: Option<String>,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Series {
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
    pub id: Option<String>,
    pub channel_id: Option<String>,
    pub title: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub season_tags: Option<Vec<String>>,
    pub images: Option<Images>,
    pub maturity_ratings: Option<Vec<String>>,
    pub episode_count: Option<u32>,
    pub season_count: Option<u32>,
    pub media_count: Option<u32>,
    pub content_provide: Option<String>,
    pub is_mature: Option<bool>,
    pub mature_blocked: Option<bool>,
    pub is_subbed: Option<bool>,
    pub is_dubbed: Option<bool>,
    pub is_simulcast: Option<bool>,
    pub seo_title: Option<String>,
    pub seo_description: Option<String>,
    pub availability_notes: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subtitle {
    pub locale: Option<String>,
    pub url: Option<String>,
    pub format: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subtitles {
    #[serde(rename = "ar-ME")]
    pub ar_me: Option<Subtitle>,
    #[serde(rename = "de-DE")]
    pub de_de: Option<Subtitle>,
    #[serde(rename = "en-US")]
    pub en_us: Option<Subtitle>,
    #[serde(rename = "es-ES")]
    pub es_es: Option<Subtitle>,
    #[serde(rename = "es-LA")]
    pub es_la: Option<Subtitle>,
    #[serde(rename = "fr-FR")]
    pub fr_fr: Option<Subtitle>,
    #[serde(rename = "it-IT")]
    pub it_it: Option<Subtitle>,
    #[serde(rename = "pt-BR")]
    pub pt_br: Option<Subtitle>,
    #[serde(rename = "ru-RU")]
    pub ru_ru: Option<Subtitle>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stream {
    pub hardsub_locale: Option<String>,
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StreamLang {
    #[serde(rename = "")]
    pub unsubbed: Option<Stream>,
    #[serde(rename = "ar-ME")]
    pub ar_me: Option<Stream>,
    #[serde(rename = "de-DE")]
    pub de_de: Option<Stream>,
    #[serde(rename = "en-US")]
    pub en_us: Option<Stream>,
    #[serde(rename = "es-ES")]
    pub es_es: Option<Stream>,
    #[serde(rename = "es-LA")]
    pub es_la: Option<Stream>,
    #[serde(rename = "fr-FR")]
    pub fr_fr: Option<Stream>,
    #[serde(rename = "it-IT")]
    pub it_it: Option<Stream>,
    #[serde(rename = "pt-BR")]
    pub pt_br: Option<Stream>,
    #[serde(rename = "ru-RU")]
    pub ru_ru: Option<Stream>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Streams {
    pub adaptive_dash: Option<StreamLang>,
    pub adaptive_hls: Option<StreamLang>,
    pub download_hls: Option<StreamLang>,
    pub drm_adaptive_dash: Option<StreamLang>,
    pub drm_adaptive_hls: Option<StreamLang>,
    pub drm_download_hls: Option<StreamLang>,
    pub drm_multitrack_adaptive_hls_v2: Option<StreamLang>,
    pub multitrack_adaptive_hls_v2: Option<StreamLang>,
    pub urls: Option<StreamLang>,
    pub vo_adaptive_dash: Option<StreamLang>,
    pub vo_adaptive_hls: Option<StreamLang>,
    pub vo_drm_adaptive_dash: Option<StreamLang>,
    pub vo_drm_adaptive_hls: Option<StreamLang>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Video {
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
    pub media_id: Option<String>,
    pub audio_locales: Option<String>,
    pub subtitles: Option<Subtitles>,
    pub captions: Option<PlaceholderEmpty>,
    pub streams: Option<Streams>,
    pub bifs: Option<Vec<String>>,
}
