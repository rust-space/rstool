use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::time::Duration;

use regex::Regex;

use trust_dns::op::{Message, MessageType, OpCode, Query};
use trust_dns::proto::error::ProtoError;
use trust_dns::rr::domain::Name;
use trust_dns::rr::record_type::RecordType;
use trust_dns::serialize::binary::*;

const DNS_SERVER: &'static str = "114.114.114.114:53";
const IP_SERVER: &'static str = "https://myip.ipip.net";

#[derive(Debug)]
pub enum IpError {
    Request(reqwest::Error),
    RegexIpAddr,
    ParseDomainName(ProtoError),
    ParseDnsServerAddress(std::net::AddrParseError),
    Encoding(ProtoError),
    Decoding(ProtoError),
    Network(std::io::Error),
    Sending(std::io::Error),
    Receving(std::io::Error),
}

impl std::fmt::Display for IpError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl From<reqwest::Error> for IpError {
    fn from(error: reqwest::Error) -> Self {
        IpError::Request(error)
    }
}

impl From<std::net::AddrParseError> for IpError {
    fn from(error: std::net::AddrParseError) -> Self {
        IpError::ParseDnsServerAddress(error)
    }
}

impl std::error::Error for IpError {}

pub fn get_local_ip() -> Result<IpAddr, IpError> {
    let socket = UdpSocket::bind("0.0.0.0:0").map_err(IpError::Network)?;
    socket.connect(self::DNS_SERVER).map_err(IpError::Network)?;
    let local_addr = socket.local_addr().map_err(IpError::Network)?;
    match local_addr {
        SocketAddr::V4(addr) => Ok(IpAddr::V4(*addr.ip())),
        SocketAddr::V6(addr) => Ok(IpAddr::V6(*addr.ip())),
    }
}

pub fn get_public_ip() -> Result<String, IpError> {
    let body = reqwest::blocking::get(self::IP_SERVER)?.text()?;

    let re = Regex::new(r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}").unwrap();
    if let Some(ip) = re.find(body.as_str()) {
        Ok(ip.as_str().to_owned())
    } else {
        Err(IpError::RegexIpAddr)
    }
}

pub fn resolve(domain_name: &str) -> Result<Option<std::net::IpAddr>, Box<dyn std::error::Error>> {
    let domain_name = Name::from_ascii(domain_name).map_err(IpError::ParseDomainName)?;

    let dns_server: SocketAddr = self::DNS_SERVER.parse()?;

    let mut request_buffer: Vec<u8> = Vec::with_capacity(64);
    let mut response_buffer: Vec<u8> = vec![0; 512];

    let mut request = Message::new();
    request.add_query(Query::query(domain_name, RecordType::A));

    request
        .set_id(message_id())
        .set_message_type(MessageType::Query)
        .set_op_code(OpCode::Query)
        .set_recursion_desired(true);

    let socket = UdpSocket::bind("0.0.0.0:0").map_err(IpError::Network)?;

    let timeout = Duration::from_secs(5);
    socket
        .set_read_timeout(Some(timeout))
        .map_err(IpError::Network)?;

    socket.set_nonblocking(false).map_err(IpError::Network)?;

    let mut encoder = BinEncoder::new(&mut request_buffer);
    request.emit(&mut encoder).map_err(IpError::Encoding)?;

    let _n_bytes_sent = socket
        .send_to(&request_buffer, dns_server)
        .map_err(IpError::Sending)?;

    loop {
        let (_b_bytes_recv, remote_port) = socket
            .recv_from(&mut response_buffer)
            .map_err(IpError::Receving)?;

        if remote_port == dns_server {
            break;
        }
    }

    let response = Message::from_vec(&response_buffer).map_err(IpError::Decoding)?;

    for answer in response.answers() {
        if answer.record_type() == RecordType::A {
            let resource = answer.rdata();
            let server_ip = resource.to_ip_addr().expect("invalid IP address received");
            return Ok(Some(server_ip));
        }
    }

    Ok(None)
}

fn message_id() -> u16 {
    let candidate = rand::random();
    if candidate == 0 {
        return message_id();
    }
    candidate
}
