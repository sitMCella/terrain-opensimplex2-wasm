mod fast;
mod smooth;
mod terrain_configuration;

use terrain_configuration::{configure_terrain, TerrainConfiguration};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen(start)]
pub fn start() {}

#[wasm_bindgen]
pub fn generate_terrain(
    width: f32,
    depth: f32,
    seed: i64,
    color: String,
    max_height: f32,
    failoff: f32,
    z: f64,
    fractal_octaves: i32,
    fractal_frequency: f64,
) -> JsValue {
    let terrain_configuration = TerrainConfiguration::new(
        width,
        depth,
        seed,
        color,
        max_height,
        failoff,
        z,
        fractal_octaves,
        fractal_frequency,
    );

    let terrain = configure_terrain(&terrain_configuration);
    serde_wasm_bindgen::to_value(&terrain).unwrap()
}
