use bevy::{DefaultPlugins, app::{App, PluginGroup, Startup, Update}, camera::{Camera2d, ClearColor}, color::Color, ecs::{resource::Resource, schedule::IntoScheduleConfigs, system::{Commands, NonSendMut, Query, Res, ResMut}, world::World}, gizmos::config::GizmoConfigStore, input::{ButtonInput, keyboard::KeyCode}, math::Vec2, text::TextFont, ui::{Node, PositionType, px, widget::Text}, window::{MonitorSelection, Window, WindowMode, WindowPlugin}};
use dynwave::{AudioPlayer, BufferSize};

use crate::karplus::{KarplusString, draw_karplus, impulse_karplus, update_karplus};

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
    
    let karplus_string: KarplusString = KarplusString::new(440, 0.995, Vec2::new(-700.0, 0.0), 1400.0, 200.0);
    commands.spawn(karplus_string);
}

fn setup_audio(world: &mut World) {
    world.insert_non_send_resource(
        AudioPlayer::<f32>::new(48000, BufferSize::Samples(800)).unwrap()
    );
}

fn debug_audio(a: Option<NonSendMut<AudioPlayer<f32>>>) {
    println!("{}", a.is_some());
}

fn toggle_sim(keys: Res<ButtonInput<KeyCode>>, mut sim_state: ResMut<SimulationState>, mut sim_text: Query<&mut Text>, audio_player: NonSendMut<AudioPlayer<f32>>) {
    if keys.just_pressed(KeyCode::KeyS) {
        sim_state.0 = !sim_state.0;
        let state_text: &str = if sim_state.0 {"on"} else {"off"};

        for mut txt in sim_text.iter_mut() {
            txt.0 = format!("simulation: {}", state_text);
        }

        audio_player.play().unwrap();
    }
}
