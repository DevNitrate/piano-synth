use bevy::{asset::{Assets, Handle}, color::Color, ecs::{component::Component, entity::Entity, query::{With, Without}, system::{Commands, Query, ResMut}}, math::{Quat, Vec2, primitives::{Circle, Rectangle}}, mesh::{Mesh, Mesh2d}, sprite_render::{ColorMaterial, MeshMaterial2d}, transform::components::Transform};

#[derive(Component, Clone)]
pub struct SoundString {
    pub origin: Vec2,
    pub length: f32,
    pub sample_count: usize,
    pub samples: Vec<f32>,
    pub samples_prev: Vec<f32>,
    // wave speed depending on tension and mass per unit length
    pub c: f32
}

#[derive(Component, Clone)]
pub struct StringEntity {
    pub parent: Entity,
    pub idx: usize
}

#[derive(Component)]
pub struct CircleEntity;

#[derive(Component)]
pub struct RectangleEntity;

pub fn spawn_string(origin: Vec2, length: f32, samples: Vec<f32>, c: f32, string_color: Color, commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<ColorMaterial>>) {
    let circle_shape: Handle<Mesh> = meshes.add(Circle::new(10.0));
    let circle_color: Handle<ColorMaterial> = materials.add(Color::srgb_u8(255, 255, 255));
    let line_color: Handle<ColorMaterial> = materials.add(string_color);
    
    let sample_count: usize = samples.len();

    let sound_str: SoundString = SoundString {
        origin,
        length,
        sample_count,
        samples: samples.clone(),
        samples_prev: samples,
        c
    };

    let sound_str_entity: Entity = commands.spawn(sound_str.clone()).id();
    let circle_dist: f32 = sound_str.length / (sample_count as f32 - 1.0);

    for i in 0..sample_count {
        let str_entity: StringEntity = StringEntity {
            parent: sound_str_entity,
            idx: i
        };

        let sample: f32 = sound_str.samples[i];

        commands.spawn((
            Mesh2d(circle_shape.clone()),
            Transform::from_xyz(sound_str.origin.x + ((i as f32) * circle_dist), sound_str.origin.y + sample, 0.0),
            MeshMaterial2d(circle_color.clone()),
            str_entity.clone(),
            CircleEntity 
        ));

        let next_sample: f32 = if i < (sample_count - 1) {sound_str.samples[i+1]} else {sample};
        let y_length: f32 = next_sample - sample;
        let x_coef: f32 = if i < (sample_count - 1) {circle_dist} else {0.0};
        let length: f32 = (x_coef*x_coef + y_length*y_length).sqrt();

        let angle: f32 = (y_length / x_coef).atan();

        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(length, 7.0))),
            Transform::from_xyz(sound_str.origin.x + (((i as f32) * circle_dist) + circle_dist * 0.5), sound_str.origin.y + ((sample + next_sample) * 0.5), -1.0).with_rotation(Quat::from_rotation_z(angle)),
            MeshMaterial2d(line_color.clone()),
            str_entity,
            RectangleEntity
        ));
    }

}

pub fn draw_strings(query_circles: Query<(&mut Transform, &StringEntity), With<CircleEntity>>, query_rect: Query<(&mut Transform, &StringEntity), (With<RectangleEntity>, Without<CircleEntity>)>, query_string: Query<&SoundString>) {
    for (mut circle_transform, string_entity) in query_circles {
        let sound_str: &SoundString = query_string.get(string_entity.parent).unwrap();

        circle_transform.translation.y = sound_str.samples[string_entity.idx] + sound_str.origin.y;
    }

    for (mut rect_transform, string_entity) in query_rect {
        let sound_str: &SoundString = query_string.get(string_entity.parent).unwrap();

        let sample: f32 = sound_str.samples[string_entity.idx];
        let next: f32 = if string_entity.idx < (sound_str.sample_count - 1) {sound_str.samples[string_entity.idx + 1]} else {sample};
        let y_length: f32 = next - sample;
        let x_length: f32 = if string_entity.idx < (sound_str.sample_count - 1) {sound_str.length / (sound_str.sample_count as f32 - 1.0)} else {0.0};

        let angle: f32 = (y_length / x_length).atan();

        rect_transform.translation.y = sound_str.origin.y + ((sample + next) * 0.5);
        rect_transform.rotation = Quat::from_rotation_z(angle);
    }
}
