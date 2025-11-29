import init, { generate_terrain } from "./pkg/terrain-webassembly.js";
import * as THREE from './three.module.js';

const scene = new THREE.Scene();
scene.background = new THREE.Color(0x87ceeb);

const light = new THREE.DirectionalLight(0xffffff, 1);
light.position.set(50, 100, 50);
scene.add(light);
scene.add(new THREE.AmbientLight(0x555555));

const renderer = new THREE.WebGLRenderer({ antialias: true });
renderer.setSize(window.innerWidth, window.innerHeight);
document.body.appendChild(renderer.domElement);

const camera = new THREE.PerspectiveCamera(
  45,
  window.innerWidth / window.innerHeight,
  0.1,
  2000
);

export class TerrainParameters {
  constructor(width, depth, seed, color, max_height, failoff, z, fractal_octaves, fractal_frequency) {
    this.width = width;
    this.depth = depth;
    this.seed = seed;
    this.color = color;
    this.max_height = max_height;
    this.failoff = failoff;
    this.z = z;
    this.fractal_octaves = fractal_octaves;
    this.fractal_frequency = fractal_frequency;
  }
}

export class CameraParameters {
  constructor(cameraPositionX, cameraPositionY, cameraPositionZ, cameraFieldViewY, cameraFarZ, cameraTargetX, cameraTargetY, cameraTargetZ) {
    this.cameraPositionX = cameraPositionX;
    this.cameraPositionY = cameraPositionY;
    this.cameraPositionZ = cameraPositionZ;
    this.cameraFieldViewY = cameraFieldViewY;
    this.cameraFarZ = cameraFarZ;
    this.cameraTargetX = cameraTargetX;
    this.cameraTargetY = cameraTargetY;
    this.cameraTargetZ = cameraTargetZ;
  }
}


let mesh;

let initialization = true;

export async function run(terrainParameters, cameraParameters) {
  await init();
  let terrain = generate_terrain(terrainParameters.width, terrainParameters.depth, terrainParameters.seed, terrainParameters.color, terrainParameters.max_height, terrainParameters.failoff, terrainParameters.z, terrainParameters.fractal_octaves, terrainParameters.fractal_frequency);

  if (mesh !== null) {
    scene.remove(mesh);
  }

  camera.position.set(cameraParameters.cameraPositionX, cameraParameters.cameraPositionY, cameraParameters.cameraPositionZ);
  camera.fov = cameraParameters.cameraFieldViewY;
  camera.near = 0.1;
  if (initialization === true) { // The mesh disappears if camera.far is modified after the first assignment.
    camera.far = cameraParameters.cameraFarZ;
  }
  camera.lookAt(cameraParameters.cameraTargetX, cameraParameters.cameraTargetY, cameraParameters.cameraTargetZ);

  camera.updateProjectionMatrix();

  const geometry = new THREE.BoxGeometry(1, 1, 1);
  const material = new THREE.MeshStandardMaterial({});

  mesh = new THREE.InstancedMesh(
    geometry,
    material,
    terrain.positions.length
  );

  const dummy = new THREE.Object3D();

  for (let i = 0; i < terrain.positions.length; i++) {
    const p = terrain.positions[i];
    const c = terrain.colors[i];

    dummy.position.set(p.x, p.y, p.z);
    dummy.updateMatrix();

    mesh.setMatrixAt(i, dummy.matrix);

    mesh.setColorAt(
      i,
      new THREE.Color(
        c.r / 255,
        c.g / 255,
        c.b / 255
      )
    );
  }

  mesh.instanceMatrix.needsUpdate = true;
  mesh.instanceColor.needsUpdate = true;

  scene.add(mesh);

  if (initialization === true) {
    function animate() {
      requestAnimationFrame(animate);
      renderer.render(scene, camera);
    }

    animate();

    window.addEventListener('resize', () => {
      camera.aspect = window.innerWidth / window.innerHeight;
      camera.updateProjectionMatrix();
      renderer.setSize(window.innerWidth, window.innerHeight);
    });

    initialization = false;
  }
}
