#![feature(slice_patterns)]
#[macro_use]

extern crate serde_derive;
extern crate docopt;
extern crate notify;

mod actor;
mod stream;

use docopt::Docopt;
use std::process::exit;
use std::str::FromStr;
use std::net::UdpSocket;
use std::net;

static USAGE: &'static str = "
Usage:
    ven [--stream | --actor] -b <bind_address>
    ven [-s | -a]
    ven [-s | -a] -b <bind_address>
    ven (-h | --help)
    ven (-v | --version)

Flags:
    -s, --stream        Stream mode to stream OS events to a bound connection [default]
    -a, --actor         Actor mode for reading events form the port and triggering events
    -h, --help          Prints help information
    -v, --version       Prints version information

Options:
    -b <bind_address>       The address and port you want to send to and listen from [default: 127.0.0.1:34254]
    -r <broadcast_address>  A host socket to send from if broadcasting and receiving from the same machine. Not normally required [default: 127.0.0.1:45243]
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_s: bool,
    flag_a: bool,
    flag_v: bool,
    flag_b: Option<String>,
    flag_r: Option<String>
}

fn addr_parser(maybe_addr: Option<String>) -> net::SocketAddrV4 {
    match net::SocketAddrV4::from_str(&maybe_addr.unwrap()) {
        Ok(x) => x,
        Err(e) => {
          println!("Could not parse the provided address as a valid IPv4: {}", e);
          exit(1)
        }
    }
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let bind_addr = addr_parser(args.flag_b);
    let broadcast_addr = addr_parser(args.flag_r);

    if args.flag_v {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        exit(0);
    } else if args.flag_a {
        let socket = UdpSocket::bind(bind_addr).expect("couldn't bind to address");
        actor::listen_and_mimic(&socket);
    } else {
        let socket = UdpSocket::bind(broadcast_addr).expect("couldn't bind to address");
        stream::listen_and_broadcast(&socket, &bind_addr);
    };
}
