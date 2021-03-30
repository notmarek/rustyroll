// MWlhZ2ZsbjAycF9yY2R3amxzZ2E6MWl2dk85eVdubDUxTEd5N2VGTm5fdVdmMVluSUNGNEE= // Crunchyroll 3.5.0 basic api key
// My user agent "Crunchyroll/3.5.0 Android/11 okhttp/4.8.1"
mod crunchyroll;
use crunchyroll::CrunchyrollClient;

#[tokio::main]
async fn main() {
    let mut cr = CrunchyrollClient::setup(
        "MWlhZ2ZsbjAycF9yY2R3amxzZ2E6MWl2dk85eVdubDUxTEd5N2VGTm5fdVdmMVluSUNGNEE=".to_string(),
        "Crunchyroll/3.5.0 Android/11 okhttp/4.8.1",
        "https://beta-api.crunchyroll.com".to_string(),
        "email",
        "password",
    )
    .await;
    println!("{:#?}", cr.cms);

    println!("{:#?}", cr.user);
    cr.refresh().await;
    println!("{:#?}", cr.user);
}
