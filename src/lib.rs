use std::{fmt::Display, net::TcpStream};

use crate::packet::Packet;

pub mod packet;

pub fn connect(ip: &String, port: u16, password: &String) -> RconResult<TcpStream> {
    let uri = format!("{}:{}", ip, port);
    let mut stream = TcpStream::connect(uri.as_str()).map_err(|_| RconError::Connection)?;

    let login_packet = Packet::login(password);
    return match login_packet.send_packet(&mut stream) {
        Ok(_) => Ok(stream),
        Err(e) => Err(RconError::Communication(e)),
    };
}

pub fn command(stream: &mut TcpStream, command: &String) -> RconResult<String> {
    let command_packet = Packet::command(command);
    return match command_packet.send_packet(stream) {
        Ok(v) => Ok(v),
        Err(e) => Err(RconError::Communication(e)),
    };
}

pub type RconResult<T> = std::result::Result<T, RconError>;

pub enum RconError {
    Connection,
    Communication(std::io::Error),
}

impl Display for RconError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err = match self {
            Self::Connection => "couldn't connect to the rcon server".to_string(),
            Self::Communication(e) => format!("std::io::Error -> {}", e),
        };

        return f.write_str(err.as_str());
    }
}
