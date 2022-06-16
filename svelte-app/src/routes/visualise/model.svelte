<script lang="ts">

	import 'aframe';
	import 'aframe-extras';

	/**
	 * This view is created to test and show the client the progess of the mountain construction
	 */
	
	import SceneViewer from '$lib/components/aframe/SceneViewer.svelte';

	import { gltfStore } from '$lib/stores/gltfStore';

	import { onMount, onDestroy } from 'svelte';

	import { THREE } from 'aframe';

	let ready = false;
	let scale: [number, number, number] = [0.05, 0.025, 0.05];
	const zAxis = new THREE.Vector3(0, 0, 1);

	function degToRad(deg: number) {
		return (deg * Math.PI) / 180;
	}

	AFRAME.registerComponent('curve-point', {
		//dependencies: ['position'],

		schema: {},

		init: function () {
			this.el.addEventListener('componentchanged', this.changeHandler.bind(this));
			this.el.emit('curve-point-change');
		},

		changeHandler: function (event: Event) {
			if (event.detail.name == 'position') {
				this.el.emit('curve-point-change');
			}
		}
	});

	AFRAME.registerComponent('curve', {
		schema: {
			type: {
				type: 'string',
				default: 'CatmullRom',
				oneOf: ['CatmullRom', 'CubicBezier', 'QuadraticBezier', 'Line']
			},
			closed: {
				type: 'boolean',
				default: false
			}
		},

		init: function () {
			this.pathPoints = null;
			this.curve = null;

			this.el.addEventListener('curve-point-change', this.update.bind(this));
		},

		update: function (oldData) {
			this.points = Array.from(this.el.querySelectorAll('a-curve-point, [curve-point]'));

			if (this.points.length <= 1) {
				console.warn('At least 2 curve-points needed to draw a curve');
				this.curve = null;
			} else {
				// Get Array of Positions from Curve-Points
				let pointsArray = this.points.map(function (point) {
					if (point.x !== undefined && point.y !== undefined && point.z !== undefined) {
						return point;
					}

					return point.object3D.getWorldPosition(new THREE.Vector3());
				});

				// Update the Curve if either the Curve-Points or other Properties changed
				if (
					!AFRAME.utils.deepEqual(pointsArray, this.pathPoints) ||
					(oldData !== 'CustomEvent' && !AFRAME.utils.deepEqual(this.data, oldData))
				) {
					this.curve = null;

					this.pathPoints = pointsArray;

					// Create Curve
					switch (this.data.type) {
						case 'CubicBezier':
							if (this.pathPoints.length != 4) {
								throw new Error(
									'The Three constructor of type CubicBezierCurve3 requires 4 points'
								);
							}
							this.curve = new THREE.CubicBezierCurve3(
								this.pathPoints[0],
								this.pathPoints[1],
								this.pathPoints[2],
								this.pathPoints[3]
							);
							break;
						case 'QuadraticBezier':
							if (this.pathPoints.length != 3) {
								throw new Error(
									'The Three constructor of type QuadraticBezierCurve3 requires 3 points'
								);
							}
							this.curve = new THREE.QuadraticBezierCurve3(
								this.pathPoints[0],
								this.pathPoints[1],
								this.pathPoints[2]
							);
							break;
						case 'Line':
							if (this.pathPoints.length != 2) {
								throw new Error('The Three constructor of type LineCurve3 requires 2 points');
							}
							this.curve = new THREE.LineCurve3(this.pathPoints[0], this.pathPoints[1]);
							break;
						case 'CatmullRom':
							this.curve = new THREE.CatmullRomCurve3(this.pathPoints);
							break;
						case 'Spline':
							this.curve = new THREE.SplineCurve3(this.pathPoints);
							break;
						default:
							throw new Error(
								'No Three constructor of type (case sensitive): ' + this.data.type + 'Curve3'
							);
					}

					this.curve.closed = this.data.closed;

					this.el.emit('curve-updated');
				}
			}
		},

		remove: function () {
			this.el.removeEventListener('curve-point-change', this.update.bind(this));
		},

		closestPointInLocalSpace: function closestPoint(point, resolution, testPoint, currentRes) {
			if (!this.curve) throw Error('Curve not instantiated yet.');
			resolution = resolution || 0.1 / this.curve.getLength();
			currentRes = currentRes || 0.5;
			testPoint = testPoint || 0.5;
			currentRes /= 2;
			let aTest = testPoint + currentRes;
			let bTest = testPoint - currentRes;
			let a = this.curve.getPointAt(aTest);
			let b = this.curve.getPointAt(bTest);
			let aDistance = a.distanceTo(point);
			let bDistance = b.distanceTo(point);
			let aSmaller = aDistance < bDistance;
			if (currentRes < resolution) {
				let tangent = this.curve.getTangentAt(aSmaller ? aTest : bTest);
				if (currentRes < resolution)
					return {
						result: aSmaller ? aTest : bTest,
						location: aSmaller ? a : b,
						distance: aSmaller ? aDistance : bDistance,
						normal: normalFromTangent(tangent),
						tangent: tangent
					};
			}
			if (aDistance < bDistance) {
				return this.closestPointInLocalSpace(point, resolution, aTest, currentRes);
			} else {
				return this.closestPointInLocalSpace(point, resolution, bTest, currentRes);
			}
		}
	});

	const tempQuaternion = new THREE.Quaternion();

	function normalFromTangent(tangent: THREE.Vector3) {
		let lineEnd = new THREE.Vector3(0, 1, 0);
		tempQuaternion.setFromUnitVectors(zAxis, tangent);
		lineEnd.applyQuaternion(tempQuaternion);
		return lineEnd;
	}

	AFRAME.registerShader('line', {
		schema: {
			color: { default: '#ff0000' }
		},

		init: function (data) {
			this.material = new THREE.LineBasicMaterial(data);
		},

		update: function (data) {
			this.material = new THREE.LineBasicMaterial(data);
		}
	});

	AFRAME.registerComponent('draw-curve', {
		//dependencies: ['curve', 'material'],

		schema: {
			curve: { type: 'selector' }
		},

		init: function () {
			this.data.curve.addEventListener('curve-updated', this.update.bind(this));
		},

		update: function () {
			if (this.data.curve) {
				this.curve = this.data.curve.components.curve;
			}

			if (this.curve && this.curve.curve) {
				let lineGeometry = new THREE.BufferGeometry().setFromPoints(
					this.curve.curve.getPoints(this.curve.curve.getPoints().length * 10)
				);
				let mesh = this.el.getOrCreateObject3D('mesh', THREE.Line);
				lineMaterial = mesh.material
					? mesh.material
					: new THREE.LineBasicMaterial({
							color: '#ff0000'
					  });

				this.el.setObject3D('mesh', new THREE.Line(lineGeometry, lineMaterial));
			}
		},

		remove: function () {
			this.data.curve.removeEventListener('curve-updated', this.update.bind(this));
			this.el.getObject3D('mesh').geometry = new THREE.Geometry();
		}
	});

	AFRAME.registerComponent('clone-along-curve', {
		//dependencies: ['curve'],

		schema: {
			curve: { type: 'selector' },
			spacing: { default: 1 },
			rotation: {
				type: 'vec3',
				default: AFRAME.utils.coordinates.parse('0 0 0')
			},
			scale: {
				type: 'vec3',
				default: AFRAME.utils.coordinates.parse('1 1 1')
			}
		},

		init: function () {
			this.el.addEventListener('model-loaded', this.update.bind(this));
			this.data.curve.addEventListener('curve-updated', this.update.bind(this));
		},

		update: function () {
			this.remove();

			if (this.data.curve) {
				this.curve = this.data.curve.components.curve;
			}

			if (!this.el.getObject3D('clones') && this.curve && this.curve.curve) {
				let mesh = this.el.getObject3D('mesh');

				let length = this.curve.curve.getLength();
				let start = 0;
				let counter = start;

				let cloneMesh = this.el.getOrCreateObject3D('clones', THREE.Group);

				let parent = new THREE.Object3D();
				mesh.scale.set(this.data.scale.x, this.data.scale.y, this.data.scale.z);
				mesh.rotation.set(
					degToRad(this.data.rotation.x),
					degToRad(this.data.rotation.y),
					degToRad(this.data.rotation.z)
				);
				mesh.rotation.order = 'YXZ';

				parent.add(mesh);

				while (counter <= length) {
					let child = parent.clone(true);

					child.position.copy(this.curve.curve.getPointAt(counter / length));

					const tangent = this.curve.curve.getTangentAt(counter / length).normalize();

					child.quaternion.setFromUnitVectors(zAxis, tangent);

					cloneMesh.add(child);

					counter += this.data.spacing;
				}
			}
		},

		remove: function () {
			this.curve = null;
			if (this.el.getObject3D('clones')) {
				this.el.removeObject3D('clones');
			}
		}
	});

	if(!AFRAME.primitives.primitives['a-draw-curve']){
		AFRAME.registerPrimitive('a-draw-curve', {
		defaultComponents: {
			'draw-curve': {}
		},
		mappings: {
			curveref: 'draw-curve.curve'
		}
	});
  } 

  if(!AFRAME.primitives.primitives['a-curve-point']){
	AFRAME.registerPrimitive('a-curve-point', {
		defaultComponents: {
			'curve-point': {}
		},
		mappings: {}
	});
}

if(!AFRAME.primitives.primitives['a-curve']){
	AFRAME.registerPrimitive('a-curve', {
		defaultComponents: {
			curve: {}
		},

		mappings: {
			type: 'curve.type'
		}
	});
}

	AFRAME.registerComponent('lava-path', {
		init: function () {
			console.log('lava path:');
			console.log($gltfStore.lava_paths);

			if (!$gltfStore.lava_paths?.length) return;

			for (let j = 0; j < $gltfStore.lava_paths.length; j++) {
				const points = $gltfStore.lava_paths[j];

				//create curve element
				const curve = document.createElement('a-curve');
				curve.setAttribute('id', 'track' + j);

				//add points per curve
				for (let i = 0; i < points.length; i++) {
					//y and z swapped wrt given paths because Aframe uses different axes
					const v = points[i];
					const x = v[0] ;
					const y = v[2] ;
					const z = v[1] ;

					const p = document.createElement('a-curve-point');
					p.setAttribute('position', {
						 x: (x/scale[0])   ,
						 y: (y/scale[1]) ,
						 z: (z/scale[2]) ,
					});

					curve.appendChild(p);
				}

				//add curve element to scene
				this.el.appendChild(curve);

				//generate cylinders on curve add cylinder along track
				const track = document.createElement('a-entity');
				track.setAttribute(
					'clone-along-curve',
					'curve: #track' + j + '; spacing: 0.55; rotation: 90 0 0;'
				);
				track.setAttribute('geometry', 'primitive:cylinder; height:0.6; radius:0.4 ;');
				track.setAttribute('material', 'color: orangered; transparency: true; opacity: 0.001');
				//track.setAttribute('animation',"property: rotation; to: 0 360 0; loop: true; dur: 10000");
				const total_time = $gltfStore.lava_paths.length * 2000;
				track.setAttribute(
					'animation',
					'property: material.opacity; to: 1; dur: 2000; loop: false; delay: ' +
						(total_time - j * 2000) +
						';'
				);

				this.el.appendChild(track);
			}
		}
	});

	onDestroy(() => {
		delete AFRAME.components['curve-point'];
		delete AFRAME.components['curve'];
		delete AFRAME.components['draw-curve'];
		delete AFRAME.components['lava-path'];
		delete AFRAME.components['clone-along-curve'];
		delete AFRAME.shaders['line'];
	});
</script>


<SceneViewer />
