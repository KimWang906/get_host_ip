use clap::{Command, Arg};
use dns_message::network::host_ip::get_host_ip;

fn main() {
    //! init
    let app = Command::new("resolve")
        .about("A simple to use DNS resolver")
        .arg(Arg::new("dns-server").short('s').default_value("1.1.1.1"))
        .arg(Arg::new("domain-name").required(true))
        .get_matches();

    let domain_name_raw: &str = app
        .get_one::<String>("domain-name").map(|s| s.as_str()).unwrap();
    let dns_server_raw: &String = app
        .get_one::<String>("dns-server").unwrap();

    get_host_ip(domain_name_raw, dns_server_raw);
}
