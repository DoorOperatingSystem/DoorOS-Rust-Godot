use std::env;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::*;
use engine;

const OS_VERSION: &str = "0.1.0";

const STAGES: [&str; 1] = [
    "os main"
];

// Contains code that needs to run when the OS is started
fn setup(mut commands: Commands, mut colours: ResMut<Assets<ColorMaterial>>, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    println!("\n+ /-= Running DoorOS{} =-\\", env::consts::EXE_SUFFIX);
    println!("| OS version: {}", OS_VERSION);
    println!("| Engine version: {}", engine::ENGINE_VERSION);
    println!("| Screen dimensions: ({}, {}) ", window.width(), window.height());
    println!("| Underlying OS: {} ({})", env::consts::OS, env::consts::ARCH);
    println!("+\n");

    // Creates the camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Setup Colours
    commands.insert_resource(engine::Colours {
        taskbar: colours.add(Color::rgb(110.0, 110.0, 110.0).into()),
        background: colours.add(Color::rgb(110.0, 0.0, 0.0).into())
    });
}

// Contains code that needs to be run every tick
fn process_loop(keyboard_input: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keyboard_input.pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}

// Main function, creates the OS window and stuff
fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: format!("DoorOS").to_string(),
            height: 800.0,
            width: 1500.0,
            mode: WindowMode::Fullscreen{use_size: true},
            ..Default::default()
        })
        .add_startup_system(setup.system())
        .add_startup_stage(STAGES[0], SystemStage::single(engine::spawn::background.system()))
        .add_startup_system_to_stage(STAGES[0], engine::spawn::taskbar.system())
        //.add_startup_system_to_stage
        .add_system(process_loop.system())
        .add_plugins(DefaultPlugins)
        .run();
}