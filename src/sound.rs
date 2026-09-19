use rodio::{OutputStream, OutputStreamHandle, Sink, Source};
use rodio::buffer::SamplesBuffer;
use rand::Rng;

pub struct SoundManager {
    _stream: OutputStream,
    handle: OutputStreamHandle,
    click_samples: Vec<f32>,
    space_samples: Vec<f32>,
    sample_rate: u32,
}

fn generate_click(sample_rate: u32) -> Vec<f32> {
    let duration = 0.015;
    let samples_count = (sample_rate as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(samples_count);
    let base_freq = 3200.0;

    for i in 0..samples_count {
        let t = i as f32 / sample_rate as f32;
        let envelope = (-t * 200.0).exp() * (1.0 - (-t * 800.0).exp());
        let click = (2.0 * std::f32::consts::PI * base_freq * t).sin();
        let harmonic = (2.0 * std::f32::consts::PI * base_freq * 2.3 * t).sin() * 0.3;
        let noise = rand::thread_rng().gen_range(-0.5..0.5_f32) * 0.15;
        let value = (click * 0.6 + harmonic * 0.25 + noise) * envelope * 0.4;
        samples.push(value.max(-1.0).min(1.0));
    }
    samples
}

fn generate_space(sample_rate: u32) -> Vec<f32> {
    let duration = 0.035;
    let samples_count = (sample_rate as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(samples_count);
    let base_freq = 1200.0;

    for i in 0..samples_count {
        let t = i as f32 / sample_rate as f32;
        let envelope = (-t * 60.0).exp() * (1.0 - (-t * 300.0).exp());
        let thock = (2.0 * std::f32::consts::PI * base_freq * t).sin();
        let deep = (2.0 * std::f32::consts::PI * base_freq * 0.5 * t).sin() * 0.5;
        let rattle = (2.0 * std::f32::consts::PI * base_freq * 3.7 * t).sin() * 0.15;
        let noise = rand::thread_rng().gen_range(-0.3..0.3_f32) * 0.2;
        let value = (thock * 0.5 + deep * 0.35 + rattle * 0.1 + noise) * envelope * 0.5;
        samples.push(value.max(-1.0).min(1.0));
    }
    samples
}

impl SoundManager {
    pub fn new() -> Option<Self> {
        let (stream, handle) = OutputStream::try_default().ok()?;
        let sample_rate = 44100;

        Some(Self {
            _stream: stream,
            handle,
            click_samples: generate_click(sample_rate),
            space_samples: generate_space(sample_rate),
            sample_rate,
        })
    }

    pub fn play_key(&self) {
        if let Ok(sink) = Sink::try_new(&self.handle) {
            let buffer = SamplesBuffer::new(1, self.sample_rate, self.click_samples.clone());
            sink.append(buffer);
            sink.detach();
        }
    }

    pub fn play_space(&self) {
        if let Ok(sink) = Sink::try_new(&self.handle) {
            let buffer = SamplesBuffer::new(1, self.sample_rate, self.space_samples.clone());
            sink.append(buffer);
            sink.detach();
        }
    }
}
