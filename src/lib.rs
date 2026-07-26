mod rendezvous_server;
pub use rendezvous_server::*;
pub mod common;
mod database;
mod peer;
mod version {
    include!(concat!(env!("OUT_DIR"), "/version.rs"));
}

use hbb_common::{allow_err, get_version_number, log, tokio};

// The periodic update check is only consumed by hbbs (src/main.rs); it lives
// here in the library crate so that hbbr, which compiles `common.rs` as its
// own module, does not carry the dead code.
pub fn check_software_update() {
    const ONE_DAY_IN_SECONDS: u64 = 60 * 60 * 24;
    std::thread::spawn(move || loop {
        std::thread::spawn(move || allow_err!(check_software_update_()));
        std::thread::sleep(std::time::Duration::from_secs(ONE_DAY_IN_SECONDS));
    });
}

#[tokio::main(flavor = "current_thread")]
async fn check_software_update_() -> hbb_common::ResultType<()> {
    let (request, url) =
        hbb_common::version_check_request(hbb_common::VER_TYPE_RUSTDESK_SERVER.to_string());
    let latest_release_response = reqwest::Client::builder()
        .build()?
        .post(url)
        .json(&request)
        .send()
        .await?;

    let bytes = latest_release_response.bytes().await?;
    let resp: hbb_common::VersionCheckResponse = serde_json::from_slice(&bytes)?;
    let response_url = resp.url;
    let latest_release_version = response_url.rsplit('/').next().unwrap_or_default();
    if get_version_number(latest_release_version) > get_version_number(crate::version::VERSION) {
        log::info!("new version is available: {}", latest_release_version);
    }
    Ok(())
}
