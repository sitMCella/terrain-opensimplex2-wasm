mod fast;
mod smooth;
mod terrain_configuration;

use terrain_configuration::{configure_terrain, TerrainConfiguration};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen(start)]
pub fn start() {}

#[wasm_bindgen]
pub struct TerrainSettings {
    width: f32,
    depth: f32,
    seed: i64,
    color: String,
    max_height: f32,
    failoff: f32,
    z: f64,
    fractal_octaves: i32,
    fractal_frequency: f64,
}

#[wasm_bindgen]
impl TerrainSettings {
    #[wasm_bindgen(constructor)]
    pub fn new(
        width: f32,
        depth: f32,
        seed: i64,
        color: String,
        max_height: f32,
        failoff: f32,
        z: f64,
        fractal_octaves: i32,
        fractal_frequency: f64,
    ) -> TerrainSettings {
        TerrainSettings {
            width,
            depth,
            seed,
            color,
            max_height,
            failoff,
            z,
            fractal_octaves,
            fractal_frequency,
        }
    }
}

#[wasm_bindgen]
pub fn generate_terrain(terrainSettings: TerrainSettings) -> JsValue {
    let terrain_configuration = TerrainConfiguration::new(
        terrainSettings.width,
        terrainSettings.depth,
        terrainSettings.seed,
        terrainSettings.color,
        terrainSettings.max_height,
        terrainSettings.failoff,
        terrainSettings.z,
        terrainSettings.fractal_octaves,
        terrainSettings.fractal_frequency,
    );

    let terrain = configure_terrain(&terrain_configuration);
    serde_wasm_bindgen::to_value(&terrain).unwrap()
}
