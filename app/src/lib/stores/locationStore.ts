/**
 * Svelte stores, store reactive global data for more info:
 * https://svelte.dev/docs#run-time-svelte-store
 *
 * This stores location data 😈
 */

import { writable } from 'svelte/store';
import Draggable from '$lib/data/draggable.ts';

type Vec2 = [number, number];

/**∏
 * Factory for creating a target of crater location store
 * @returns location store with method subscribe, add and remove
 */
// Get cache from local storage
function convertToTargets(cachedTargets: Draggable[]): Draggable[] {
	return cachedTargets.map((target) => {
		const dragItem = new Draggable(target.x, target.y, target.size, target.offsetX, target.offsetY);

		dragItem.enableSelection();
		dragItem.deselect();

		return dragItem;
	});
}

function createLocationStore<T>(storageIndex: string, convertToLocation: (locations: T[]) => T[]) {
	function getCache() {
		// Get initial locations from the local storage
		if (typeof window !== 'undefined') {
			const cachedLocations = localStorage?.getItem(storageIndex);

			try {
				if (cachedLocations) {
					const parsedLocations = JSON.parse(cachedLocations);
					return convertToLocation(parsedLocations);
				}
			} catch (_) {
				return [];
			}
		}
		return [];
	}

	// Set cache to local storage
	function setCache(locations: T[]) {
		if (typeof window !== 'undefined') {
			localStorage.setItem(storageIndex, JSON.stringify(locations));
		}
	}

	const { subscribe, update, set } = writable<T[]>(getCache());

	return {
		subscribe,
		clear: () => set([]),
		set: (locations: T[]) => {
			set(locations);
			setCache(locations);
		},
		add: (location: T) =>
			update((oldLocations) => {
				// append new location to the end of the array
				const newLocations = [...oldLocations, location];
				setCache(newLocations);
				return newLocations;
			}),
		remove: (index: number) =>
			update((oldLocations) => {
				// remove location at index
				const newLocations = oldLocations.filter((_, i) => i !== index);
				setCache(newLocations);
				return newLocations;
			})
	};
}

export const targetLocations = createLocationStore<Draggable>('targets', convertToTargets);
export const craterLocations = createLocationStore<Vec2>('craters', (ls) => ls);
