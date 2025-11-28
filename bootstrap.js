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

let mesh;

let initialization = true;

export async function run(width, depth, seed, color, max_height, failoff, z, fractal_octaves, fractal_frequency, cameraPositionX, cameraPositionY, cameraPositionZ, cameraFieldViewY, cameraFarZ, cameraTargetX, cameraTargetY, cameraTargetZ) {
  await init();
  let terrain = generate_terrain(width, depth, seed, color, max_height, failoff, z, fractal_octaves, fractal_frequency);

  if (mesh !== null) {
    scene.remove(mesh);
  }

  camera.position.set(cameraPositionX, cameraPositionY, cameraPositionZ);
  camera.fov = cameraFieldViewY;
  camera.near = 0.1;
  if (initialization === true) { // Error: the mesh disappears if the parameter var is modified after the first assignment.
    camera.far = cameraFarZ;
  }
  camera.lookAt(cameraTargetX, cameraTargetY, cameraTargetZ);

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
