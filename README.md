# Terrain Generation using OpenSimplex 2

![Document](https://github.com/sitMCella/terrain-opensimplex2-wasm/wiki/images/terrain.png)

## Table of contents

* [Introduction](#introduction)
* [Development](#development)
    * [Setup Development](#setup-development)
    * [Build Project Development](#build-project-development)
* [Run Application](#run-application)
    * [Terrain Application](#terrain-application)
    * [Control Panel](#control-panel)

## Introduction

Terrain generation and visualization using OpenSimplex 2 and 3D noise.

This project includes code originally made available under CC0 1.0 Universal (Public Domain).
Original source: https://github.com/KdotJPG/OpenSimplex2

## Development

The project involves visualizing a terrain using the Three.js library.

The terrain is generated with the OpenSimplex 2 algorithm, implemented in Rust and compiled to WebAssembly.

A control panel is used for adjusting the terrain and camera parameters.

### Setup Development

Install Rust and Cargo. Recommended version:
- Rustc and Cargo >= 1.88.0

Prerequisites for WebAssembly:

```sh
rustup update
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
```

### Build Project Development

#### Format Code (Backend)

```sh
cargo fmt
```

#### Build

```sh
wasm-pack build --release --target web --out-name terrain-webassembly
```

#### Run application

```sh
python3 -m http.server 8080 
```

Open in the browser: http://localhost:8080/terrain.html
