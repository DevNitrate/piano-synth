use std::fs;

use bevy::ecs::resource::Resource;

pub struct WavHeader {
    type_bloc_id: [u8; 4],
    file_size: u32,
    file_format_id: [u8; 4],

    format_bloc_id: [u8; 4],
    bloc_size: u32,
    audio_format: u16,
    n_channels: u16,
    frequency: u32,
    bytes_per_sec: u32,
    bytes_per_bloc: u16,
    bits_per_sample: u16,

    data_bloc_id: [u8; 4],
    data_size: u32
}

impl WavHeader {
    pub fn new(frequency: u32, duration: u32) -> Self {
        let num_samples: u32 = frequency * duration;
        let file_size: u32 = (num_samples as usize * size_of::<f32>() + 44) as u32;

        let header: WavHeader = WavHeader {
            type_bloc_id: *b"RIFF",
            file_size: file_size - 8,
            file_format_id: *b"WAVE",
            
            format_bloc_id: *b"fmt ",
            bloc_size: 16,
            audio_format: 3,
            n_channels: 1,
            frequency,
            bytes_per_sec: frequency * size_of::<f32>() as u32,
            bytes_per_bloc: size_of::<f32>() as u16,
            bits_per_sample: size_of::<f32>() as u16 * 8,

            data_bloc_id: *b"data",
            data_size: num_samples * size_of::<f32>() as u32
        };

        header
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.extend_from_slice(&self.type_bloc_id);
        bytes.extend_from_slice(&self.file_size.to_le_bytes());
        bytes.extend_from_slice(&self.file_format_id);
        bytes.extend_from_slice(&self.format_bloc_id);
        bytes.extend_from_slice(&self.bloc_size.to_le_bytes());
        bytes.extend_from_slice(&self.audio_format.to_le_bytes());
        bytes.extend_from_slice(&self.n_channels.to_le_bytes());
        bytes.extend_from_slice(&self.frequency.to_le_bytes());
        bytes.extend_from_slice(&self.bytes_per_sec.to_le_bytes());
        bytes.extend_from_slice(&self.bytes_per_bloc.to_le_bytes());
        bytes.extend_from_slice(&self.bits_per_sample.to_le_bytes());
        bytes.extend_from_slice(&self.data_bloc_id);
        bytes.extend_from_slice(&self.data_size.to_le_bytes());

        bytes
    }
}

#[derive(Resource)]
pub struct WavFile {
    _frequency: u32,
    _duration: u32,
    header: WavHeader,
    pub num_samples: usize,
    pub samples: Vec<f32>
}

impl WavFile {
    pub fn new(frequency: u32, duration: u32) -> Self {
        let num_samples: usize = (frequency * duration) as usize;

        WavFile {
            _frequency: frequency,
            _duration: duration,
            header: WavHeader::new(frequency, duration),
            num_samples,
            samples: Vec::with_capacity(num_samples)
        }
    }

    pub fn push_sample(&mut self, sample: f32) {
        self.samples.push(sample);
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = self.header.to_bytes();
        let sample_bytes: Vec<u8> = self.samples.iter().flat_map(|s| s.to_le_bytes()).collect();
        bytes.extend_from_slice(&sample_bytes);

        bytes
    }

    pub fn write_to_file(&self, file: &str) {
        let bytes: Vec<u8> = self.to_bytes();

        fs::write(file, &bytes).unwrap();
    }
}
