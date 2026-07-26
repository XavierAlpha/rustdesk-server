use std::{env, fs, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH");

    let cargo_toml = fs::read_to_string("Cargo.toml").expect("failed to read Cargo.toml");
    let version = cargo_toml
        .lines()
        .map(str::trim)
        .find_map(|line| line.strip_prefix("version = "))
        .expect("failed to find package version in Cargo.toml");

    // Deterministic build date: honor SOURCE_DATE_EPOCH (reproducible builds),
    // otherwise fall back to the current UTC time.
    let build_date = env::var("SOURCE_DATE_EPOCH")
        .ok()
        .and_then(|epoch| epoch.parse::<i64>().ok())
        .and_then(|secs| chrono::DateTime::<chrono::Utc>::from_timestamp(secs, 0))
        .unwrap_or_else(chrono::Utc::now)
        .format("%Y-%m-%d %H:%M")
        .to_string();

    // Write generated build metadata into OUT_DIR so cross can keep the
    // project source mounted read-only inside the container.
    let version_rs =
        PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is not set")).join("version.rs");
    fs::write(
        version_rs,
        format!(
            "pub const VERSION: &str = {version};\n#[allow(dead_code)]\npub const BUILD_DATE: &str = \"{build_date}\";\n"
        ),
    )
    .expect("failed to write generated version.rs");
}
