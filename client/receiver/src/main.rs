use std::net::{SocketAddr, UdpSocket};

use anyhow::anyhow;
use cpal::{
    traits::{DeviceTrait, HostTrait},
    BufferSize, SampleRate,
};

const INITIAL_BUFFER: &str = "receiver";

fn main() -> anyhow::Result<()> {
    let server_addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let addr = SocketAddr::from(([127, 0, 0, 1], 0));
    let host = cpal::default_host();
    let speaker = host
        .default_output_device()
        .expect("cannot find output device");
    println!("Speaker: {:?}", speaker.name()?);

    let socket = UdpSocket::bind(addr)?;
    socket.connect(server_addr)?;
    socket.send(INITIAL_BUFFER.as_bytes())?;

    let mut buf = [0; 1024];
    let bytes_read = socket.recv(&mut buf)?;
    let response = String::from_utf8(buf[0..bytes_read].to_vec())?;
    if response == "occupied" {
        return Err(anyhow!(response));
    }

    let sample_rate: u32 = response.parse()?;
    let mut config = speaker.default_output_config()?.config();
    config.sample_rate = SampleRate(sample_rate);
    config.channels = 1;
    config.buffer_size = BufferSize::Fixed(1024);
    println!("Config: {config:?}");
    socket.send("listening".as_bytes())?;

    let rb = ringbuf::HeapRb::<u8>::new(1024 * 4);
    let (mut producer, _consumer) = rb.split();

    for _ in 0..1024 {
        producer.push(0).unwrap();
    }

    Ok(())
}
