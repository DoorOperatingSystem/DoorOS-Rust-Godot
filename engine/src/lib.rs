use bevy::prelude::*;

pub const ENGINE_VERSION: &str = "0.1.0";

struct TaskBar;
struct Background;

pub struct Colours {
    pub taskbar: Handle<ColorMaterial>,
    pub background: Handle<ColorMaterial>
}

pub mod spawn {
    use bevy::prelude::*;
    use crate::*;

    pub fn taskbar(mut commands: Commands, colours: Res<Colours>, mut windows: ResMut<Windows>) {
        let window = windows.get_primary_mut().unwrap();
        let mut start_of_the_window: f32 = window.width() / 2.;
        start_of_the_window = start_of_the_window - start_of_the_window;
    
        commands
            .spawn_bundle(SpriteBundle {
                material: colours.taskbar.clone(),
                sprite: Sprite::new(Vec2::new(window.width(), window.height() / 15.)),
                transform: Transform::from_translation(Vec3::new(
                    start_of_the_window, 	
                    -(window.height() / 2. - (window.height() / 15. - (window.height() / 30.))), 999.)),
                ..Default::default()
            })
            .insert(TaskBar);
    }

    pub fn background(mut commands: Commands, colours: Res<Colours>, mut windows: ResMut<Windows>) {
        let window = windows.get_primary_mut().unwrap();
        let mut start_of_the_window_width: f32 = window.width() / 2.;
        start_of_the_window_width = start_of_the_window_width - start_of_the_window_width;
        let mut start_of_the_window_height: f32 = window.height() / 2.;
        start_of_the_window_height = start_of_the_window_height - start_of_the_window_height;
    
        commands
            .spawn_bundle(SpriteBundle {
                material: colours.background.clone(),
                sprite: Sprite::new(Vec2::new(window.width(), window.height())),
                transform: Transform::from_translation(Vec3::new(
                    start_of_the_window_width, 
                    start_of_the_window_height,
                    0.)),
                ..Default::default()
            })
            .insert(Background);
    }
}