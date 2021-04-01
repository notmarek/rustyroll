// MWlhZ2ZsbjAycF9yY2R3amxzZ2E6MWl2dk85eVdubDUxTEd5N2VGTm5fdVdmMVluSUNGNEE= // Crunchyroll 3.5.0 basic api key
// My user agent "Crunchyroll/3.5.0 Android/11 okhttp/4.8.1"
mod crunchyroll;
mod models;
mod downloader;
use crunchyroll::CrunchyrollClient;
use models::*;
use downloader::download;

#[tokio::main]
async fn main() {
    let mut cr = CrunchyrollClient::setup(
        "MWlhZ2ZsbjAycF9yY2R3amxzZ2E6MWl2dk85eVdubDUxTEd5N2VGTm5fdVdmMVluSUNGNEE=".to_string(),
        "Crunchyroll/3.5.0 Android/11 okhttp/4.8.1",
        "https://beta-api.crunchyroll.com".to_string(),
        "**Removed**",
        "**Removed**",
    )
    .await;
    println!("{:#?}", cr.cms);
    // println!("{:#?}", cr.user);
    // cr.refresh().await;
    // println!("{:#?}", cr.user);
    // println!("{:#?}", cr.search("slime").await);
    println!("{:#?}", cr.get_seasons("G6P5MMXV6").await);
    // let streams: Video = cr.get_video_streams("G07FNQK95").await;
    // let subs: String = streams.subtitles.unwrap().en_us.unwrap().url.unwrap();
    // let video: String = streams.streams.unwrap().vo_adaptive_hls.unwrap().unsubbed.unwrap().url.unwrap();
    // download(&video, &subs, "1920x1080".to_string(), "Test.mkv", 15).await;
}
