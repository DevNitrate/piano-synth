use bevy::{DefaultPlugins, app::{App, Startup, Update}, asset::{Assets}, camera::{Camera2d, ClearColor}, color::Color, ecs::{system::{Commands, ResMut}}, math::Vec2, mesh::Mesh, sprite_render::ColorMaterial};

use crate::sound_string::{draw_strings, spawn_string};

mod sound_string;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, draw_strings)
        .insert_resource(ClearColor(Color::srgb_u8(13, 13, 18)))
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2d);

    let mut samples: Vec<f32> = Vec::new();

    for i in 0..25 {
        let sample: f32 = (i as f32 / 2.5).sin() * 100.0;

        samples.push(sample);
    }
    
    spawn_string(Vec2::new(-800.0, 0.0), 1600.0, samples, 100.0, Color::srgb_u8(181, 61, 61), &mut commands, &mut meshes, &mut materials);    
}
