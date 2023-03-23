/**
 * Svelte stores, store reactive global data for more info:
 * https://svelte.dev/docs#run-time-svelte-store
 *
 * This stores size of the image, video, and canvas
 */

import { browser } from '$app/environment';
import { writable } from 'svelte/store';

const CACHE_KEY = 'size';
const DEFAULT_SIZE = '{}';

type Size = { width: number; height: number };

const stored = browser ? localStorage?.getItem(CACHE_KEY) || DEFAULT_SIZE : DEFAULT_SIZE;
const size = JSON.parse(stored) as Size;

const sizeStore = writable<Size>(size ?? { width: 800, height: 667 });

export default sizeStore;

// Set cache to local storage
sizeStore.subscribe((value) => {
	if (browser) localStorage?.setItem(CACHE_KEY, JSON.stringify(value));
});
