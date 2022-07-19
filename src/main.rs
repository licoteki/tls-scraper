use ipnet::Ipv4Net;
use iprange::IpRange;
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use std::env;
use std::net::TcpStream;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        usage();
        exit(1);
    }

    let mut ip_range: IpRange<Ipv4Net> = IpRange::new();

    match args[1].parse::<Ipv4Net>() {
        Ok(n) => ip_range.add(n),
        Err(e) => {
            eprintln!("{}", e);
            usage();
            exit(1);
        }
    };

    let mut connector_builder = SslConnector::builder(SslMethod::tls()).unwrap();
    connector_builder.set_verify(SslVerifyMode::empty());
    let connector = connector_builder.build();

    for ip in ip_range.iter() {
        let stream = TcpStream::connect(format!("{}:443", ip.addr())).unwrap();
        let tls_stream = connector
            .connect(format!("{}", ip.addr()).as_str(), stream)
            .unwrap();
        let ssl = tls_stream.ssl();
        let cert = ssl.peer_certificate().unwrap();
        cert.subject_name()
            .entries()
            .for_each(|x| println!("{:?}", x.data().as_utf8().unwrap()));
    }
}

fn usage() {
    println!("Usage: cert-scraper IP/CIDR");
}
