use std::net::{SocketAddr, UdpSocket};

const INITIAL_BUFFER: &str = "receiver";

fn main() -> anyhow::Result<()> {
    let server_addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let addr = SocketAddr::from(([127, 0, 0, 1], 0));

    let socket = UdpSocket::bind(addr)?;
    socket.send_to(INITIAL_BUFFER.as_bytes(), server_addr)?;
    let mut buf = [0; 1024];
    let (bytes_read, _) = socket.recv_from(&mut buf)?;
    println!("{:?}", String::from_utf8(buf[0..bytes_read].to_vec()));
    Ok(())
}
