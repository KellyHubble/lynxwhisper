use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use tokio::sync::mpsc::Sender;  // Explicitly use Tokio's Sender

pub fn setup_audio_input(tx: Sender<Vec<i16>>) -> cpal::Stream {
    let host = cpal::default_host();
    let device = host.default_input_device().expect("No input device available");
    let _config: cpal::StreamConfig = device.default_input_config().unwrap().into();  // Unused fix
    let stream_config = cpal::StreamConfig {
        channels: 1,
        sample_rate: cpal::SampleRate(16000),
        buffer_size: cpal::BufferSize::Default,
    };

    let stream = device.build_input_stream(
        &stream_config,
        move |data: &[i16], _: &cpal::InputCallbackInfo| {
            tx.try_send(data.to_vec()).unwrap();  // Non-blocking send
        },
        |err| eprintln!("Audio error: {}", err),
        None,
    ).unwrap();

    stream.play().unwrap();
    stream
}