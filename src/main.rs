// https://tools.ietf.org/rfc/rfc5128.txt
// https://blog.csdn.net/bytxl/article/details/44344855

use clap::Arg;
use flexi_logger::*;
use hbb_common::{bail, config::RENDEZVOUS_PORT, ResultType};
use hbbs::{common::*, *};

const RMEM: usize = 0;

fn main() -> ResultType<()> {
    let _logger = Logger::try_with_env_or_str("info")?
        .log_to_stdout()
        .format(opt_format)
        .write_mode(WriteMode::Async)
        .start()?;
    let args = vec![
        Arg::new("config")
            .short('c')
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file"),
        Arg::new("bind")
            .short('b')
            .long("bind")
            .value_name("IP")
            .help("Sets the IP address to bind to (default: all interfaces)"),
        Arg::new("hbbs-port")
            .short('p')
            .long("port")
            .value_name(format!("NUMBER(default={RENDEZVOUS_PORT})"))
            .help("Sets the listening port"),
        Arg::new("serial")
            .short('s')
            .long("serial")
            .value_name("NUMBER(default=0)")
            .help("[DEPRECATED] Sets configure update serial number"),
        Arg::new("rendezvous-servers")
            .short('R')
            .long("rendezvous-servers")
            .value_name("HOSTS")
            .help("[DEPRECATED] Sets rendezvous servers, separated by comma"),
        Arg::new("relay-servers")
            .short('r')
            .long("relay-servers")
            .value_name("HOST")
            .help("Sets the default relay servers, separated by comma"),
        Arg::new("api-server")
            .long("api-server")
            .value_name("URL")
            .help("Sets the API server used by the built-in TCP API proxy"),
        Arg::new("trust-proxy-headers")
            .long("trust-proxy-headers")
            .value_name("Y/N")
            .help("Trust X-Real-IP/X-Forwarded-For on websocket listeners"),
        Arg::new("rmem")
            .short('M')
            .long("rmem")
            .value_name(format!("NUMBER(default={RMEM})"))
            .help("Sets UDP recv buffer size, set system rmem_max first, e.g., sudo sysctl -w net.core.rmem_max=52428800. vi /etc/sysctl.conf, net.core.rmem_max=52428800, sudo sysctl –p"),
        Arg::new("mask")
            .long("mask")
            .value_name("MASK")
            .help("[DEPRECATED] Determine if the connection comes from LAN, e.g. 192.168.0.0/16"),
        Arg::new("key")
            .short('k')
            .long("key")
            .value_name("KEY")
            .help("Only allow the client with the same key"),
    ];
    init_args(args, "hbbs", "RustDesk ID/Rendezvous Server")?;
    let port = get_arg_or("hbbs-port", RENDEZVOUS_PORT.to_string()).parse::<i32>()?;
    if !(3..=65_533).contains(&port) {
        bail!("Port must be between 3 and 65533");
    }
    let bind_addr = parse_bind_address(&get_arg("bind"))?;
    let rmem = get_arg_opt("rmem")
        .filter(|value| !value.trim().is_empty())
        .map(|value| value.parse::<usize>())
        .transpose()?
        .unwrap_or(RMEM);
    let serial = get_arg_or("serial", "0".to_owned()).parse::<i32>()?;
    if serial < 0 {
        bail!("Serial must not be negative");
    }
    RendezvousServer::start_with_bind(
        bind_addr,
        port,
        serial,
        &get_arg_or("key", "-".to_owned()),
        rmem,
    )?;
    Ok(())
}
