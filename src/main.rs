use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod ml;
mod ui;

#[derive(Component, Clone, Copy)]
pub struct Point {
    x: f32,
    y: f32,
}

#[derive(Resource)]
pub struct Model {
    w: f32,
    b: f32,
}

fn startup(mut commands: bevy::ecs::system::Commands) {
    commands.spawn(Point { x: 10.0, y: 110.0 });
    commands.spawn(Point { x: 50.0, y: 150.0 });
    commands.spawn(Point { x: 100.0, y: 200.0 });
    commands.spawn(Point { x: 200.0, y: 300.0 });

    commands.insert_resource(Model { w: 1.0, b: 10.0 });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_systems(Startup, startup)
        .add_systems(Update, ml::ml)
        .add_systems(Update, ui::ui)
        .run();
}
