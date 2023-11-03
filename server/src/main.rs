use std::net::{SocketAddr, UdpSocket};

use anyhow::{anyhow, Ok};

enum Request {
    Sender(SocketAddr),
    Receiver(SocketAddr),
}

impl Request {
    fn parse(buf: &[u8], addr: SocketAddr) -> anyhow::Result<Request> {
        let data = String::from_utf8(buf.to_vec())?;
        if data == "sender" {
            return Ok(Request::Sender(addr));
        }
        return Ok(Request::Receiver(addr));
    }
}

#[derive(Debug)]
struct Room {
    sender: Option<SocketAddr>,
    receiver: Option<SocketAddr>,
}

impl Room {
    fn populate(&mut self, request: Request) -> anyhow::Result<()> {
        match request {
            Request::Sender(addr) => {
                if self.sender.is_some() {
                    return Err(anyhow!("sender exists"));
                }
                self.sender = Some(addr)
            }
            Request::Receiver(addr) => {
                if self.receiver.is_some() {
                    return Err(anyhow!("receiver exists"));
                }
                self.receiver = Some(addr)
            }
        }
        Ok(())
    }

    fn is_full(&self) -> bool {
        self.sender.is_some() && self.receiver.is_some()
    }
}

fn main() -> anyhow::Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let socket = UdpSocket::bind(addr)?;
    let mut room = Room {
        sender: None,
        receiver: None,
    };

    loop {
        let mut buf = [0; 1024];
        let (bytes_read, addr) = socket.recv_from(&mut buf)?;
        let request = Request::parse(&buf[0..bytes_read], addr)?;
        room.populate(request).unwrap_or_else(|err| {
            socket.send_to(err.to_string().as_bytes(), addr).unwrap();
        });
        if room.is_full() {
            break;
        }
    }

    println!("{room:?}");

    let mut buf = [0; 1024];
    socket.send_to("sample_rate".as_bytes(), room.sender.unwrap())?;
    let (bytes_read, addr) = socket.recv_from(&mut buf)?;
    if addr != room.sender.unwrap() {
        return Err(anyhow!("occupied"));
    }
    let sample_rate: u32 = String::from_utf8(buf[0..bytes_read].to_vec())?.parse()?;
    println!("sample_rate: {sample_rate:?}");

    socket.send_to(sample_rate.to_string().as_bytes(), room.receiver.unwrap())?;
    let (_, addr) = socket.recv_from(&mut [0; 1024])?;
    if addr != room.receiver.unwrap() {
        return Err(anyhow!("occupied"));
    }

    Ok(())
}
