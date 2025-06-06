use assert_no_alloc::*;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, SizedSample, Stream};
use fundsp::hacker::*;

#[cfg(debug_assertions)] // required when disable_release is set (default)
#[global_allocator]
static A: AllocDisabler = AllocDisabler;


pub fn init_snd() -> (Stream, Net) {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .expect("Failed to find a default output device");
    let config = device.default_output_config().unwrap();

    match config.sample_format() {
        cpal::SampleFormat::F32 => start_stream::<f32>(&device, &config.into()).unwrap(),
        cpal::SampleFormat::I16 => start_stream::<i16>(&device, &config.into()).unwrap(),
        cpal::SampleFormat::U16 => start_stream::<u16>(&device, &config.into()).unwrap(),
        _ => panic!("Unsupported format"),
    }
}


fn start_stream<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> anyhow::Result<(Stream, Net)>
where
    T: SizedSample + FromSample<f32>,
{
    let sample_rate = config.sample_rate.0 as f64;
    let channels = config.channels as usize;

    let mut net = Net::new(0, 2);
    net.check();
    net.set_sample_rate(sample_rate);
    let mut backend = BlockRateAdapter::new(Box::new(net.backend()));
    let mut next_value = move || assert_no_alloc(|| backend.get_stereo() );

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            write_data(data, channels, &mut next_value)
        },
        err_fn,
        None,
    )?;
    stream.play()?;
    Ok((stream, net))
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> (f32, f32))
where
    T: SizedSample + FromSample<f32>,
{
    for frame in output.chunks_mut(channels) {
        let sample = next_sample();
        let left = T::from_sample(sample.0);
        let right: T = T::from_sample(sample.1);

        for (channel, sample) in frame.iter_mut().enumerate() {
            if channel & 1 == 0 {
                *sample = left;
            } else {
                *sample = right;
            }
        }
    }
}
