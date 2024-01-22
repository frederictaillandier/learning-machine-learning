use bevy::ecs::system::Query;

use crate::{Model, Point};

fn update_model(model: &mut Model) {}

fn compute_cost(query: Query<&Point>, model: bevy::ecs::system::Res<crate::Model>) -> f32 {
    let mut cumulated_cost = 0.0;
    for p in query.iter() {
        let f_wb = model.w * p.x + model.b;
        let cost = (f_wb - p.y).powi(2);
        cumulated_cost += cost;
    }
    let total_cost = cumulated_cost / (2.0 * query.iter().len() as f32);
    total_cost
}

pub fn ml(query: Query<&mut Point>, model: bevy::ecs::system::ResMut<crate::Model>) {
    println!("ml");
}
