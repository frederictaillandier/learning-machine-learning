use bevy::ecs::system::Query;
use bevy_egui::egui::Pos2;

use crate::Point;

fn draw_line(ui: &bevy_egui::egui::Ui, w: f32, b: f32) {
    let x1 = -500.0;
    let y1 = w * x1 + b;

    let x2 = 500.0;
    let y2 = w * x2 + b;

    let p1 = Pos2 {
        x: 500.0 + x1,
        y: 500.0 - y1,
    };
    let p2 = Pos2 {
        x: 500.0 + x2,
        y: 500.0 - y2,
    };

    ui.painter().line_segment(
        [p1, p2],
        bevy_egui::egui::Stroke::new(1.0, bevy_egui::egui::Color32::WHITE),
    );
}

fn draw_segment(ui: &bevy_egui::egui::Ui, a: bevy_egui::egui::Pos2, b: bevy_egui::egui::Pos2) {
    ui.painter().line_segment(
        [a, b],
        bevy_egui::egui::Stroke::new(1.0, bevy_egui::egui::Color32::WHITE),
    );
}

fn draw_cross(ui: &bevy_egui::egui::Ui, p: &crate::Point, r: f32) {
    let p = Pos2 {
        x: 500.0 + p.x,
        y: 500.0 - p.y,
    };

    draw_segment(ui, Pos2 { x: p.x - r, y: p.y }, Pos2 { x: p.x + r, y: p.y });
    draw_segment(ui, Pos2 { x: p.x, y: p.y + r }, Pos2 { x: p.x, y: p.y - r });
}

pub fn ui(
    mut contexts: bevy_egui::EguiContexts,
    query: Query<&mut Point>,
    model: bevy::ecs::system::ResMut<crate::Model>,
) {
    let ctx = contexts.ctx_mut();
    bevy_egui::egui::panel::CentralPanel::default().show(ctx, |ui| {
        draw_cross(ui, &Point { x: 0.0, y: 0.0 }, 500.0);

        for p in query.iter() {
            draw_cross(ui, p, 5.0);
        }
        draw_line(ui, model.w, model.b);
    });
}
