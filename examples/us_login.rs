use rustyroll::CrunchyrollClient;
use std::env;

#[tokio::main]
async fn main() {
    let email;
    let password;
    match env::var("CR_EMAIL") {
        Ok(val) => email = val,
        Err(_e) => email = "".to_string(),
    }
    match env::var("CR_PASS") {
        Ok(val) => password = val,
        Err(_e) => password = "".to_string(),
    }
    let mut cr = CrunchyrollClient::setup_with_custom_login_url(
        "MWlhZ2ZsbjAycF9yY2R3amxzZ2E6MWl2dk85eVdubDUxTEd5N2VGTm5fdVdmMVluSUNGNEE=".to_string(),
        "Crunchyroll/3.5.0 Android/11 okhttp/4.8.1",
        "https://beta-api.crunchyroll.com".to_string(),
        &email,
        &password,
        "https://rustyroll.herokuapp.com"
    )
    .await;
    println!("{:#?}", cr.user);
    println!("{:#?}", cr.cms);
    cr.close().await;
}
