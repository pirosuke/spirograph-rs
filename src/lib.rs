extern crate serde_json;

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[derive(Serialize)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Deserialize)]
pub struct Setting {
    outer_radius: i32,
    inner_radius: i32,
    draw_radius: i32,
    area_width: i32,
    area_height: i32,
}

pub fn calc_hypotrochoid(outer_radius: i32, inner_radius: i32, draw_radius: i32, angle: f64) -> Point {
    let f_outer_radius = f64::from(outer_radius);
    let f_inner_radius = f64::from(inner_radius);
    let f_draw_radius = f64::from(draw_radius);
    let x = (f_outer_radius - f_inner_radius) * angle.cos() + f_draw_radius * ((f_outer_radius - f_inner_radius) / f_inner_radius * angle).cos();
    let y = (f_outer_radius - f_inner_radius) * angle.sin() - f_draw_radius * ((f_outer_radius - f_inner_radius) / f_inner_radius * angle).sin();
    Point {
        x: x.round() as i32,
        y: y.round() as i32,
    }
}

#[wasm_bindgen]
pub fn calc_point(setting_json: &str, angle: f64) -> String {
    let setting: Setting = serde_json::from_str(setting_json).unwrap();
    let point = calc_hypotrochoid(setting.outer_radius, setting.inner_radius, setting.draw_radius, angle);
    let center_x = setting.area_width / 2;
    let center_y = setting.area_height / 2;
    let real_point = Point {
        x: point.x + center_x,
        y: point.y + center_y,
    };
    serde_json::to_string(&real_point).unwrap()
}