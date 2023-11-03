use std::net::UdpSocket;

use anyhow::anyhow;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    BufferSize, Sample, SampleRate,
};
use dotenv::{dotenv, var};

const INITIAL_BUFFER: &str = "receiver";

fn main() -> anyhow::Result<()> {
    dotenv().expect("404 .env not found");

    let addr = var("ADDR").expect("404 ADDR not found");
    let server_addr = var("SERVER_ADDR").expect("404 SERVER_ADDR not found");
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

    let latency = ((150.0 / 1_000.0) * sample_rate as f32) as usize;
    let rb = ringbuf::HeapRb::<u8>::new(latency * 2);
    let (mut producer, mut consumer) = rb.split();

    for _ in 0..latency {
        producer.push(0).unwrap();
    }

    let stream = speaker.build_output_stream(
        &config,
        move |data: &mut [f32], _: &_| {
            for sample in data.iter_mut() {
                let buf = consumer.pop();
                if buf.is_none() {
                    println!("increase latency");
                    *sample = 0.0;
                } else {
                    *sample = buf.unwrap().to_sample();
                }
            }
        },
        |err| {
            println!("{err:?}");
        },
        None,
    )?;
    stream.play()?;

    loop {
        let mut buf = [0; 1024];
        let bytes_read = socket.recv(&mut buf)?;
        for sample in buf[0..bytes_read].into_iter() {
            producer.push(*sample).unwrap_or_else(|_err| {
                println!("latency needs increasing");
            });
        }
    }
}
