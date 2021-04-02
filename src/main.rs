// MWlhZ2ZsbjAycF9yY2R3amxzZ2E6MWl2dk85eVdubDUxTEd5N2VGTm5fdVdmMVluSUNGNEE= // Crunchyroll 3.5.0 basic api key
// My user agent "Crunchyroll/3.5.0 Android/11 okhttp/4.8.1"
mod crunchyroll;
mod models;
mod downloader;
use crunchyroll::CrunchyrollClient;
use models::*;
use downloader::download;
use std::env;

#[tokio::main]
async fn main() {
    let mut email;
    let mut password;
    match env::var("CR_EMAIL") {
        Ok(val) => email = val,
        Err(_e) => email = "none".to_string(),
    }
    match env::var("CR_PASS") {
        Ok(val) => password = val,
        Err(_e) => password = "none".to_string(),
    }
    let mut cr = CrunchyrollClient::setup(
        "MWlhZ2ZsbjAycF9yY2R3amxzZ2E6MWl2dk85eVdubDUxTEd5N2VGTm5fdVdmMVluSUNGNEE=".to_string(),
        "Crunchyroll/3.5.0 Android/11 okhttp/4.8.1",
        "https://beta-api.crunchyroll.com".to_string(),
        &email,
        &password,
    )
    .await;
    // println!("{:#?}", cr.cms);
    // println!("{:#?}", cr.user);
    // cr.refresh().await;
    // println!("{:#?}", cr.user);
    // println!("{:#?}", cr.search("slime").await);
    // println!("{:#?}", cr.get_seasons("G6P5MMXV6").await);
    //GR24GGE06
    let episodes: Wrapper<Episode> = cr.get_episodes("GR24GGE06").await;
    // println!("{:#?}", episodes);
    for x in 0..episodes.items.as_ref().unwrap().len() {
        let episode_id: String = episodes.items.as_ref().unwrap()[x].id.as_ref().unwrap().clone();
        let episode: Episode = cr.get_episode(&episode_id).await;
        let media_link: String = episode.links.unwrap().streams.unwrap().href.unwrap();
        let streams: Video = cr.get_video_streams_by_link(&media_link).await;
        let subs: String = streams.subtitles.unwrap().en_us.unwrap().url.unwrap();
        let video: String = streams.streams.unwrap().vo_adaptive_hls.unwrap().unsubbed.unwrap().url.unwrap();
        println!("{:#?}", video);
        download(&video, &subs, "1920x1080".to_string(), &format!("[Rustyroll] {title} #{episode} (1080p).mkv", title=episode.series_title.unwrap(), episode=episode.episode.unwrap()), 15).await;
    }
}
