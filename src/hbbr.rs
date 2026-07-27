use clap::{Arg, Command};
mod common;
mod relay_server;
use flexi_logger::*;
use hbb_common::{config::RELAY_PORT, ResultType};
use relay_server::*;
use std::path::Path;
mod version {
    pub const VERSION: &str = env!("CARGO_PKG_VERSION");
}

fn main() -> ResultType<()> {
    let _logger = Logger::try_with_env_or_str("info")?
        .log_to_stdout()
        .format(opt_format)
        .write_mode(WriteMode::Async)
        .start()?;
    let matches = Command::new("hbbr")
        .version(version::VERSION)
        .author("Purslane Ltd. <info@rustdesk.com>")
        .about("RustDesk Relay Server")
        .arg(
            Arg::new("bind")
                .short('b')
                .long("bind")
                .value_name("IP")
                .help("Sets the IP address to bind to (default: all interfaces)"),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .value_name(format!("NUMBER(default={RELAY_PORT})"))
                .help("Sets the listening port"),
        )
        .arg(
            Arg::new("key")
                .short('k')
                .long("key")
                .value_name("KEY")
                .help("Only allow the client with the same key"),
        )
        .arg(
            Arg::new("trust-proxy-headers")
                .long("trust-proxy-headers")
                .value_name("Y/N")
                .help("Trust X-Real-IP/X-Forwarded-For on websocket listeners"),
        )
        .get_matches();
    let default_path = Path::new(".env");
    common::load_arg_file_if_present(default_path)?;
    let default_port = common::get_arg_or("HBBR_PORT", RELAY_PORT.to_string());
    let bind = matches
        .get_one::<String>("bind")
        .map(String::to_owned)
        .unwrap_or_else(|| common::get_arg("BIND"));
    let bind_addr = common::parse_bind_address(&bind)?;
    let key = matches
        .get_one::<String>("key")
        .map(String::to_owned)
        .unwrap_or_else(|| common::get_arg("KEY"));
    let trust_proxy_headers = match matches.get_one::<String>("trust-proxy-headers") {
        Some(value) => common::parse_yes_no("trust-proxy-headers", value)?,
        None => common::get_yes_no_arg("TRUST_PROXY_HEADERS", false)?,
    };
    start_with_bind(
        bind_addr,
        matches
            .get_one::<String>("port")
            .map(String::as_str)
            .unwrap_or(&default_port),
        &key,
        trust_proxy_headers,
    )?;
    Ok(())
}
