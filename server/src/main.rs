use std::{
    io::Read,
    net::{SocketAddr, TcpListener, TcpStream},
};

#[derive(Debug)]
struct Room {
    sender: Option<SocketAddr>,
    receiver: Option<SocketAddr>,
}

impl Room {
    fn populate_with(&mut self, request: Request) {
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

fn main() -> anyhow::Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr)?;
    let mut room = Room {
        sender: None,
        receiver: None,
    };
    for stream in listener.incoming() {
        let request = handle_stream(&mut stream?)?;
        room.populate_with(request);
        if room.is_full() {
            break;
        }
    }
    println!("{room:?}");
    Ok(())
}

fn handle_stream(stream: &mut TcpStream) -> anyhow::Result<Request> {
    let mut buf = String::new();
    stream.read_to_string(&mut buf)?;
    let addr = stream.peer_addr()?;
    if buf == "sender" {
        return Ok(Request::Sender(addr));
    }
    Ok(Request::Receiver(addr))
}
