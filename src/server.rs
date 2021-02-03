use super::config::Config;
use super::dbus::Conn;
use super::device::Data;
use super::Result;
use std::str::FromStr;
use tiny_http::{Header, Method, Response, Server};

const PLAIN: &str = "Content-Type: text/plain";
const JSON: &str = "Content-Type: application/json";

pub fn run(conf: &Config) -> Result<()> {
    let server = Server::http((conf.ip.as_str(), conf.port))?;
    let conn = Conn::new(conf)?;
    let mut json = Vec::new();
    println!("Listening on {}:{}", conf.ip, conf.port);

    for req in server.incoming_requests() {
        let name = req.url().trim_matches('/');
        let address = conf.devices.get(name);
        match (req.method(), address) {
            (Method::Get, Some(addr)) => {
                Data::new(name, addr, &conn).to_json(&mut json)?;
                let res = Response::from_data(json.as_slice())
                    .with_header(Header::from_str(JSON).unwrap());
                req.respond(res)?;
                json.clear();
            }
            (Method::Get, None) => {
                let res = Response::from_data(&b"Not Found"[..])
                    .with_status_code(404)
                    .with_header(Header::from_str(PLAIN).unwrap());
                req.respond(res)?;
            }
            _ => {
                let res = Response::from_data(&b"Method Not Allowed"[..])
                    .with_status_code(405)
                    .with_header(Header::from_str(PLAIN).unwrap());
                req.respond(res)?;
            }
        }
    }
    Ok(())
}
