use std::{
    io::Write,
    net::{SocketAddr, TcpStream},
};

fn main() -> anyhow::Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let mut stream = TcpStream::connect(addr)?;
    let buf = "Hello World!";
    stream.write_all(buf.as_bytes())?;
    Ok(())
}
