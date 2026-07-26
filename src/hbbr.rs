use clap::{Arg, Command};
mod common;
mod relay_server;
use flexi_logger::*;
use hbb_common::{config::RELAY_PORT, ResultType};
use relay_server::*;
mod version {
    include!(concat!(env!("OUT_DIR"), "/version.rs"));
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
    if let Ok(v) = ini::Ini::load_from_file(".env") {
        if let Some(section) = v.section(None::<String>) {
            section.iter().for_each(|(k, v)| common::set_arg(k, v));
        }
    }
    let mut port = RELAY_PORT;
    if let Some(v) = common::get_arg_opt("PORT") {
        let v: i32 = v.parse().unwrap_or_default();
        if v > 0 {
            port = v + 1;
        }
    }
    let default_port = port.to_string();
    let bind = matches
        .get_one::<String>("bind")
        .map(String::to_owned)
        .unwrap_or_else(|| common::get_arg("BIND"));
    let bind_addr = common::parse_bind_address(&bind)?;
    let key = matches
        .get_one::<String>("key")
        .map(String::to_owned)
        .unwrap_or_else(|| common::get_arg("KEY"));
    let trust_proxy_headers = matches
        .get_one::<String>("trust-proxy-headers")
        .map(String::to_owned)
        .unwrap_or_else(|| common::get_arg("TRUST_PROXY_HEADERS"));
    start_with_bind(
        bind_addr,
        matches
            .get_one::<String>("port")
            .map(String::as_str)
            .unwrap_or(&default_port),
        &key,
        trust_proxy_headers.eq_ignore_ascii_case("Y"),
    )?;
    Ok(())
}
