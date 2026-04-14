use std::{i32, io::{ErrorKind, Read, Write}, net::TcpStream, vec,};

use rand::Rng;

const HEADER_LEN: usize = 8;
const FOOTER_LEN: usize = 2;

const SERVERDATA_AUTH : i32 = 3;
const SERVERDATA_AUTH_RESPONSE: i32 = 2;
const SERVERDATA_EXECCOMMAND: i32 = 2;
const SERVERDATA_RESPONSE_VALUE: i32 = 0;

pub struct Packet {
    pub id: i32,
    pub packet_type: i32,
    pub body: String,
}

impl Packet {
    pub fn login(password: &String) -> Self {
        return Self {
            id: rand::rng().random_range(1..i32::MAX),
            packet_type: SERVERDATA_AUTH,
            body: password.clone(),
        };
    }

    pub fn command(command: &String) -> Self {
        return Self {
            id: rand::rng().random_range(1..i32::MAX),
            packet_type: SERVERDATA_EXECCOMMAND,
            body: command.clone(),
        };
    }

    fn read(&self, stream: &mut TcpStream) -> std::io::Result<String> {
        let mut response_len: [u8; 4] = [0; 4];
        let mut response_id: [u8; 4] = [0; 4];
        let mut response_type: [u8; 4] = [0; 4];

        stream.read_exact(&mut response_len)?;
        stream.read_exact(&mut response_id)?;
        stream.read_exact(&mut response_type)?;

        let payload_len = i32::from_le_bytes(response_len) as usize - HEADER_LEN - FOOTER_LEN;

        let mut buffer = vec![0; payload_len];
        stream.read_exact(&mut buffer)?;
        stream.read_exact(&mut [0; 2])?;

        match i32::from_le_bytes(response_type) {
            SERVERDATA_RESPONSE_VALUE => {
                let response_body = String::from_utf8(buffer).map_err(|_| {
                    std::io::Error::new(
                        ErrorKind::Other,
                        "response payload contained invalid utf-8 characters"
                    )
                })?;

                if i32::from_le_bytes(response_id) == self.id {
                    return Ok(response_body);
                }
            },
            SERVERDATA_AUTH_RESPONSE => {
                return Ok("logged in".to_string());
            }
            _ => (),
        };

        let error = std::io::Error::new(ErrorKind::Other, "invalid response type");
        return Err(error);
    }

    fn write(&self, stream: &mut TcpStream) -> std::io::Result<()> {
        let request_len = (HEADER_LEN + self.body.len() + 1 + FOOTER_LEN) as i32;

        if request_len > 4088 {
            let error = std::io::Error::new(ErrorKind::Other, "request is too big");
            return Err(error);
        }

        let mut request: Vec<u8> = Vec::new();

        request.write_all(&request_len.to_le_bytes())?;
        request.write_all(&self.id.to_le_bytes())?;
        request.write_all(&self.packet_type.to_le_bytes())?;
        request.write_all(&self.body.as_bytes())?;
        request.write_all(b"\0\0\0")?;

        stream.write_all(&request)?;

        return Ok(());
    }

    pub fn send_packet(&self, stream: &mut TcpStream) -> std::io::Result<String> {
        self.write(stream)?;

        return self.read(stream);
    }
}
