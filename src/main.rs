extern crate portaudio;

use portaudio::PortAudio;
use std::collections::VecDeque;

fn wait_for_stream<F>(f: F, name: &str) -> u32
    where F: Fn() -> Result<portaudio::StreamAvailable, portaudio::error::Error> {
    loop {
        match f() {
            Ok(available) => match available {
                portaudio::StreamAvailable::Frames(frames) => return frames as u32,
                portaudio::StreamAvailable::InputOverflowed => println!("Input stream has overflowed"),
                portaudio::StreamAvailable::OutputUnderflowed => println!("Output stream has underflowed"),
            },
            Err(err) => panic!("An error occured while waiting for {} stream: {}", name, err),
        }
    }
}

fn main() {
    let pa = PortAudio::new().unwrap();
    let dev_indx = pa.default_input_device().unwrap();
    let dev_info = pa.device_info(dev_indx).unwrap();
    let dev_latency = dev_info.default_low_input_latency;
    let inp_params = portaudio::StreamParameters::<f32>::new(dev_indx, 2, true, dev_latency);
    let devo_indx = pa.default_output_device().unwrap();
    let devo_info = pa.device_info(devo_indx).unwrap();
    let devo_latency = devo_info.default_low_output_latency;
    let out_params = portaudio::StreamParameters::<f32>::new(devo_indx, 2, true, devo_latency);
    let settings = portaudio::DuplexStreamSettings::new(inp_params, out_params, 44_100.0, 256);
    let portaudio::DeviceIndex(index) = dev_indx;
    let mut stream = pa.open_blocking_stream(settings).unwrap();
    let mut buffer: VecDeque<f32> = VecDeque::with_capacity(512);

    println!("Number of devices: {}", pa.device_count().unwrap());
    println!("Default device index: {}", index);
    println!("Default device information: {:?}", dev_info);

    stream.start().unwrap();
    loop{
        let in_frames = wait_for_stream(|| stream.read_available(), "Read");

        if in_frames > 0 {

            let input_samples = stream.read(in_frames).unwrap();
            buffer.extend(input_samples.into_iter());
            println!("Read {:?} frames from the input stream", in_frames);
        }

        let out_frames = wait_for_stream(|| stream.write_available(), "Write");

        let buffer_frames = (buffer.len() / 2) as u32;

        if out_frames > 0 && buffer_frames > 0 {
            let write_frames = if buffer_frames >= out_frames {out_frames} else {buffer_frames};
            let n_write_samples = (write_frames * 2) as usize;

            stream.write(write_frames, |output| {
                for i in 0..n_write_samples {
                    output[i] = buffer.pop_front().unwrap();
                }
                println!("Write {:?} frames to the output stream", out_frames);
            }).unwrap();
        }
    }
}