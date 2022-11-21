<script lang="ts">
	import P5 from './P5.svelte'
	import type p5 from 'p5';

	import { craterLocations, targetLocations } from '$lib/stores/locationStore';
	import { difficultyStore } from '$lib/stores/difficultyStore';

	export let foregroundWidth: number; // Width of the foreground canvas
	export let foregroundHeight: number; // Height of the foreground canvas
	export let targetSelected: number;
	export let curves: [number, number][][];

	function sketch(p5: p5) {
		p5.setup = () => {
			p5.createCanvas(foregroundWidth, foregroundHeight);
		};

		p5.draw = () => {
			p5.background(230);
			p5.stroke(30);

			curves.forEach((layer) => {
				// Display the level-curves to the user
				p5.strokeWeight(4);
				p5.noFill();
				p5.beginShape();

				// Add points to the shape
				for (let [x, y] of layer) p5.vertex(x, y);

				// Close the shape
				p5.endShape(p5.CLOSE);
			});

			// Draw the craters | if the craters are stored in the gltf store -> pick those craters
			//                  | else -> pick the (cached) craters from the localstorage
			const craters = $craterLocations ?? [];
			// console.log({ gltf: $gltfStore?.craters, $craterLocations });
			for (let crater of $craterLocations) {
				p5.strokeWeight(1);
				p5.stroke('#e15f55f0');

				const [posX, posY] = crater;

				// Inner ring
				p5.fill('#e15f55f0');
				p5.ellipse(posX, posY, 50, 50);
				p5.noFill();

				// Outer ring - for showing users the minumum distance from the crater
				p5.strokeWeight(0.5);
				p5.ellipse(
					posX,
					posY,
					$difficultyStore.min_crater_distance * 2,
					$difficultyStore.min_crater_distance * 2
				);
			}

			// Draw the turbine-markers
			let markerSize = 20;

			for (let i = 0; i < $targetLocations.length; i++) {
				let target = $targetLocations[i];
				target.update(
					p5,
					craters,
					$difficultyStore.min_crater_distance,
					$targetLocations,
					$difficultyStore.min_steam_turbine_separation,
					i
				);
				target.drawCircle(p5, markerSize, i + 1);
			}

			// The user will have to insert at least a certain amount of steam-turbines
			// this amount is defined in the difficultyStore
			let msg = '';
			if (
				$targetLocations.length != $difficultyStore.min_steam_turbines &&
				$difficultyStore.min_steam_turbines == $difficultyStore.max_steam_turbines
			) {
				// The player hasn't yet placed enough turbines
				msg = 'You must place exactly ' + $difficultyStore.min_steam_turbines + ' steam turbines';
			} else if ($targetLocations.length < $difficultyStore.min_steam_turbines) {
				// The player hasn't yet placed enough turbines
				msg = 'You must place at least ' + $difficultyStore.min_steam_turbines + ' steam turbines';
			} else if ($targetLocations.length == $difficultyStore.max_steam_turbines) {
				// The player placed too many turbines
				msg =
					'You have reached the maximal amount of ' +
					$difficultyStore.max_steam_turbines +
					' steam turbines';
			}

			if (
				$targetLocations.length < $difficultyStore.min_steam_turbines ||
				$targetLocations.length >= $difficultyStore.max_steam_turbines
			) {
				p5.noStroke();
				p5.fill(51);
				p5.textSize(15);
				p5.textAlign(p5.CENTER);

				let text_width = p5.textWidth(msg);

				p5.rectMode(p5.CENTER);
				p5.rect(p5.width / 2, 60, text_width + 10, 30);

				p5.strokeWeight(1);
				p5.fill(255);

				p5.text(msg, p5.width / 2, 65);
			}
		};

		// If the user presses/releases their mouse, signal this to all Draggable points
		p5.mousePressed = () => {
			if (!$targetLocations?.length) return;

			let foundFirstMarker = false;

			for (let i = 0; i < $targetLocations.length; i++) {
				let isPressed = false;
				$targetLocations[i].deselect();

				if (!foundFirstMarker) {
					isPressed = $targetLocations[i].pressed(p5, true);
					foundFirstMarker = isPressed;
				}

				if (isPressed) {
					targetSelected = i;
				}
			}
		};

		p5.mouseReleased = () => {
			if (!$targetLocations?.length) return;

			for (let target of $targetLocations) target.released();

			// When mouse is released -> reset the cache
			targetLocations.set($targetLocations);
		};
	}
</script>

<P5 {sketch} />
