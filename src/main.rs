use openssl::ssl::{SslConnector, SslMethod};
use std::net::{TcpStream, IpAddr};
use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    let network_address = &args[1];

    let (network,cidr) = match parse_network_address(network_address) {
        Ok((n,c)) => (n,c),
        Err(e) => {
            eprintln!("{}",e);
            usage();
            exit(1);
        }
    };

    let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();

    let stream = TcpStream::connect("tenable.com:443").unwrap();
    let tls_stream = connector.connect("tenable.com", stream).unwrap();
    let ssl = tls_stream.ssl();
    let cert = ssl.peer_certificate().unwrap();
    cert.subject_name().entries().for_each(|x| println!("{:?}", x.data().as_utf8()));
}

fn usage() {
    println!("Usage: cert-scraper IP/CIDR");
}

fn parse_network_address(network_address: &str) -> std::io::Result<(IpAddr,u8)> {
    let network = match network_address.split_once('/') {
        Some(n) => n,
        None => {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "network_address required one slash."));
        }
    };

    let ip: IpAddr = match network.0.parse() {
        Ok(i) => i,
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Can't parse ip address.")),
    };


    if let Err(_) = network.1.parse::<u8>() {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "network_address required one slash."));
    };

    let cidr: u8 = match network.1.parse().unwrap() {
        n @ 0..=32 => n,
        _ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "The cidr must be between 0 and 32.")),
    };
    
    Ok((ip,cidr))
}
