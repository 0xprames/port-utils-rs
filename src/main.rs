use clap::{arg, command, value_parser};
use std::{error::Error, net::TcpListener};

fn find_free_port() -> Result<u16, Box<dyn Error>> {
    println!("Finding free port");
    Ok(TcpListener::bind(("127.0.0.1", 0))?.local_addr()?.port())
}

fn check_port_free(port: u16) {
    match TcpListener::bind(("127.0.0.1", port)) {
        Ok(_) => println!("Port {port} is available!"),
        Err(_) => println!("Port {port} is unavailable!"),
    }
}

fn main() {
    let matches = command!()
        .arg(
            arg!([port] "port arg to check availability - if no arg, first free port is returned")
                .value_parser(value_parser!(u16)),
        )
        .get_matches();

    if let Some(port) = matches.get_one::<u16>("port") {
        check_port_free(*port)
    } else {
        let port = find_free_port().unwrap();
        println!("Available tcp port is {port}");
    }
}
