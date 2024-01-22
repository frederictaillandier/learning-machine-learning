use bevy::prelude::*;
use bevy_egui::EguiPlugin;

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
    commands.spawn(Point { x: 10.0, y: 10.0 });
    commands.spawn(Point { x: 100.0, y: 100.0 });
    commands.insert_resource(Model { w: 1.0, b: 100.0 });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_systems(Startup, startup)
        // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
        .add_systems(Update, ui::ui)
        .run();
}
