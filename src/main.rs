use bevy::prelude::*;
use bevy_egui::{egui::Pos2, EguiPlugin};

fn draw_line(ui: &bevy_egui::egui::Ui, a: bevy_egui::egui::Pos2, b: bevy_egui::egui::Pos2) {
    ui.painter().line_segment(
        [a, b],
        bevy_egui::egui::Stroke::new(1.0, bevy_egui::egui::Color32::WHITE),
    );
}

fn ui_example_system(mut contexts: bevy_egui::EguiContexts) {
    let ctx = contexts.ctx_mut();

    bevy_egui::egui::panel::CentralPanel::default().show(ctx, |ui| {
        draw_line(
            ui,
            Pos2 { x: 0.0, y: 500.0 },
            Pos2 {
                x: 1000.0,
                y: 500.0,
            },
        );
        draw_line(
            ui,
            Pos2 { x: 500.0, y: 0.0 },
            Pos2 {
                x: 500.0,
                y: 1000.0,
            },
        );
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
        .add_systems(Update, ui_example_system)
        .run();
}
