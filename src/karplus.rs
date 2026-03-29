use bevy::{color::Color, ecs::{component::Component, system::{NonSendMut, Query, Res}}, gizmos::gizmos::Gizmos, input::{ButtonInput, keyboard::KeyCode}, math::Vec2};
use dynwave::AudioPlayer;

use crate::{SimulationState};

#[derive(Component)]
pub struct KarplusString {
    pub buffer: Vec<f32>,
    pub len: usize,
    head: usize,
    decay: f32,

    // for drawing 
    vertices: Vec<Vec2>,
    origin: Vec2,
    scaling: f32
}

impl KarplusString {
    pub fn new(frequency: usize, decay: f32, origin: Vec2, width: f32, scaling: f32) -> Self {
        let len: usize = 48000 / frequency;
        let mut buffer: Vec<f32> = Vec::new();
        let mut vertices: Vec<Vec2> = Vec::new();
        let head: usize = rand::random_range(0..(len as f32 * 0.2) as usize);
        println!("head: {}", head);

        let spacing: f32 = width / (len-1) as f32;

        for i in 0..len {
            buffer.push(rand::random_range(-1.0..1.0));
            vertices.push(Vec2::new((i as f32)*spacing + origin.x, (buffer[i] * scaling) + origin.y));
        }

        KarplusString {
            buffer,
            len,
            head,
            decay,

            vertices,
            origin,
            scaling
        }
    }

    pub fn step(&mut self) -> f32 {
        let next_idx: usize = (self.head + 1) % self.len;
        let new_sample: f32 = (0.5 * (self.buffer[self.head] + self.buffer[next_idx])) * self.decay;
        self.buffer[self.head] = new_sample;
        self.vertices[self.head].y = (self.buffer[self.head] * self.scaling) + self.origin.y;
        self.head = next_idx;

        new_sample
    }

    fn impulse(&mut self) {
        let strike_start: usize = (0.1 * self.len as f32) as usize;
        let strike_width: usize = (0.1 * self.len as f32) as usize;

        for i in 0..self.len {
            if i >= strike_start && i < (strike_start + strike_width) {
                let t = (i - strike_start) as f32 / strike_width as f32;

                self.buffer[i] = ((1.0 - t) + (t*0.5)) * rand::random_range(0.8..1.0);
                self.vertices[i].y = (self.buffer[i] * self.scaling) + self.origin.y;
            } else {
                self.buffer[i] = 0.0;
                self.vertices[i].y = self.origin.y;
            } 
        }
    }
}

pub fn draw_karplus(mut gizmos: Gizmos, mut karplus_strings: Query<&mut KarplusString>) { 
    for karplus_string in karplus_strings.iter_mut() {
        gizmos.linestrip_2d(karplus_string.vertices.clone(), Color::srgb_u8(181, 61, 61));
    }
}

pub fn update_karplus(mut karplus_strings: Query<&mut KarplusString>, sim_state: Res<SimulationState>, mut audio_player: NonSendMut<AudioPlayer<f32>>) {
    if !sim_state.0 {
        audio_player.queue(&vec![0.0; 1600]);
        return;
    }
    
    let mut queue: [f32; 1600] = [0.0; 1600];

    for mut karplus_string in karplus_strings.iter_mut() {

        for i in 0..800 {
            let sample: f32 = karplus_string.step();
            queue[i*2] += sample;
            queue[i*2+1] += sample;
        }
    }

    for el in queue.iter_mut() {
        *el /= karplus_strings.iter().len() as f32;
    }

    audio_player.queue(&queue);
}

pub fn impulse_karplus(keys: Res<ButtonInput<KeyCode>>, mut karplus_strings: Query<&mut KarplusString>) {
    if keys.just_pressed(KeyCode::KeyI) {
        for mut karplus_string in karplus_strings.iter_mut() {
            karplus_string.impulse();
        }
    }
}
