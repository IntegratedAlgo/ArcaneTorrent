use std::path::Path;

use arctor_tvdb::{MovieExplorerImpl, MediaExplorer};
use config::Config;

#[tokio::test]
async fn lookup_movies() {
    let settings = Config::builder()
    // Add in `./Settings.toml`
        .add_source(config::File::from(Path::new("/home/codingjinxx/Git/arcane_torrent_rewrite/arcane_torrent/arctor_tvdb/ApiKeys.toml")))
        .build()
        .unwrap();

    let mut explorer = MovieExplorerImpl::new();
    
    explorer.authenticate(&settings.get_string("tvdb").unwrap()).await;

    let results = explorer.query("Frozen").with_year(2013).execute().await;

    println!("{:?}", results);
}