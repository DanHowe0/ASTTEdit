use semver::Version;
use serde::Deserialize;

const RELEASE_URL: &str = "https://api.github.com/repos/DanHowe0/ASTTEdit/releases/latest";
const APP_VERSION: &str = match option_env!("ASTTE_VERSION") {
    Some(version) => version,
    None => env!("CARGO_PKG_VERSION"),
};

#[derive(Deserialize)]
struct LatestRelease {
    tag_name: String,
    html_url: String,
}

fn parse_version(version: &str) -> Option<Version> {
    let version = version.trim().trim_start_matches(['v', 'V']);

    if let Ok(version) = Version::parse(version) {
        return Some(version);
    }

    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() != 2 {
        return None;
    }

    Version::parse(&format!("{}.{}.0", parts[0], parts[1])).ok()
}

pub async fn check_for_update() -> Option<(String, String)> {
    let response = reqwest::Client::new()
        .get(RELEASE_URL)
        .header(reqwest::header::USER_AGENT, "ASTTE")
        .send()
        .await
        .ok()?;

    if !response.status().is_success() {
        return None;
    }

    let release = response.json::<LatestRelease>().await.ok()?;
    let current_version = parse_version(APP_VERSION)?;
    let latest_version = parse_version(&release.tag_name)?;

    (latest_version > current_version).then_some((latest_version.to_string(), release.html_url))
}