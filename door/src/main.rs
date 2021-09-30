use bevy::prelude::*;
use bevy::window::*;

struct TaskBar;

struct Colours {
    taskbar: Handle<ColorMaterial>,
    background: Handle<ColorMaterial>
}

fn spawn_background(mut commands: Commands, colours: Res<Colours>, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    let mut start_of_the_window_width: f32 = window.width() / 2.0;
    start_of_the_window_width = start_of_the_window_width - start_of_the_window_width;
    let mut start_of_the_window_height: f32 = window.height() / 2.0;
    start_of_the_window_height = start_of_the_window_height - start_of_the_window_height;

    commands
        .spawn_bundle(SpriteBundle {
            material: colours.background.clone(),
            sprite: Sprite::new(Vec2::new(window.width(), window.height())),
            transform: Transform::from_translation(Vec3::new(
                start_of_the_window_width, 
                start_of_the_window_height,
                0.0)),
            ..Default::default()
        })
        .insert(TaskBar);
}

fn spawn_taskbar(mut commands: Commands, colours: Res<Colours>, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    let mut start_of_the_window: f32 = window.width() / 2.0;
    start_of_the_window = start_of_the_window - start_of_the_window;

    commands
        .spawn_bundle(SpriteBundle {
            material: colours.taskbar.clone(),
            sprite: Sprite::new(Vec2::new(window.width(), window.height() / 15.0)),
            transform: Transform::from_translation(Vec3::new(
                start_of_the_window, 
                -(window.height() / 2.0 - (window.height() / 15.0 - (window.height() / 30.0))), 999.)),
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
        taskbar: colours.add(Color::rgb(0.128, 0.128, 0.135).into()),
        background: colours.add(Color::rgb(128.0, 0.0, 0.0).into())
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
        .add_startup_system_to_stage("os_main", spawn_background.system())
        .add_system(process_loop.system())
        .add_plugins(DefaultPlugins)
        .run();
}