use bevy::prelude::*;
use bevy::window::*;

// Contains code that needs to run when the OS is started
fn setup() {
    
}

// Contains code that needs to be run every tick
fn process_loop() {

}

// Main function, creates the OS window and stuff
fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "DoorOS".to_string(),
            height: 4000.0,
            width: 4500.0,
            mode: WindowMode::Fullscreen{use_size: true},
            ..Default::default()
        })
        .add_startup_system(setup.system())
        .add_system(process_loop.system())
        .add_plugins(DefaultPlugins)
        .run();
}