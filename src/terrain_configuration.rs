use cgmath::vec3;
use cgmath::Vector3;
use serde::{Deserialize, Serialize};

use crate::smooth::noise3_ImproveXZ;

const CUBE_SIZE: f32 = 1.0;

#[derive(Debug, Clone)]
pub struct TerrainConfiguration {
    tot_width: f32,
    tot_depth: f32,
    seed: i64,
    color: String,
    max_height: f32,
    failoff: f32,
    z: f64,
    fractal_octaves: i32,
    fractal_frequency: f64,
}

impl TerrainConfiguration {
    pub fn new(
        tot_width: f32,
        tot_depth: f32,
        seed: i64,
        color: String,
        max_height: f32,
        failoff: f32,
        z: f64,
        fractal_octaves: i32,
        fractal_frequency: f64,
    ) -> Self {
        Self {
            tot_width,
            tot_depth,
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

pub type Vec3 = Vector3<f32>;

#[derive(Deserialize, Serialize)]
pub struct Srgba {
    /// Red component
    pub r: u8,
    /// Green component
    pub g: u8,
    /// Blue component
    pub b: u8,
    /// Alpha component
    pub a: u8,
}

impl Srgba {
    ///
    /// Creates a new sRGBA color with the given values.
    ///
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Vec3Wrapper {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// Convert to nalgebra::Vector3<f32>
impl From<Vec3Wrapper> for Vector3<f32> {
    fn from(v: Vec3Wrapper) -> Self {
        Vector3::new(v.x, v.y, v.z)
    }
}

#[derive(Deserialize, Serialize)]
pub struct Mesh {
    positions: Vec<Vec3Wrapper>,
    indices: Vec<u32>,
    colors: Vec<Srgba>,
}

fn convert_vec3s(vec3s: Vec<Vec3>) -> Vec<Vec3Wrapper> {
    vec3s
        .into_iter()
        .map(|v| Vec3Wrapper {
            x: v.x,
            y: v.y,
            z: v.z,
        })
        .collect()
}

fn fractal_noise(
    terrain_configuration: &TerrainConfiguration,
    width: f32,
    depth: f32,
    z: f64,
) -> f32 {
    let mut height: f32 = 0.0;
    let mut amplitude: f32 = 1.0;
    let mut frequency: f64 = 1.0;
    let octaves: i32 = terrain_configuration.fractal_octaves;
    for _i in 0..octaves {
        height += noise3_ImproveXZ(
            terrain_configuration.seed,
            f64::from(width) * frequency,
            f64::from(depth) * frequency,
            z,
        ) * amplitude;
        amplitude *= 0.5;
        frequency *= terrain_configuration.fractal_frequency;
    }
    height *= terrain_configuration.max_height;
    height.clamp(0.0, terrain_configuration.max_height)
}

pub fn configure_terrain(terrain_configuration: &TerrainConfiguration) -> Mesh {
    let mut terrain: Vec<Vec<Cube>> = Vec::new();
    let z: f64 = terrain_configuration.z;

    let mut width: f32 = 0.0;
    let mut depth: f32;

    while width < terrain_configuration.tot_width {
        let mut terrain_layer: Vec<Cube> = Vec::new();
        depth = 0.0;
        while depth < terrain_configuration.tot_depth {
            let value = fractal_noise(terrain_configuration, width, depth, z);
            let dist = (width * width + depth * depth).sqrt();
            let falloff = (1.0 - (dist / terrain_configuration.failoff)).max(0.0);
            let cube = Cube {
                x: width,
                y: depth,
                z: value * falloff + CUBE_SIZE,
            };
            terrain_layer.push(cube);
            depth += CUBE_SIZE;
        }
        terrain.push(terrain_layer);
        width += CUBE_SIZE;
    }

    cubes_to_voxel_mesh(&terrain, terrain_configuration)
}

#[derive(Debug)]
struct Cube {
    x: f32,
    y: f32,
    z: f32,
}

fn add_cube(positions: &mut Vec<Vec3>, indices: &mut Vec<u32>, base: Vec3, size: f32, height: f32) {
    let start = positions.len() as u32;

    let p0 = base;
    let p1 = base + vec3(size, 0.0, 0.0);
    let p2 = base + vec3(size, height, 0.0);
    let p3 = base + vec3(0.0, height, 0.0);

    let p4 = base + vec3(0.0, 0.0, size);
    let p5 = base + vec3(size, 0.0, size);
    let p6 = base + vec3(size, height, size);
    let p7 = base + vec3(0.0, height, size);

    // 8 cube corners
    positions.extend([
        p0, p1, p2, p3, // front
        p4, p5, p6, p7, // back
    ]);

    // 12 triangles (2 per face)
    indices.extend([
        // Front
        start,
        start + 1,
        start + 2,
        start,
        start + 2,
        start + 3,
        // Back
        start + 4,
        start + 6,
        start + 5,
        start + 4,
        start + 7,
        start + 6,
        // Left
        start + 4,
        start,
        start + 3,
        start + 4,
        start + 3,
        start + 7,
        // Right
        start + 1,
        start + 5,
        start + 6,
        start + 1,
        start + 6,
        start + 2,
        // Top
        start + 3,
        start + 2,
        start + 6,
        start + 3,
        start + 6,
        start + 7,
        // Bottom
        start + 4,
        start + 5,
        start + 1,
        start + 4,
        start + 1,
        start,
    ]);
}

fn cubes_to_voxel_mesh(
    cubes: &Vec<Vec<Cube>>,
    terrain_configuration: &TerrainConfiguration,
) -> Mesh {
    let mut positions = Vec::new();
    let mut indices = Vec::new();
    let mut colors: Vec<Srgba> = Vec::new();

    let color_r = u8::from_str_radix(&terrain_configuration.color[0..2], 16).unwrap();

    let color_g = u8::from_str_radix(&terrain_configuration.color[2..4], 16).unwrap();

    let color_b = u8::from_str_radix(&terrain_configuration.color[4..6], 16).unwrap();

    let base_color = color_g;

    for row in cubes {
        for cube in row {
            if cube.z < 1.0 {
                continue;
            }

            let height_trunc = cube.z.trunc();
            let fractional_part = cube.z.fract();

            let height = height_trunc.floor() as i32;

            // stack from ground (0) up to cube.z
            for level in 1..height {
                let base = vec3(cube.x, level as f32 * CUBE_SIZE, cube.y);

                add_cube(&mut positions, &mut indices, base, CUBE_SIZE, CUBE_SIZE);

                // darker color at bottom, lighter at top
                for _ in 0..8 {
                    let t = cube.z - ((height - level + 1) as f32 / height as f32 * 0.5);
                    let green = (base_color as f32 + 0.25 + (0.45 * t) * 50.0) as u8;
                    colors.push(Srgba::new(color_r, green, color_b, 255));
                }
            }

            let base = vec3(cube.x, CUBE_SIZE * height as f32, cube.y);

            add_cube(
                &mut positions,
                &mut indices,
                base,
                CUBE_SIZE,
                fractional_part,
            );

            for _ in 0..8 {
                let t = cube.z;
                let green = (base_color as f32 + 0.25 + (0.45 * t) * 50.0) as u8;
                colors.push(Srgba::new(color_r, green, color_b, 255));
            }
        }
    }

    Mesh {
        positions: convert_vec3s(positions),
        indices: indices,
        colors: colors,
    }
}
