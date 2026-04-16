/*
Copyright (C) 2026 leoflo

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::{io::{self, Write}, process::exit};

use clap::{arg, value_parser, Command};

fn cli() -> Command {
    return Command::new("Rust rcon")
        .bin_name("rrcon")
        .about("Simple rcon client")
        .arg(
            arg!(--ip [address] "The ip address of the rcon server")
                .value_parser(value_parser!(String))
                .required(false)
        )
        .arg(
            arg!(--port [port] "The port of the rcon server")
                .value_parser(value_parser!(u16))
                .required(false)
        )
        .arg(
            arg!(<password> "The rcon password")
                .value_parser(value_parser!(String))
                .required(true)
        );
}

fn prompt() -> std::io::Result<String> {
    let mut buffer = String::new();

    print!("> ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut buffer)?;

    return Ok(buffer);
}

fn main() {
    let matches = cli().get_matches();

    let ip = match matches.get_one::<String>("ip") {
        Some(v) => v.clone(),
        None => "127.0.0.1".to_string(),
    };

    let port = match matches.get_one::<u16>("port") {
        Some(v) => v.clone(),
        None => 25575,
    };

    let password = matches.get_one::<String>("password")
        .expect("Error: the password is needed in order to connect");

    let mut stream = match rcon_client::connect(&ip, port, password) {
        Ok(v) => {
            println!("Connected :O");
            v
        },
        Err(e) => {
            println!("Error: {}", e);
            exit(0x01);
        },
    };

    loop {
        let mut command = match prompt() {
            Ok(v) => v,
            Err(e) => {
                println!("Error: {}", e);
                exit(0x01);
            },
        };

        command.pop();

        match command.as_str() {
            "exit" => {
                println!("Bye bye :D");
                exit(0x00);
            },
            _ => {
                match rcon_client::command(&mut stream, &command) {
                    Ok(v) => println!("{}", v),
                    Err(e) => println!("Error: {}", e),
                };
            },
        };
    }
}
