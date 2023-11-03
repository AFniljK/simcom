use std::net::{SocketAddr, UdpSocket};

#[derive(Debug)]
struct Room {
    sender: Option<SocketAddr>,
    receiver: Option<SocketAddr>,
}

impl Room {
    fn populate(&mut self, request: Request) {
        match request {
            Request::Sender(addr) => {
                if self.sender.is_none() {
                    self.sender = Some(addr)
                }
            }
            Request::Receiver(addr) => {
                if self.receiver.is_none() {
                    self.receiver = Some(addr)
                }
            }
        }
    }

    fn is_full(&self) -> bool {
        self.sender.is_some() && self.receiver.is_some()
    }
}

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

fn main() -> anyhow::Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let socket = UdpSocket::bind(addr)?;
    let mut room = Room {
        sender: None,
        receiver: None,
    };

    loop {
        let mut buf = [0; 1024];
        let (bytes_recv, addr) = socket.recv_from(&mut buf)?;
        let request = Request::parse(&buf[0..bytes_recv], addr)?;
        room.populate(request);
        if room.is_full() {
            break;
        }
    }

    println!("{room:?}");
    Ok(())
}
