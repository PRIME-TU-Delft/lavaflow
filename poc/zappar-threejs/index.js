const targetImagePath = new URL('./tracker2.zpt', import.meta.url).href;

// ZapparThree provides a LoadingManager that shows a progress bar while
// the assets are downloaded
const manager = new ZapparThree.LoadingManager();

// Setup ThreeJS in the usual way
const renderer = new THREE.WebGLRenderer();
document.body.appendChild(renderer.domElement);

renderer.setSize(window.innerWidth, window.innerHeight);
window.addEventListener('resize', () => {
  renderer.setSize(window.innerWidth, window.innerHeight);
});

renderer.setAnimationLoop(render);

// Setup a Zappar camera instead of one of ThreeJS's cameras
const camera = new ZapparThree.Camera();

// The Zappar library needs your WebGL context, so pass it
ZapparThree.glContextSet(renderer.getContext());

// Create a ThreeJS Scene and set its background to be the camera background texture
const scene = new THREE.Scene();
scene.background = camera.backgroundTexture;

// Request the necessary permission from the user
ZapparThree.permissionRequestUI().then((granted) => {
  if (granted) camera.start();
  else ZapparThree.permissionDeniedUI();
});

// Set up our image tracker group
// Pass our loading manager in to ensure the progress bar works correctly
const tracker = new ZapparThree.ImageTrackerLoader(manager).load(
  targetImagePath
);
const trackerGroup = new ZapparThree.ImageAnchorGroup(camera, tracker);
scene.add(trackerGroup);

// Add some content
const box = new THREE.Mesh(
  new THREE.BoxBufferGeometry(),
  new THREE.MeshBasicMaterial()
);
box.position.set(0, 0, 0.5);
trackerGroup.add(box);

// Set up our render loop
function render() {
  camera.updateFrame(renderer);
  renderer.render(scene, camera);
}
