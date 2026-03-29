use bevy::{DefaultPlugins, app::{App, PluginGroup, Startup, Update}, camera::{Camera2d, ClearColor}, color::Color, ecs::{resource::Resource, schedule::IntoScheduleConfigs, system::{Commands, Query, Res, ResMut}, world::World}, gizmos::config::GizmoConfigStore, input::{ButtonInput, keyboard::KeyCode}, math::Vec2, text::TextFont, ui::{Node, PositionType, px, widget::Text}, window::{MonitorSelection, Window, WindowMode, WindowPlugin}};
use dynwave::{AudioPlayer, BufferSize};

use crate::{karplus::{KarplusString, draw_karplus, impulse_karplus, update_karplus}};

mod karplus;

#[derive(Resource)]
pub struct SimulationState(bool);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..Default::default()
            }),
            ..Default::default()
        }))
        //.add_plugins(FpsOverlayPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_audio)
        .add_systems(Update, update_karplus)
        .add_systems(Update, (draw_karplus.after(update_karplus), toggle_sim, impulse_karplus))
        .insert_resource(ClearColor(Color::srgb_u8(13, 13, 18)))
        .run();
}

fn setup(mut commands: Commands, mut config_store: ResMut<GizmoConfigStore>) {
    commands.spawn(Camera2d);
    commands.insert_resource(SimulationState(false));

    commands.spawn((
            Text::new("simulation: off"),
            TextFont {
                font_size: 35.0,
                ..Default::default()
            },
            Node {
                position_type: PositionType::Absolute,
                top: px(15),
                left: px(15),
                ..Default::default()
            }
    ));

    for (_, conf, _) in config_store.iter_mut() {
        conf.line.width = 5.0;
    }
    
    let karplus_string: KarplusString = KarplusString::new(440, 0.998, Vec2::new(-700.0, 0.0), 1400.0, 50.0);
    let karplus_string1: KarplusString = KarplusString::new(440, 0.998, Vec2::new(-700.0, -100.0), 1400.0, 50.0); 
    let karplus_string2: KarplusString = KarplusString::new(440, 0.998, Vec2::new(-700.0, 100.0), 1400.0, 50.0);
    commands.spawn(karplus_string);
    commands.spawn(karplus_string1);
    commands.spawn(karplus_string2);
}

fn setup_audio(world: &mut World) {
    let player: AudioPlayer<f32> = AudioPlayer::<f32>::new(48000, BufferSize::HalfSecond).unwrap();
    player.play().unwrap();

    world.insert_non_send_resource(
        player
    );
}

fn toggle_sim(keys: Res<ButtonInput<KeyCode>>, mut sim_state: ResMut<SimulationState>, mut sim_text: Query<&mut Text>) {
    if keys.just_pressed(KeyCode::KeyS) {
        sim_state.0 = !sim_state.0;
        let state_text: &str = if sim_state.0 {"on"} else {"off"};

        for mut txt in sim_text.iter_mut() {
            txt.0 = format!("simulation: {}", state_text);
        }
    }
}
