use cataclysm_manager::{cli, http_client, log, models};

use serde_json::error::Category;
use models::github::Release;
use tracing_attributes::instrument;
use tracing::{debug, info, trace};

// All CDDA releases; usually experimental
pub static RELEASES_URI: &str = "https://api.github.com/repos/CleverRaven/Cataclysm-DDA/releases";
// Latest non-experimental CDDA releases
pub static STABLE_RELEASE_URI: &str = "https://api.github.com/repos/CleverRaven/Cataclysm-DDA/releases/latest";


#[tokio::main]
#[instrument]
async fn main() -> anyhow::Result<()> {
    log::init().expect("Unable to initialize global log subscriber");
    info!("Cataclysm Mod Manager {}", cataclysm_manager::VERSION);

    let config = cataclysm_manager::Config::default();
    debug!("{:#?}", config);

    config.user_cache.map(|p| debug!("User cache; exists: {}, location: {:?}", p.exists(), p));
    config.user_config.map(|p| debug!("User config; exists: {}, location: {:?}", p.exists(), p));
    config.user_data.map(|p| debug!("User data; exists: {}, location: {:?}", p.exists(), p));

    let args = cli::parse_arguments(std::env::args().collect());

    match args.subcommand() {
        ("releases", Some(releases_args)) => {
            match releases_args.subcommand() {
                ("list", Some(list_args)) => {
                    println!("List command called: {:#?}", list_args);

                    let uri: &str = match list_args.value_of("RELEASE_TYPE") {
                        Some("stable") => {
                            println!("Stable release.");
                            STABLE_RELEASE_URI
                        },
                        Some("experimental") => {
                            println!("Experimental release.");
                            RELEASES_URI
                        },
                        _ => unreachable!()
                    };

                    let client = http_client::build_client();
                    let releases = fetch_releases(client, uri).await?;

                    list_releases(releases);
                },
                _ => println!("Unknown subcommand called for releases: {:#?}", releases_args)
            }
        },
        _ => { println!("No subcommand was entered.") }
    }

    Ok(())
}

async fn list_releases(releases: Vec<Release>) {
    for release in releases {
        trace!("{:#?}", release);

        match release.assets.len() {
            0 => { trace!("Skipping {}, no assets", release.id ) },
            _ => {
                println!("{}: {}", release.name, release.html_url);
                for asset in release.assets {
                    println!("  {} ({}): {}", asset.name, asset.size, asset.browser_download_url);
                }
            }
        }
    }
}

async fn fetch_releases(client: http_client::Client, uri: &str) -> Result<Vec<Release>, anyhow::Error> {
    let req = http_client::github_request()
        .uri(uri)
        .method("GET")
        .body(hyper::Body::empty())
        .expect(&format!("Invalid URI in constant RELEASES_URI: {}", uri));

    let resp = client.request(req).await?;
    debug!("Response: {}", resp.status());
    debug!("Response headers: {:#?}", resp.headers());

    let bytes: bytes::Bytes = hyper::body::to_bytes(resp.into_body()).await?;

    let json = std::str::from_utf8(&bytes)?;

    let data: std::result::Result<Vec<Release>, serde_json::Error> = serde_json::from_str(json);

    match data {
        Err(e) => {
            let enriched_error =
                handle_json_deserialization_failure::<Vec<Release>>(json, e);
            let res = enriched_error.await;
            Err(res)
        },
        Ok(releases) => Ok(releases)
    }
}

/// Given a column and line of a semantic parse error in the source JSON,
/// extract an excrept from the JSON data to assist diagnosing
/// serde deserilization errors.
///
/// Returns an {anyhow::Error} with the relevant diagnostic information
/// included in the error description.
///
/// Example:
/// ```
/// async fn parsing_invalid_json () -> anyhow::Error {
///     use serde::{Deserialize, Serialize};
///
///     #[derive(Deserialize, Debug, Serialize)]
///     struct Greeting { hello: String }
///
///     let json = r#"{ "hello": 42 }"#;
///
///     match serde_json::from_str::<Greeting>(json) {
///         Ok(_) => unreachable!(), // Invalid JSON always causes an error to be returned
///         Err(json_error) => {
///             let error = handle_json_deserialization_failure::<Greeting>(
///                 json, json_error
///             ).await;
///
///             error
///         }
///     }
/// }
///
/// assert!(
///     error.description,
///     "Unable to parse release data due to semantically invalid JSON data when attempting to extract Greeting at 1:12:"
/// )
/// ```
async fn handle_json_deserialization_failure<SerdeType>(
    json: &str,
    json_error: serde_json::error::Error
) -> anyhow::Error {
    let serde_type = std::any::type_name::<SerdeType>();

    match json_error.classify() {
        Category::Io | Category::Syntax | Category::Eof => json_error.into(),
        Category::Data => {
            let line: Option<&str> = json.lines()
                .enumerate()
                .skip_while(|tuple| {
                    tuple.0 + 1 < json_error.line()
                })
                .next()
                .map(|tuple| tuple.1);

            if let Some(json_line) = line {
                let (_head, json_frag) = json_line.split_at(json_error.column());
                let enriched_error = anyhow::format_err!(
                    "Unable to parse release data due to semantically invalid JSON data when attempting to extract {} at {}:{}:\n{}",
                    json_error.line(),
                    json_error.column(),
                    serde_type,
                    json_frag
                );

                enriched_error
            } else {
                // Fall back to serde_json's default message if we aren't able to
                // find the problematic JSON fragment
                json_error.into()
            }
        }
    }
}
