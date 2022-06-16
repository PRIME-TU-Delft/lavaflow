<script lang="ts">
	import Button from '$lib/components/Button.svelte';
	import Icon from '$lib/components/Icon.svelte';
	import SceneViewer from '$lib/components/aframe/SceneViewer.svelte';

	import { mdiChevronLeft, mdiChevronRight } from '@mdi/js';

	import '@ar-js-org/ar.js';
	import { onDestroy } from 'svelte';
	import type { Component, Entity, ObjectMap, System } from 'aframe';
	import type { Vector3 } from 'three';

	interface GestureHandlerType extends Component<any, System<any>> {
		isVisible: boolean;
		initialScale: Vector3;
		scaleFactor: number;
		el: Entity<ObjectMap<Component<any, System<any>>>> & {
			sceneEl: any;
		};
		handleRotation: (event: CustomEvent<{ positionChange: { x: number; y: number } }>) => any;
	}

	AFRAME.registerComponent<GestureHandlerType>('gesture-handler', {
		schema: {
			enabled: { default: true },
			rotationFactor: { default: 5 },
			minScale: { default: 0.3 },
			maxScale: { default: 8 }
		},

		init: function () {
			this.handleScale = this.handleScale.bind(this);
			this.handleRotation = this.handleRotation.bind(this);

			this.isVisible = false;
			this.initialScale = this.el.object3D.scale.clone();
			this.scaleFactor = 1;

			this.el.sceneEl.addEventListener('markerFound', () => {
				this.isVisible = true;
			});

			this.el.sceneEl.addEventListener('markerLost', () => {
				this.isVisible = false;
			});
		},

		update: function () {
			if (this.data.enabled) {
				this.el.sceneEl.addEventListener('onefingermove', this.handleRotation);
				this.el.sceneEl.addEventListener('twofingermove', this.handleScale);
			} else {
				this.el.sceneEl.removeEventListener('onefingermove', this.handleRotation);
				this.el.sceneEl.removeEventListener('twofingermove', this.handleScale);
			}
		},

		remove: function () {
			this.el.sceneEl.removeEventListener('onefingermove', this.handleRotation);
			this.el.sceneEl.removeEventListener('twofingermove', this.handleScale);
		},

		handleRotation: function (event) {
			if (this.isVisible) {
				this.el.object3D.rotation.y += event.detail.positionChange.x * this.data.rotationFactor;
				this.el.object3D.rotation.x += event.detail.positionChange.y * this.data.rotationFactor;
			}
		},

		handleScale: function (event) {
			if (this.isVisible) {
				this.scaleFactor *= 1 + event.detail.spreadChange / event.detail.startSpread;

				this.scaleFactor = Math.min(
					Math.max(this.scaleFactor, this.data.minScale),
					this.data.maxScale
				);

				this.el.object3D.scale.x = this.scaleFactor * this.initialScale.x;
				this.el.object3D.scale.y = this.scaleFactor * this.initialScale.y;
				this.el.object3D.scale.z = this.scaleFactor * this.initialScale.z;
			}
		}
	});

	// Component that detects and emits events for touch gestures

	interface GestureDetectorType {
		targetElement: Entity<ObjectMap<Component<any, System<any>>>>;
		internalState: {
			previousState?: { position: { x: number; y: number; z: number }; spread: number };
		};
		emitGestureEvent: {
			bind: (t: GestureDetectorType & Component<any, System<any>>) => () => any;
		};
	}

	AFRAME.registerComponent<GestureDetectorType>('gesture-detector', {
		schema: {
			element: { default: '' }
		},

		init: function () {
			this.targetElement = this.data.element && document.querySelector(this.data.element);

			if (!this.targetElement) {
				this.targetElement = this.el;
			}

			this.internalState = {
				previousState: undefined
			};

			this.emitGestureEvent = this.emitGestureEvent.bind(this);

			this.targetElement.addEventListener('touchstart', this.emitGestureEvent);

			this.targetElement.addEventListener('touchend', this.emitGestureEvent);

			this.targetElement.addEventListener('touchmove', this.emitGestureEvent);
		},

		remove: function () {
			this.targetElement.removeEventListener('touchstart', this.emitGestureEvent);

			this.targetElement.removeEventListener('touchend', this.emitGestureEvent);

			this.targetElement.removeEventListener('touchmove', this.emitGestureEvent);
		},

		emitGestureEvent(event: Event) {
			const currentState = this.getTouchState(event);

			const previousState = this.internalState.previousState;

			const gestureContinues =
				previousState && currentState && currentState.touchCount == previousState.touchCount;

			const gestureEnded = previousState && !gestureContinues;

			const gestureStarted = currentState && !gestureContinues;

			if (gestureEnded) {
				const eventName = this.getEventPrefix(previousState.touchCount) + 'fingerend';

				this.el.emit(eventName, previousState);

				this.internalState.previousState = undefined;
			}

			if (gestureStarted) {
				currentState.startTime = performance.now();

				currentState.startPosition = currentState.position;

				currentState.startSpread = currentState.spread;

				const eventName = this.getEventPrefix(currentState.touchCount) + 'fingerstart';

				this.el.emit(eventName, currentState);

				this.internalState.previousState = currentState;
			}

			if (gestureContinues) {
				const eventDetail: Partial<{
					positionChange: { x: number; y: number };
					spreadChange: number;
				}> = {
					positionChange: {
						x: currentState.position.x - previousState.position.x,

						y: currentState.position.y - previousState.position.y
					}
				};

				if (currentState.spread) {
					eventDetail.spreadChange = currentState.spread - previousState.spread;
				}

				// Update state with new data

				Object.assign(previousState, currentState);

				// Add state data to event detail

				Object.assign(eventDetail, previousState);

				const eventName = this.getEventPrefix(currentState.touchCount) + 'fingermove';

				this.el.emit(eventName, eventDetail);
			}
		},

		getTouchState: function (event) {
			if (event.touches.length === 0) {
				return null;
			}

			// Convert event.touches to an array so we can use reduce

			const touchList = [];

			for (let i = 0; i < event.touches.length; i++) {
				touchList.push(event.touches[i]);
			}

			const touchState = {
				touchCount: touchList.length
			};

			// Calculate center of all current touches

			const centerPositionRawX =
				touchList.reduce((sum, touch) => sum + touch.clientX, 0) / touchList.length;

			const centerPositionRawY =
				touchList.reduce((sum, touch) => sum + touch.clientY, 0) / touchList.length;

			touchState.positionRaw = { x: centerPositionRawX, y: centerPositionRawY };

			// Scale touch position and spread by average of window dimensions

			const screenScale = 2 / (window.innerWidth + window.innerHeight);

			touchState.position = {
				x: centerPositionRawX * screenScale,
				y: centerPositionRawY * screenScale
			};

			// Calculate average spread of touches from the center point

			if (touchList.length >= 2) {
				const spread =
					touchList.reduce((sum, touch) => {
						return (
							sum +
							Math.sqrt(
								Math.pow(centerPositionRawX - touch.clientX, 2) +
									Math.pow(centerPositionRawY - touch.clientY, 2)
							)
						);
					}, 0) / touchList.length;

				touchState.spread = spread * screenScale;
			}

			return touchState;
		},

		getEventPrefix(touchCount: number) {
			const numberNames = ['one', 'two', 'three', 'many'];

			return numberNames[Math.min(touchCount, 4) - 1];
		}
	});

	onDestroy(() => {
		delete AFRAME.components['gesture-handler'];
		delete AFRAME.components['gesture-detector'];
	});
</script>

<!-- Load scene in arMode -->
<SceneViewer arMode>
	<!-- Custom back button that will reload the page-->
	<a slot="backButton" sveltekit:reload href="/scan/preview">
		<Button icon={mdiChevronLeft}>Back to preview</Button>
	</a>

	<!-- Custom target button -->
	<a slot="targetButton" sveltekit:reload href="/targetplacement">
		<Button>
			<span>Place targets</span>

			<Icon path={mdiChevronRight} />
		</Button>
	</a>
</SceneViewer>
