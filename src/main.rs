extern crate chrono;

mod serdes;
mod log;
mod http_client;

use bytes::buf::ext::BufExt;
use serdes::Release;
use tracing_attributes::instrument;
use tracing::{debug, info};

// Metadata provided by Cargo
// See: https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates
pub static VERSION: &str = env!("CARGO_PKG_VERSION");

// Github API endpoint for CleverRaven/Cataclysm-DDA releases
pub static RELEASES_URI: &str = "https://api.github.com/repos/CleverRaven/Cataclysm-DDA/releases";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
#[instrument]
async fn main() -> Result<()> {
    log::init().expect("Unable to initialize global log subscriber");
    info!("Cataclysm Mod Manager {}", VERSION);

    let client = http_client::build_client();

    let req = http_client::github_request()
        .uri(RELEASES_URI)
        .method("GET")
        .body(hyper::Body::empty())
        .expect(&format!("Invalid URI in constant RELEASES_URI: {}", RELEASES_URI));

    let resp = client.request(req).await?;
    debug!("Response: {}", resp.status());


    let body = hyper::body::aggregate(resp).await?;  

    let releases: Vec<Release> = serde_json::from_reader(body.reader())?;

    for release in releases {
        println!("{}", release);
        println!("{}", release.body);
        for asset in release.assets {
            println!{"  {}", asset};
        }
        println!("");
    }

    Ok(())
}
