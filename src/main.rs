use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use std::env;
use std::net::TcpStream;
use std::process::exit;
use std::time::Duration;
use ipaddress::IPAddress;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        usage();
        exit(1);
    }

    let ip: IPAddress = match IPAddress::parse(&args[1]) {
        Ok(ip) => ip,
        Err(e) => {
            eprintln!("Can't parse input ip address. :{}", e);
            exit(1);
        }
    };

    let mut connector_builder = SslConnector::builder(SslMethod::tls()).unwrap();
    connector_builder.set_verify(SslVerifyMode::empty());
    let connector = connector_builder.build();

    ip.each_host(|i| {
        let ip_str = i.to_s();
        print!("{}:", ip_str);
        let socket_address = format!("{}:443", ip_str).parse().unwrap();
        let stream = match TcpStream::connect_timeout(&socket_address, Duration::new(2, 0)) {
            Ok(s) => s,
            Err(_) => {
                println!("Failed estabilish connection using tcp.");
                return;
            }
        };

        let tls_stream = match connector.connect(&i.to_s().as_str(), stream) {
            Ok(t) => t,
            Err(_) => {
                println!("Failed estabilish connection using tls.");
                return;
            }
        };

        let ssl = tls_stream.ssl();
        let cert = ssl.peer_certificate().unwrap();
        cert.subject_name()
            .entries()
            .for_each(|x| println!("{:?}", x.data().as_utf8().unwrap()));
    });
}

fn usage() {
    println!("Usage: cert-scraper IP/CIDR");
}
