use std::{
    io::Write,
    net::{SocketAddr, TcpStream},
};

const INITIAL_BUFFER: &str = "receiver";

fn main() -> anyhow::Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let mut stream = TcpStream::connect(addr)?;
    stream.write_all(INITIAL_BUFFER.as_bytes())?;
    Ok(())
}
