use std::ops::Deref;

use bevy::ecs::system::Query;

use crate::{Model, Point};

fn update_model(model: &mut Model) {}

fn compute_gradient(query: Query<&Point>, model: &Model) -> (f32, f32) {
    let mut w = 0.0;
    let mut b = 0.0;
    let m = query.iter().len() as f32;

    for p in query.iter() {
        let f_wb = model.w * p.x + model.b;
        let d_w = (f_wb - p.y) * p.x;
        let d_b = f_wb - p.y;
        w += d_w;
        b += d_b;
    }
    (w / m, b / m)
}

fn compute_cost(query: Query<&Point>, model: &mut Model) -> f32 {
    let mut cumulated_cost = 0.0;
    for p in query.iter() {
        let f_wb = model.w * p.x + model.b;
        let cost = (f_wb - p.y).powi(2);
        cumulated_cost += cost;
    }
    let total_cost = cumulated_cost / (2.0 * query.iter().len() as f32);
    total_cost
}

pub fn ml(query: Query<&Point>, mut model: bevy::ecs::system::ResMut<crate::Model>) {
    let grad = compute_gradient(query, &mut model);
}
