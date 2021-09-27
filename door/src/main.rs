use bevy::prelude::*;
use bevy::window::*;

struct TaskBar;

struct Colours {
    taskbar: Handle<ColorMaterial>,
}

fn spawn_taskbar(mut commands: Commands, colours: Res<Colours>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: colours.taskbar.clone(),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .insert(TaskBar);
}

// Contains code that needs to run when the OS is started
fn setup(mut commands: Commands, mut colours: ResMut<Assets<ColorMaterial>>) {
    // Creates the camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Setup Colours
    commands.insert_resource(Colours {
        taskbar: colours.add(Color::rgb(0.7, 0.7, 0.7).into()),
    });
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
        .add_startup_stage("os_main", SystemStage::single(spawn_taskbar.system()))
        .add_system(process_loop.system())
        .add_plugins(DefaultPlugins)
        .run();
}