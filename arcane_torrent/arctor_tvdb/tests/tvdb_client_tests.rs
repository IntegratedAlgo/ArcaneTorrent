use std::path::Path;

use config::Config;

use tvdb_client::{apis::{search_api::{get_search_results}, configuration::Configuration, login_api}, models::LoginPostRequest};

#[tokio::test]
async fn simple_search() {
    let settings = Config::builder()
    // Add in `./Settings.toml`
        .add_source(config::File::from(Path::new("/home/codingjinxx/Git/arcane_torrent_rewrite/arcane_torrent/arctor_tvdb/ApiKeys.toml")))
        .build()
        .unwrap();

    let mut config = Configuration::new();

    let bearer_token = login_api::login_post(&config, LoginPostRequest {
        apikey: settings.get_string("tvdb").unwrap(),
        pin: None
    }).await.unwrap();

    config.bearer_access_token = Some(bearer_token.data.expect("Unable to access data").token.expect("Unable to read token"));

    let results = get_search_results(&config, Some("Frozen"), None, None, None, None, None, None, Some("ENG"), None, None, None, None, None).await.unwrap();

    println!("{:?}", results);
}