use std::{
    io::Read,
    net::{SocketAddr, TcpListener, TcpStream},
};

use anyhow::Ok;

fn main() -> anyhow::Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr)?;
    for stream in listener.incoming() {
        handle_stream(&mut stream?)?;
    }
    Ok(())
}

fn handle_stream(stream: &mut TcpStream) -> anyhow::Result<()> {
    let mut buf = String::new();
    stream.read_to_string(&mut buf)?;
    println!("{buf:?}");
    Ok(())
}
