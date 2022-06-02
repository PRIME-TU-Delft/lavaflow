<script lang="ts">
	import NavigationButton from '$lib/components/NavigationButton.svelte';

	import { gltfUrl } from '$lib/stores/gltfStore';
	import { onMount } from 'svelte';

	let mounted: boolean;
	let aframe: boolean;
	let curveComponent: boolean
	let sceneEl: HTMLElement;
	$: ready = ((aframe && curveComponent) || window?.AFRAME) && mounted;

	function aframeLoaded() {
		aframe = true;

		console.log(window.AFRAME);
		// window.AFRAME.registerComponent('lava-paths', {
		// 	init: function () {
		// 		const lavaPaths: [number, number, number][][] = [
		// 			[
		// 				[3, 0, -5],
		// 				[-3, 2, 5],
		// 				[3, 0, -5]
		// 			],
		// 			[
		// 				[4, 4, 7],
		// 				[1, 1, 3],
		// 				[4, 2, 2],
		// 				[3, 3, -1]
		// 			],
		// 			[
		// 				[1, 1, 3],
		// 				[0, -2, -4],
		// 				[1, 1, -2],
		// 				[1, 2, -1],
		// 				[2, 2, 0]
		// 			],
		// 			[
		// 				[1, 1, -2],
		// 				[1, 4, -2],
		// 				[12, 1, -1]
		// 			]
		// 		];

		// 		for (let i = 0; i < lavaPaths.length; i++) {
		// 			const points = lavaPaths[i];

		// 			const curve = document.createElement('a-curve');
		// 			curve.setAttribute('id', 'track' + i);

		// 			points.forEach((point) => {
		// 				const [x, y, z] = point;

		// 				const aPoint = document.createElement('a-curve-point');
		// 				aPoint.setAttribute('position', x + ' ' + z + ' ' + y);
		// 				curve.appendChild(aPoint);
		// 			});

		// 			// Create track following each curve
		// 			const track = document.createElement('a-entity');
		// 			track.setAttribute('id', 'lava' + i);
		// 			track.setAttribute(
		// 				'clone-along-curve',
		// 				'curve: #track' + i + '; spacing: 0.2; rotation: 90 0 0;'
		// 			);
		// 			track.setAttribute('geometry', 'primitive:cylinder; height:0.33; radius:0.2; color:red');

		// 			this.el.appendChild(curve);
		// 			this.el.appendChild(track);
		// 		}
		// 	}
		// });
	}

	onMount(async () => {
		mounted = true;
	});
</script>

<svelte:head>
	{#if mounted && !window?.AFRAME}
		<script src="https://aframe.io/releases/1.0.0/aframe.min.js" on:load={aframeLoaded} ></script>
	{/if}

	<!-- {#if ready}
		<script src="https://rawgit.com/protyze/aframe-curve-component/master/dist/aframe-curve-component.min.js"></script>
	{/if} -->
</svelte:head>

{#if ready}
	<!-- MAKING COMPONENT FOR LAVA FLOW : (copied from the repo)-->>
<script>
	/**
	 * Curve component for A-Frame to deal with spline curves
	 */
	 var zAxis = new THREE.Vector3(0, 0, 1);
	var degToRad = THREE.Math.degToRad;

	AFRAME.registerComponent('curve-point', {

	    //dependencies: ['position'],

	    schema: {},

	    init: function () {
	        this.el.addEventListener("componentchanged", this.changeHandler.bind(this));
	        this.el.emit("curve-point-change");
	    },

	    changeHandler: function (event) {
	        if (event.detail.name == "position") {
	            this.el.emit("curve-point-change");
	        }
	    }

	});

	AFRAME.registerComponent('curve', {

	    //dependencies: ['curve-point'],

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

	        this.el.addEventListener("curve-point-change", this.update.bind(this));
	    },

	    update: function (oldData) {

	        this.points = Array.from(this.el.querySelectorAll("a-curve-point, [curve-point]"));

	        if (this.points.length <= 1) {
	            console.warn("At least 2 curve-points needed to draw a curve");
	            this.curve = null;
	        } else {
	            // Get Array of Positions from Curve-Points
	            var pointsArray = this.points.map(function (point) {

	                if (point.x !== undefined && point.y !== undefined && point.z !== undefined) {
	                    return point;
	                }

	                return point.object3D.getWorldPosition();
	            });

	            // Update the Curve if either the Curve-Points or other Properties changed
	            if (!AFRAME.utils.deepEqual(pointsArray, this.pathPoints) || (oldData !== 'CustomEvent' && !AFRAME.utils.deepEqual(this.data, oldData))) {
	                this.curve = null;

	                this.pathPoints = pointsArray;

	                // Create Curve
	                switch (this.data.type) {
	                    case 'CubicBezier':
	                        if (this.pathPoints.length != 4) {
	                            throw new Error('The Three constructor of type CubicBezierCurve3 requires 4 points');
	                        }
	                        this.curve = new THREE.CubicBezierCurve3(this.pathPoints[0], this.pathPoints[1], this.pathPoints[2], this.pathPoints[3]);
	                        break;
	                    case 'QuadraticBezier':
	                        if (this.pathPoints.length != 3) {
	                            throw new Error('The Three constructor of type QuadraticBezierCurve3 requires 3 points');
	                        }
	                        this.curve = new THREE.QuadraticBezierCurve3(this.pathPoints[0], this.pathPoints[1], this.pathPoints[2]);
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
	                        throw new Error('No Three constructor of type (case sensitive): ' + this.data.type + 'Curve3');
	                }

	                this.curve.closed = this.data.closed;

	                this.el.emit('curve-updated');
	            }
	        }

	    },

	    remove: function () {
	        this.el.removeEventListener("curve-point-change", this.update.bind(this));
	    },

	    closestPointInLocalSpace: function closestPoint(point, resolution, testPoint, currentRes) {
	        if (!this.curve) throw Error('Curve not instantiated yet.');
	        resolution = resolution || 0.1 / this.curve.getLength();
	        currentRes = currentRes || 0.5;
	        testPoint = testPoint || 0.5;
	        currentRes /= 2;
	        var aTest = testPoint + currentRes;
	        var bTest = testPoint - currentRes;
	        var a = this.curve.getPointAt(aTest);
	        var b = this.curve.getPointAt(bTest);
	        var aDistance = a.distanceTo(point);
	        var bDistance = b.distanceTo(point);
	        var aSmaller = aDistance < bDistance;
	        if (currentRes < resolution) {

	            var tangent = this.curve.getTangentAt(aSmaller ? aTest : bTest);
	            if (currentRes < resolution) return {
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


	var tempQuaternion = new THREE.Quaternion();

	function normalFromTangent(tangent) {
	    var lineEnd = new THREE.Vector3(0, 1, 0);
	    tempQuaternion.setFromUnitVectors(zAxis, tangent);
	    lineEnd.applyQuaternion(tempQuaternion);
	    return lineEnd;
	}

	AFRAME.registerShader('line', {
	    schema: {
	        color: {default: '#ff0000'},
	    },

	    init: function (data) {
	        this.material = new THREE.LineBasicMaterial(data);
	    },

	    update: function (data) {
	        this.material = new THREE.LineBasicMaterial(data);
	    },
	});

	AFRAME.registerComponent('draw-curve', {

	    //dependencies: ['curve', 'material'],

	    schema: {
	        curve: {type: 'selector'}
	    },

	    init: function () {
	        this.data.curve.addEventListener('curve-updated', this.update.bind(this));
	    },

	    update: function () {
	        if (this.data.curve) {
	            this.curve = this.data.curve.components.curve;
	        }

	        if (this.curve && this.curve.curve) {
	            var lineGeometry = new THREE.BufferGeometry().setFromPoints(this.curve.curve.getPoints(this.curve.curve.getPoints().length * 10));
	            var mesh = this.el.getOrCreateObject3D('mesh', THREE.Line);
	            lineMaterial = mesh.material ? mesh.material : new THREE.LineBasicMaterial({
	                color: "#ff0000"
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
	        curve: {type: 'selector'},
	        spacing: {default: 1},
	        rotation: {
	            type: 'vec3',
	            default: '0 0 0'
	        },
	        scale: {
	            type: 'vec3',
	            default: '1 1 1'
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
	            var mesh = this.el.getObject3D('mesh');

	            var length = this.curve.curve.getLength();
	            var start = 0;
	            var counter = start;

	            var cloneMesh = this.el.getOrCreateObject3D('clones', THREE.Group);

	            var parent = new THREE.Object3D();
	            mesh.scale.set(this.data.scale.x, this.data.scale.y, this.data.scale.z);
	            mesh.rotation.set(degToRad(this.data.rotation.x), degToRad(this.data.rotation.y), degToRad(this.data.rotation.z));
	            mesh.rotation.order = 'YXZ';

	            parent.add(mesh);

	            while (counter <= length) {
	                var child = parent.clone(true);

	                child.position.copy(this.curve.curve.getPointAt(counter / length));

	                tangent = this.curve.curve.getTangentAt(counter / length).normalize();

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

	AFRAME.registerPrimitive('a-draw-curve', {
	    defaultComponents: {
	        'draw-curve': {},
	    },
	    mappings: {
	        curveref: 'draw-curve.curve',
	    }
	});

	AFRAME.registerPrimitive('a-curve-point', {
	    defaultComponents: {
	        'curve-point': {},
	    },
	    mappings: {}
	});

	AFRAME.registerPrimitive('a-curve', {
	    defaultComponents: {
	        'curve': {}
	    },

	    mappings: {
	        type: 'curve.type',
	    }
	});


  </script>


	<a-scene embedded>
		<div class="backButton">
			<NavigationButton back to="/scan/mapscanning">Rescan image</NavigationButton>
		</div>

		<a-box position="0 1 0" material="opacity: 0.5;" color="red" />

		<a-entity
			gltf-model="url({$gltfUrl})"
			position="3 0 -5"
			scale="0.00038 0.1 0.00038"
			rotation="0 -90 0"
			material="opacity: 0.5;"
			id="model"
		/>

	<!-- <a-curve id="track1">
      <a-curve-point position="-1 1 -3"></a-curve-point>
      <a-curve-point position="0 2 -3"></a-curve-point>
      <a-curve-point position="1 1 -3"></a-curve-point>
    </a-curve> -->

    <!-- <a-curve id="track2">
      <a-curve-point position="-3 2 -3"></a-curve-point>
      <a-curve-point position="0 5 -3"></a-curve-point>
      <a-curve-point position="1 1 -3"></a-curve-point>
    </a-curve> -->

	

    <!-- Clone a Box along the Curve -->
	<!-- <a-draw-curve curveref="#track1" material="shader: line; color: red;"></a-draw-curve>
	<a-draw-curve curveref="#track2" material="shader: line; color: red;"></a-draw-curve> -->


    <!-- <a-entity
      clone-along-curve="curve: #track1; spacing: 0.2; rotation: 90 0 0;"
      geometry="primitive:cylinder; height:0.33; radius:0.2; color:red"
    ></a-entity>
    <a-entity
      clone-along-curve="curve: #track2; spacing: 0.2; rotation: 90 0 0;"
      geometry="primitive:cylinder; height:0.33; radius:0.2; color:red"
    ></a-entity> -->

		<a-camera look-controls />
	</a-scene>
	<!-- var sceneEl = document.querySelector('a-scene'); -->
	<script>
		       
    
   
				var sceneEl = document.querySelector('a-scene');
	
				var curves = [  [ [4, 4, 7], [1, 1, 3], [4, 2, 2], [3, 3, -1] ],
								[ [1, 1, 3], [0,-2, -4], [1, 1, -2], [1, 2, -1] , [2, 2, 0] , ],           
								[ [1, 1, -2], [1, 4, -2], [12, 1, -1] ]
							];

				//mult curve, mult points 
						//add curves with point

				for(let j = 0; j < curves.length; j++){
				    //get points on curve
					var points = curves[j];
					
				    //create curve element
					var curve = document.createElement('a-curve');
					curve.setAttribute('id' , "track" + j);
					

					//add points per curve
					for (let i = 0; i < points.length; i++) {
						
						var v = points[i];
						var x = v[0];
						var y = v[1];
						var z = v[2];
						
						var p = document.createElement('a-curve-point');

							p.setAttribute('position', { x: x , y: z, z: y });
							
							curve.appendChild(p);
						
					}

					//add curve element to scene
					sceneEl.appendChild(curve);
					
					//generate cylinders on curve add cylinder along track
					var track = document.createElement('a-entity');
					track.setAttribute('clone-along-curve',"curve: #track" + j + "; spacing: 0.2; rotation: 90 0 0;" );
					track.setAttribute('geometry',"primitive:cylinder; height:0.33; radius:0.2" );
					track.setAttribute('material', 'color: crimson; transparency: true; opacity: 0.01');
					//track.setAttribute('animation',"property: rotation; to: 0 360 0; loop: true; dur: 10000");
					track.setAttribute('animation', "property: material.opacity; to: 1; dur: 10000; loop: false; delay: " + j * 5000 + ";"  ) ;

					sceneEl.appendChild(track);

				}

		</script>
{/if}

<style>
	.backButton {
		position: absolute;
		top: 1rem;
		left: 1rem;
		z-index: 1;
		width: 15rem;
		max-width: calc(100vw - 2rem);
	}
</style>
