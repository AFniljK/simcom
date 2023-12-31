use std::net::UdpSocket;

use anyhow::anyhow;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    BufferSize, Sample,
};
use dotenv::{dotenv, var};

const INITIAL_BUFFER: &str = "sender";

fn main() -> anyhow::Result<()> {
    dotenv().expect("404 .env not found");

    let addr = var("ADDR").expect("404 ADDR not found");
    let server_addr = var("SERVER_ADDR").expect("404 SERVER_ADDR not found");
    let host = cpal::default_host();
    let microphone = host
        .default_input_device()
        .expect("cannot find input device");
    println!("Microphone: {}", microphone.name()?);

    let socket = UdpSocket::bind(addr)?;
    socket.connect(server_addr)?;
    socket.send(INITIAL_BUFFER.as_bytes())?;

    let mut buf = [0; 1024];
    let bytes_read = socket.recv(&mut buf)?;
    let response = String::from_utf8(buf[0..bytes_read].to_vec())?;
    if response == "occupied" {
        return Err(anyhow!(response));
    }

    let mut config = microphone.default_input_config()?.config();
    config.channels = 1;
    config.buffer_size = BufferSize::Fixed(1024);
    println!("Config: {config:?}");
    socket.send(config.sample_rate.0.to_string().as_bytes())?;

    let stream = microphone.build_input_stream(
        &config,
        move |data: &[f32], _: &_| {
            let data: Vec<u8> = data.into_iter().map(|sample| sample.to_sample()).collect();
            socket.try_clone().unwrap().send(data.as_slice()).unwrap();
        },
        |err| {
            println!("{err:?}");
        },
        None,
    )?;
    stream.play()?;

    loop {}
}
