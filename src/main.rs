use openssl::ssl::{SslConnector, SslMethod};
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();

    let stream = TcpStream::connect("tenable.com:443").unwrap();
    let mut stream = connector.connect("tenable.com", stream).unwrap();
    let ssl = stream.ssl();
    let cert = ssl.peer_certificate().unwrap();
    cert.subject_name().entries().for_each(|x| println!("{:?}", x.data().as_utf8()));
}
